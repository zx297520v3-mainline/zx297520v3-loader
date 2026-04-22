use clap::Parser;
use nusb::io::{EndpointRead, EndpointWrite};
use nusb::transfer::{Bulk, In, Out};
use nusb::{DeviceInfo, MaybeFuture, list_devices};
use simpleport::{SimpleRead, SimpleWrite};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;
use zerocopy::IntoBytes;
use zx297520v3_loader::STAGE1_BASE;
use zx297520v3_loader::err::{Error, Result};
use zx297520v3_loader::header::Header;

type EpIn = EndpointRead<Bulk>;
type EpOut = EndpointWrite<Bulk>;

const STAGE2_BASE: u32 = 0x27ef0000;

const ACK: u8 = 0x5a;
const DEVICE_ACK: u8 = 0xa5;

const SEND_IMAGE: u8 = 0x7a;
const IMAGE_SETUP_ACCEPTED: u8 = 0xa1;
const IMAGE_ACCEPTED: u8 = 0xa7;

const JUMP: u8 = 0x8a;
const JUMP_ACCEPTED: u8 = 0xa8;

const STAGE2_ACK: u8 = 0x5a;
const STAGE2_DEVICE_ACK: u8 = 0xa7;

#[derive(Parser)]
struct Cli {
    /// Stage 1 (tloader or openloader) file
    #[arg(short = '1', long)]
    stage1: PathBuf,

    /// Stage 2 (tboot or upstream U-Boot) file
    #[arg(short = '2', long)]
    stage2: PathBuf,

    /// Sync with stock tboot
    #[arg(short, long)]
    stock: bool,
}

fn get_dev() -> Result<Option<DeviceInfo>> {
    Ok(list_devices().wait()?.find(|dev| dev.vendor_id() == 0x19d2 && dev.product_id() == 0x0256))
}

fn ack(r: &mut EpIn, w: &mut EpOut) -> Result<()> {
    w.write_u8(ACK)?;
    w.flush()?;
    if r.read_u8()? == DEVICE_ACK { Ok(()) } else { Err(Error::InvalidAck) }
}

fn stage2_ack(r: &mut EpIn, w: &mut EpOut) -> Result<()> {
    w.write_u8(STAGE2_ACK)?;
    w.flush()?;
    if r.read_u8()? == STAGE2_DEVICE_ACK { Ok(()) } else { Err(Error::InvalidAck) }
}

fn send_image(r: &mut EpIn, w: &mut EpOut, base: u32, data: &[u8], chunk_size: usize) -> Result<()> {
    w.write_u8(SEND_IMAGE)?;
    w.write_u32_be(base)?;
    w.write_u32_be(data.len() as u32)?;
    w.flush()?;

    if r.read_u8()? != IMAGE_SETUP_ACCEPTED {
        return Err(Error::StageSetupNotAccepted);
    }

    for i in data.chunks(chunk_size) {
        w.write_all(i)?;
        w.flush()?;
    }

    if r.read_u8()? == IMAGE_ACCEPTED { Ok(()) } else { Err(Error::StageNotAccepted) }
}

fn jumpout(r: &mut EpIn, w: &mut EpOut, addr: u32) -> Result<()> {
    w.write_u8(JUMP)?;
    w.write_u32_be(addr)?;
    w.flush()?;
    if r.read_u8()? == JUMP_ACCEPTED { Ok(()) } else { Err(Error::JumpNotAccepted) }
}

fn entry() -> Result<()> {
    let cli = Cli::parse();

    println!("Waiting for the device...");
    let dev = loop {
        match get_dev()? {
            Some(dev) => break dev,
            None => sleep(Duration::from_millis(500)),
        }
    };

    let dev = dev.open().wait()?;
    let interface = dev.claim_interface(0).wait()?;
    let mut reader = interface.endpoint::<Bulk, In>(0x81)?.reader(512);
    let mut writer = interface.endpoint::<Bulk, Out>(0x01)?.writer(512);

    println!("Device connected");
    ack(&mut reader, &mut writer)?;

    let mut payload = fs::read(cli.stage1)?;

    if let Err(_) = Header::try_read(&payload) {
        let mut header = Header::default();
        let size = size_of::<Header>();
        header.entry = STAGE1_BASE + size as u32 | 1;
        header.data_size = payload.len() as u32;

        payload.reserve(size as usize);

        if let Err(e) = header.write_to(&mut payload[0..size as usize]).map_err(|_| Error::Zerocopy) {
            eprintln!("Error on appending header: {e}");
        } else {
            println!("Appended header to the image, size: {:#x}", size);
        }
    }

    println!("Uploading stage 1");
    send_image(&mut reader, &mut writer, STAGE1_BASE, &payload, 0x2000)?;

    println!("Jumping to stage 1");
    jumpout(&mut reader, &mut writer, STAGE1_BASE)?;

    ack(&mut reader, &mut writer)?;
    println!("Uploading stage 2");
    send_image(&mut reader, &mut writer, STAGE2_BASE, &fs::read(cli.stage2)?, 0x20000)?;
    println!("Jumping to stage 2");
    jumpout(&mut reader, &mut writer, STAGE2_BASE)?;

    if cli.stock {
        stage2_ack(&mut reader, &mut writer)?;
        println!("Got loader sync");
    };

    Ok(())
}

fn main() -> core::result::Result<(), String> {
    if let Err(e) = entry() { Err(e.to_string()) } else { Ok(()) }
}
