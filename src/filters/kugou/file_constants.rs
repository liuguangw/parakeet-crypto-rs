pub const KGM_HEADER: [u8; 16] = [
    0x7C, 0xD5, 0x32, 0xEB, 0x86, 0x02, 0x7F, 0x4B, 0xA8, 0xAF, 0xA6, 0x8E, 0x0F, 0xFF, 0x99, 0x14,
];

pub const KGM_EXPECTED_DECRYPTION_RESULT: [u8; 16] = [
    0x38, 0x85, 0xED, 0x92, 0x79, 0x5F, 0xF8, 0x4C, 0xB3, 0x03, 0x61, 0x41, 0x16, 0xA0, 0x1D, 0x47,
];

pub const VPR_HEADER: [u8; 16] = [
    0x05, 0x28, 0xBC, 0x96, 0xE9, 0xE4, 0x5A, 0x43, 0x91, 0xAA, 0xBD, 0xD0, 0x7A, 0xF5, 0x36, 0x31,
];

pub const VPR_EXPECTED_DECRYPTION_RESULT: [u8; 16] = [
    0x1D, 0x5A, 0x05, 0x34, 0x0C, 0x41, 0x8D, 0x42, 0x9C, 0x83, 0x92, 0x6C, 0xAE, 0x16, 0xFE, 0x56,
];
