import { writeFileSync } from "node:fs";

const s1buf = Memory.alloc(25080);
const s2buf = Memory.alloc(561352);
let offset = 0;

function main() {
    Interceptor.attach(Module.getGlobalExportByName("WriteFile"), {
        onEnter(args) {
            const n = args[2].toUInt32();
            const data = args[1].readByteArray(n)!;
            console.log(`${n} bytes =>`);

            if (n <= 512) {
                console.log(hexdump(data));
            }

            if (n == 8192 || n == 504) {
                Memory.copy(s1buf.add(offset), args[1], n);
                offset += n;

                if (offset == 25080) {
                    try {
                        console.log("write");
                        writeFileSync("stage1.bin", Buffer.from(s1buf.readByteArray(offset)!));
                        console.log("dumped stage 1", offset);
                    } catch (e) {
                        console.log(`error on writing image: ${e}`)
                    } finally {
                        offset = 0;
                    }
                }
            } else if (n == 131072 || n == 37064) {
                Memory.copy(s2buf.add(offset), args[1], n);
                offset += n;

                if (offset == 561352) {
                    try {
                        console.log("write");
                        writeFileSync("stage2.bin", Buffer.from(s2buf.readByteArray(offset)!));
                        console.log("dumped stage 2", offset);
                    } catch (e) {
                        console.log(`error on writing image: ${e}`)
                    } finally {
                        new NativeFunction(Module.getGlobalExportByName("exit"), "void", ["int"])(0x42);
                    }
                }
            }
        }
    });

    Interceptor.attach(Module.getGlobalExportByName("ReadFile"), {
        onEnter(args) {
            this.buf = args[1];
            this.n_ptr = args[3];
        },
        onLeave(ret) {
            const n = this.n_ptr.readU32();
            if (n == 512) // oh noes
                return;

            if (n <= 512) {
                console.log(hexdump(this.buf.readByteArray(n)));
            }

            console.log(`${n} bytes <=`);
        }
    });
}

(globalThis as any).main = main;

