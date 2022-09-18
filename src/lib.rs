
pub mod aptio;
pub mod efs;

use std::fs::File;
use std::io::Read;
use pretty_hex::*;
use regex::bytes::Regex;

use aptio::*;
use efs::*;

#[derive(Debug)]
pub enum ImageKind {
    Bare,
    Aptio(AptioHeader),
}

/// Container for a firmware image.
pub struct ROMFile;
impl ROMFile {

    // FIXME: Need a way to validate that matches are well-formed ...
    pub fn parse_efs(data: &[u8], data_len: usize, off: usize) {
        let hdr = EfsHeader::new(&data[off..off+0x40]);
        println!("{:#08x?}", hdr);
    }
    
    pub fn parse_bare(data: &[u8], data_len: usize) {
        println!("{:08x} {:08x}", data.len(), data_len);
        assert!(data.len() == data_len);
        let re = Regex::new(EFS_MAGIC_REGEX).unwrap();
        let matches: Vec<usize> = re.find_iter(&data)
            .map(|m| m.start())
            .filter(|off| off & 0x7ff == 0)
            .collect();

        if matches.len() == 0 {
            panic!("No EFS entries?");
        }

        println!("EFS Magic: {:08x?}", matches);
        for off in matches.iter() {
            Self::parse_efs(data, data_len, *off);
        }

    }

    pub fn parse(filename: &str) {

        let mut f = File::open(filename).unwrap();
        let file_len = f.metadata().unwrap().len() as usize;
        let mut buf = vec![0u8; file_len];
        f.read(&mut buf).unwrap();

        println!("Parsing len={:08x} '{}' ", file_len, filename);

        let mut cursor = 0;
        while cursor < file_len {
            let mut data   = &buf[cursor..];
            println!("cursor={:08x} rem={:08x}", cursor, data.len());

            let mut container = ImageKind::Bare;
            if data[0..0x10] == APTIO_MAGIC {
                println!("Found Aptio image");
                let aptio_hdr = AptioHeader::new(&data[0..0x20]);
                let tbl_off = aptio_hdr.table_offset();
                container = ImageKind::Aptio(aptio_hdr);
            }

            match container {
                ImageKind::Bare => {
                    println!("Parsing bare image ...");
                    Self::parse_bare(data, data.len());
                    cursor += data.len();
                },
                ImageKind::Aptio(hdr) => {
                    println!("Parsing Aptio capsule ...");
                    println!("{:#08x?}", hdr);
                    let start = 0;
                    let end   = hdr.capsule_len();
                    Self::parse_bare(&data[0..hdr.capsule_len()], 
                                     hdr.capsule_len());
                    cursor += hdr.capsule_len();
                }
                _ => unimplemented!("{:?}", container),
            }
        }

    }
}

