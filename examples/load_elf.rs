// Copyright 2016 6WIND S.A. <quentin.monnet@6wind.com>
//
// Licensed under the Apache License, Version 2.0 <http://www.apache.org/licenses/LICENSE-2.0> or
// the MIT license <http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![cfg_attr(feature = "cargo-clippy", allow(unreadable_literal))]

extern crate elf;
use std::path::PathBuf;

extern crate rbpf;
use rbpf::helpers;

// The following example uses an ELF file that has been compiled from the C program available in
// `load_elf__block_a_port.c` in the same directory.
//
// It was compiled with the following command:
//
// ```bash
// clang -O2 -emit-llvm -c load_elf__block_a_port.c -o - | \
//     llc -march=bpf -filetype=obj -o load_elf__block_a_port.o
// ```
//
// Once compiled, this program can be injected into Linux kernel, with tc for instance. Sadly, we
// need to bring some modifications to the generated bytecode in order to run it: the three
// instructions with opcode 0x61 load data from a packet area as 4-byte words, where we need to
// load it as 8-bytes double words (0x79). The kernel does the same kind of translation before
// running the program, but rbpf does not implement this.
//
// In addition, the offset at which the pointer to the packet data is stored must be changed: since
// we use 8 bytes instead of 4 for the start and end addresses of the data packet, we cannot use
// the offsets produced by clang (0x4c and 0x50), the addresses would overlap. Instead we can use,
// for example, 0x40 and 0x50.
//
// These change were applied with the following script:
//
// ```bash
// xxd load_elf__block_a_port.o | sed '
//     s/6112 5000 0000 0000/7912 5000 0000 0000/ ;
//     s/6111 4c00 0000 0000/7911 4000 0000 0000/ ;
//     s/6111 2200 0000 0000/7911 2200 0000 0000/' | xxd -r > load_elf__block_a_port.tmp

// mv load_elf__block_a_port.tmp load_elf__block_a_port.o
// ```
//
// The eBPF program was placed into the `.classifier` ELF section (see C code above), which means
// that you can retrieve the raw bytecode with `readelf -x .classifier load_elf__block_a_port.o` or
// with `objdump -s -j .classifier load_elf__block_a_port.o`.
//
// Once the bytecode has been edited, we can load the bytecode directly from the ELF object file.

fn main() {

    let filename = "examples/load_elf__block_a_port.o";

    let path = PathBuf::from(filename);
    let file = match elf::File::open_path(&path) {
        Ok(f) => f,
        Err(e) => panic!("Error: {:?}", e),
    };

    let text_scn = match file.get_section(".classifier") {
        Some(s) => s,
        None => panic!("Failed to look up .classifier section"),
    };

    let prog = &text_scn.data;

    let packet1 = &mut [
        0x01, 0x23, 0x45, 0x67, 0x89, 0xab,
        0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54,
        0x08, 0x00, // ethertype
        0x45, 0x00, 0x00, 0x3b, // start ip_hdr
        0xa6, 0xab, 0x40, 0x00,
        0x40, 0x06, 0x96, 0x0f,
        0x7f, 0x00, 0x00, 0x01,
        0x7f, 0x00, 0x00, 0x01,
        // Program matches the next two bytes: 0x9999 returns 0xffffffff, else return 0.
        0x99, 0x99, 0xc6, 0xcc, // start tcp_hdr
        0xd1, 0xe5, 0xc4, 0x9d,
        0xd4, 0x30, 0xb5, 0xd2,
        0x80, 0x18, 0x01, 0x56,
        0xfe, 0x2f, 0x00, 0x00,
        0x01, 0x01, 0x08, 0x0a, // start data
        0x00, 0x23, 0x75, 0x89,
        0x00, 0x23, 0x63, 0x2d,
        0x71, 0x64, 0x66, 0x73,
        0x64, 0x66, 0x0au8
    ];

    let packet2 = &mut [
        0x01, 0x23, 0x45, 0x67, 0x89, 0xab,
        0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54,
        0x08, 0x00, // ethertype
        0x45, 0x00, 0x00, 0x3b, // start ip_hdr
        0xa6, 0xab, 0x40, 0x00,
        0x40, 0x06, 0x96, 0x0f,
        0x7f, 0x00, 0x00, 0x01,
        0x7f, 0x00, 0x00, 0x01,
        // Program matches the next two bytes: 0x9999 returns 0xffffffff, else return 0.
        0x98, 0x76, 0xc6, 0xcc, // start tcp_hdr
        0xd1, 0xe5, 0xc4, 0x9d,
        0xd4, 0x30, 0xb5, 0xd2,
        0x80, 0x18, 0x01, 0x56,
        0xfe, 0x2f, 0x00, 0x00,
        0x01, 0x01, 0x08, 0x0a, // start data
        0x00, 0x23, 0x75, 0x89,
        0x00, 0x23, 0x63, 0x2d,
        0x71, 0x64, 0x66, 0x73,
        0x64, 0x66, 0x0au8
    ];

    let mut vm = rbpf::EbpfVmFixedMbuff::new(Some(prog), 0x40, 0x50).unwrap();
    vm.register_helper(helpers::BPF_TRACE_PRINTK_IDX, helpers::bpf_trace_printf);

    let res = vm.prog_exec(packet1);
    println!("Packet #1, program returned: {:?} ({:#x})", res, res);
    assert_eq!(res, 0xffffffff);

    #[cfg(not(windows))]
    {
        vm.jit_compile();

        let res = unsafe { vm.prog_exec_jit(packet2) };
        println!("Packet #2, program returned: {:?} ({:#x})", res, res);
        assert_eq!(res, 0);
    }

    #[cfg(windows)]
    {
        let res = vm.prog_exec(packet2);
        println!("Packet #2, program returned: {:?} ({:#x})", res, res);
        assert_eq!(res, 0);
    }
}
