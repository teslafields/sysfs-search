mod utils;
mod sysfssearch;

use std::io;
use std::env;
use std::fs::{self};
use std::path::Path;

const USB_VENDOR_ID: (&str, usize) = ("ID_VENDOR_ID", 4);
const USB_MODEL_ID: (&str, usize) = ("ID_MODEL_ID", 4);
const USB_TTY_NUM: (&str, usize) = ("ID_USB_INTERFACE_NUM", 2);

const SIMCOM: u32 = 7694; // 0x1e0e
const SIM7600: u32 = 36865; // 0x9001

fn main() {
    let argv: Vec<String> = env::args().collect();
    let mut filter: Option<Vec<&str>> = None;
    if argv.len() < 2 {
        println!("missing path argument");
        return;
    } else if argv.len() > 2 {
        let mut filter_vec = Vec::<&str>::new();
        for i in 2..argv.len() {
            println!("{} {}", i, argv[i]);
            filter_vec.push(argv[i].as_str());
        }
        filter = Some(filter_vec);
    }
    let path = &argv[1];
    let mut search = sysfssearch::SysfsSearch::new(path.as_str(), filter, Some(7));
    search.search();
}

