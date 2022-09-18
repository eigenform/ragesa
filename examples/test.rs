

use std::env;
use std::fs;

use ragesa::*;
use pretty_hex::*;

fn main() {
    if env::args().len() == 1 {
        return;
    }

    let args: Vec<String> = env::args().collect();
    
    let paths = fs::read_dir(&args[1]).unwrap();
    for path in paths {
        let s = path.unwrap().path()
            .into_os_string().into_string()
            .unwrap();
        ROMFile::parse(&s);
        println!();
    }



}
