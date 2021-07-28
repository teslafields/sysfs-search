mod utils;
mod sysfssearch;
mod simcom;
mod usbdev;

use std::env;

const ARGC: usize = 3;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let mut filter: Option<Vec<&str>> = None;
    if argv.len() < ARGC {
        panic!("Missing path argument!");
    } else if argv.len() > ARGC {
        let mut filter_vec = Vec::<&str>::new();
        for i in ARGC..argv.len() {
            filter_vec.push(argv[i].as_str());
        }
        filter = Some(filter_vec);
    }
    let path = &argv[2];
    if argv[1] == simcom::SimCom::SIM7600 {
        let mut simdev = simcom::SimCom::new(path.as_str(), filter, Some(7));
        simdev.find_simcom_dev();
    }
}

