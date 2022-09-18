//! EFS (Embedded File System) format

pub const EFS_MAGIC: [u8; 4] = [ 0xaa, 0x55, 0xaa, 0x55 ];
pub const EFS_MAGIC_REGEX: &'static str = r"(?-u)\xaa\x55\xaa\x55";

#[derive(Debug)]
pub struct EfsHeader {
    pub data: [u32; 16],
    pub imc_off: u32,
    pub gec_off: u32,
    pub xhci_off: u32,
    pub psp_off: u32,
    pub psp_combo_off: u32,
    pub bios0_off: u32,
    pub bios1_off: u32,
    pub bios2_off: u32,
    pub flags: u32,
    pub bios3_off: u32,
    pub bios4_off: u32,
    pub prom_off: u32,
    pub prom_lp_off: u32,
    pub unk_38: u32,
    pub unk_3c: u32,
}
impl EfsHeader {
    pub fn new(data: &[u8]) -> Self {
        assert!(data.len() == 0x40);
        let mut res = [0u32; 16];
        unsafe {
            res.copy_from_slice(
                std::slice::from_raw_parts(data.as_ptr()as *const u32, 16)
            )
        }
        Self { 
            data: res,
            imc_off: res[1],
            gec_off: res[2],
            xhci_off: res[3],
            psp_off: res[4],
            psp_combo_off: res[5],
            bios0_off: res[6],
            bios1_off: res[7],
            bios2_off: res[8],
            flags: res[9],
            bios3_off: res[10],
            bios4_off: res[11],
            prom_off: res[12],
            prom_lp_off: res[13],
            unk_38: res[14],
            unk_3c: res[15],
        }
    }
}


