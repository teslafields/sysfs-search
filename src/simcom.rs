use crate::sysfssearch::SysfsSearch;
use crate::usbdev;

pub struct SimCom<'a> {
    sysfsobj: SysfsSearch<'a>,
}

impl SimCom<'_> {
    const SIMCOM_VID: &'static str = "1e0e"; // 7694
    const SIM7600_MID: &'static str = "9001"; // 36865
    pub const SIM7600: &'static str = "SIM7600";

    pub fn new<'a>(
            path: &'a str,
            filter: Option<Vec<&'a str>>,
            search_limit: Option<u32>,
            ) -> SimCom<'a> {
        SimCom {
            sysfsobj: SysfsSearch::new(path, filter, search_limit),
        }
    }

    pub fn get_property(&mut self, key: &str, criteria: Option<&Vec<(&str, &str)>>)
            -> Option<Vec<String>> {
        self.sysfsobj.get_property(key, criteria)
    }

    pub fn find_simcom_dev(&mut self) {
        self.sysfsobj.search();
        let criteria = vec![(usbdev::VENDOR_ID, Self::SIMCOM_VID), (usbdev::MODEL_ID, Self::SIM7600_MID)];
        let result = self.get_property(usbdev::TTY_NUM, Some(&criteria));
        if result.is_some() {
            let result = result.unwrap();
            let tty_num: u32 = result.iter().map(|x| x.parse::<u32>().unwrap()).min().unwrap();
            println!("TTY_START={}\nTTY_TOTAL={}", tty_num, result.len());
        }

    }
}

