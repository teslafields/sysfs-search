mod utils;
mod sysfssearch;
mod simcom;
mod usbdev;

use std::env;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let mut filter: Option<Vec<&str>> = None;
    if argv.len() < 2 {
        panic!("Missing path argument!");
    } else if argv.len() > 2 {
        let mut filter_vec = Vec::<&str>::new();
        for i in 2..argv.len() {
            filter_vec.push(argv[i].as_str());
        }
        filter = Some(filter_vec);
    }
    let path = &argv[1];
    let mut simdev = simcom::SimCom::new(path.as_str(), filter, Some(7));
    simdev.find_simcom_dev();
}

