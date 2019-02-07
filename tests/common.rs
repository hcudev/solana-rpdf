// Converted from the tests for uBPF <https://github.com/iovisor/ubpf>
// Copyright 2015 Big Switch Networks, Inc
// Copyright 2016 6WIND S.A. <quentin.monnet@6wind.com>
//
// Licensed under the Apache License, Version 2.0 <http://www.apache.org/licenses/LICENSE-2.0> or
// the MIT license <http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![allow(clippy::deprecated_cfg_attr)]
#![cfg_attr(rustfmt, rustfmt_skip)]

// Assembly code and data for tcp_sack testcases.

#[allow(dead_code)]
pub const TCP_SACK_ASM: &'static str = "
    ldxb r2, [r1+12]
    ldxb r3, [r1+13]
    lsh r3, 0x8
    or r3, r2
    mov r0, 0x0
    jne r3, 0x8, +37
    ldxb r2, [r1+23]
    jne r2, 0x6, +35
    ldxb r2, [r1+14]
    add r1, 0xe
    and r2, 0xf
    lsh r2, 0x2
    add r1, r2
    mov r0, 0x0
    ldxh r4, [r1+12]
    add r1, 0x14
    rsh r4, 0x2
    and r4, 0x3c
    mov r2, r4
    add r2, -20
    mov r5, 0x15
    mov r3, 0x0
    jgt r5, r4, +20
    mov r5, r3
    lsh r5, 0x20
    arsh r5, 0x20
    mov r4, r1
    add r4, r5
    ldxb r5, [r4]
    jeq r5, 0x1, +4
    jeq r5, 0x0, +12
    mov r6, r3
    jeq r5, 0x5, +9
    ja +2
    add r3, 0x1
    mov r6, r3
    ldxb r3, [r4+1]
    add r3, r6
    lsh r3, 0x20
    arsh r3, 0x20
    jsgt r2, r3, -18
    ja +1
    mov r0, 0x1
    exit";

#[allow(dead_code)]
pub const TCP_SACK_BIN: [u8;352] = [
    0x71, 0x12, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x71, 0x13, 0x0d, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x67, 0x03, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00,
    0x4f, 0x23, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xb7, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x55, 0x03, 0x25, 0x00, 0x08, 0x00, 0x00, 0x00,
    0x71, 0x12, 0x17, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x55, 0x02, 0x23, 0x00, 0x06, 0x00, 0x00, 0x00,
    0x71, 0x12, 0x0e, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x07, 0x01, 0x00, 0x00, 0x0e, 0x00, 0x00, 0x00,
    0x57, 0x02, 0x00, 0x00, 0x0f, 0x00, 0x00, 0x00,
    0x67, 0x02, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
    0x0f, 0x21, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xb7, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x69, 0x14, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x07, 0x01, 0x00, 0x00, 0x14, 0x00, 0x00, 0x00,
    0x77, 0x04, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
    0x57, 0x04, 0x00, 0x00, 0x3c, 0x00, 0x00, 0x00,
    0xbf, 0x42, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x07, 0x02, 0x00, 0x00, 0xec, 0xff, 0xff, 0xff,
    0xb7, 0x05, 0x00, 0x00, 0x15, 0x00, 0x00, 0x00,
    0xb7, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x2d, 0x45, 0x14, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xbf, 0x35, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x67, 0x05, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00,
    0xc7, 0x05, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00,
    0xbf, 0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x0f, 0x54, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x71, 0x45, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x15, 0x05, 0x04, 0x00, 0x01, 0x00, 0x00, 0x00,
    0x15, 0x05, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xbf, 0x36, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x15, 0x05, 0x09, 0x00, 0x05, 0x00, 0x00, 0x00,
    0x05, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x07, 0x03, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
    0xbf, 0x36, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x71, 0x43, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x0f, 0x63, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x67, 0x03, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00,
    0xc7, 0x03, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00,
    0x6d, 0x32, 0xee, 0xff, 0x00, 0x00, 0x00, 0x00,
    0x05, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xb7, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
    0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
];

#[allow(dead_code)]
pub const TCP_SACK_MATCH: [u8;78] = [
    0x00, 0x26, 0x62, 0x2f, 0x47, 0x87, 0x00, 0x1d,
    0x60, 0xb3, 0x01, 0x84, 0x08, 0x00, 0x45, 0x00,
    0x00, 0x40, 0xa8, 0xde, 0x40, 0x00, 0x40, 0x06,
    0x9d, 0x58, 0xc0, 0xa8, 0x01, 0x03, 0x3f, 0x74,
    0xf3, 0x61, 0xe5, 0xc0, 0x00, 0x50, 0xe5, 0x94,
    0x3f, 0x77, 0xa3, 0xc4, 0xc4, 0x80, 0xb0, 0x10,
    0x01, 0x3e, 0x34, 0xb6, 0x00, 0x00, 0x01, 0x01,
    0x08, 0x0a, 0x00, 0x17, 0x95, 0x6f, 0x8d, 0x9d,
    0x9e, 0x27, 0x01, 0x01, 0x05, 0x0a, 0xa3, 0xc4,
    0xca, 0x28, 0xa3, 0xc4, 0xcf, 0xd0
];

#[allow(dead_code)]
pub const TCP_SACK_NOMATCH: [u8;66] = [
    0x00, 0x26, 0x62, 0x2f, 0x47, 0x87, 0x00, 0x1d,
    0x60, 0xb3, 0x01, 0x84, 0x08, 0x00, 0x45, 0x00,
    0x00, 0x40, 0xa8, 0xde, 0x40, 0x00, 0x40, 0x06,
    0x9d, 0x58, 0xc0, 0xa8, 0x01, 0x03, 0x3f, 0x74,
    0xf3, 0x61, 0xe5, 0xc0, 0x00, 0x50, 0xe5, 0x94,
    0x3f, 0x77, 0xa3, 0xc4, 0xc4, 0x80, 0x80, 0x10,
    0x01, 0x3e, 0x34, 0xb6, 0x00, 0x00, 0x01, 0x01,
    0x08, 0x0a, 0x00, 0x17, 0x95, 0x6f, 0x8d, 0x9d,
    0x9e, 0x27
];
