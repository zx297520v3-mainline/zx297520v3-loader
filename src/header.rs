use std::fmt::Display;

use crate::err::{Error, Result};
use serde::{Deserialize, Serialize};
use zerocopy::{Immutable, IntoBytes, KnownLayout, TryFromBytes};

const MAGIC: [u8; 8] = [b'Z', b'X', b'7', b'5', b'2', b'1', b'V', b'1'];

#[derive(Debug, Immutable, KnownLayout, TryFromBytes, IntoBytes, PartialEq, Serialize, Deserialize)]
#[repr(C)]
pub struct Header {
    unk: u32,
    magic: [u8; 8],
    pub data_size: u32,
    #[serde(with = "serde_byte_array")]
    rsa_pub_e: [u8; 128],
    #[serde(with = "serde_byte_array")]
    rsa_pub_n: [u8; 128],
    #[serde(with = "serde_byte_array")]
    hash_y: [u8; 128],
    unk1: u32,
    stack: u32,
    pub entry: u32,
    panic: u32,
    panic1: u32,
    bss_start: u32,
    nop: [u16; 2],
    bss_end: u32,
    para_start: u32,
    para_end: u32,
}

impl Header {
    pub fn try_read(src: &[u8]) -> Result<&Self> {
        // sorry.
        let header = Self::try_ref_from_bytes(src).map_err(|_| Error::Zerocopy)?;

        if header.magic.as_ref() == MAGIC {
            Ok(header)
        } else {
            Err(Error::InvalidHeaderMagic(header.magic))
        }
    }
}

impl Default for Header {
    fn default() -> Self {
        Self {
            unk: 0x08,
            magic: MAGIC,
            data_size: 0,
            rsa_pub_e: [0; 128],
            rsa_pub_n: [0; 128],
            hash_y: [0; 128],
            unk1: 0,
            stack: 0x8a000,
            entry: 0,
            panic: 0,
            panic1: 0,
            bss_start: 0,
            nop: [0xc046; 2],
            bss_end: 0,
            para_start: 0x82000000,
            para_end: 0x82000050,
        }
    }
}

impl Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Data size: {:#x}", self.data_size)?;
        writeln!(f, "RSA pubkey exponent: {:02x?}", self.rsa_pub_e)?;
        writeln!(f, "RSA pubkey modulus: {:02x?}", self.rsa_pub_n)?;
        writeln!(f, "Hash: {:02x?}", self.hash_y)?;
        writeln!(f, "Stack ptr: {:#x}", self.stack)?;
        writeln!(f, "Entrypoint: {:#x}", self.entry & !1)?;
        writeln!(f, "BSS start: {:#x}", self.bss_start)?;
        write!(f, "BSS end: {:#x}", self.bss_end)
    }
}
