//! AMI Aptio capsule format

pub const APTIO_MAGIC: [u8; 16] = [
    0x8b, 0xa6, 0x3c, 0x4a, 0x23, 0x77, 0xfb, 0x48,
    0x80, 0x3d, 0x57, 0x8c, 0xc1, 0xfe, 0xc4, 0x4d
];

pub struct AptioTableEntry {
    pub off: u32,
    pub flg0: u32,
    pub off2: u32,
    pub size: u32,
    pub flg1: u32,
    pub flg2: u32,
}

#[derive(Debug)]
pub struct AptioHeader {
    pub magic: [u8; 0x10],
    pub hdr_len: u32,
    pub unk_14:  u32,
    pub cap_len: u32,
    pub cap_off:  u16,
    pub tbl_off:  u16,
}
impl AptioHeader {
    pub fn volume_offset(&self) -> usize { self.cap_off as usize }
    pub fn table_offset(&self) -> usize { self.tbl_off as usize }
    pub fn header_len(&self) -> usize { self.hdr_len as usize }
    pub fn capsule_len(&self) -> usize { self.cap_len as usize }

    pub fn new(data: &[u8]) -> Self {
        assert!(data.len() == 0x20);
        let mut magic = [0u8; 0x10];
        magic.copy_from_slice(&data[0..0x10]);

        let hdr_len = u32::from_le_bytes(
            data[0x10..0x14].try_into().unwrap()
        );
        let unk_14 = u32::from_le_bytes(
            data[0x14..0x18].try_into().unwrap()
        );
        let cap_len = u32::from_le_bytes(
            data[0x18..0x1c].try_into().unwrap()
        );
        let cap_off = u16::from_le_bytes(
            data[0x1c..0x1e].try_into().unwrap()
        );
        let tbl_off = u16::from_le_bytes(
            data[0x1e..0x20].try_into().unwrap()
        );

        Self { magic, hdr_len, unk_14, cap_len, cap_off, tbl_off }
    }
}


