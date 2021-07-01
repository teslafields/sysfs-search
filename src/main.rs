/// #!/bin/bash
/// 
/// num_of_ifaces=0
/// start_iface=100
/// for devpath in $(find /sys/bus/usb/devices/usb*/ -name dev); do
///     syspath="${devpath%/dev}"
///     devname="$(udevadm info -q name -p $syspath)"
///     [[ "$devname" == "bus/"* ]] && continue
///     eval "$(udevadm info -q property --export -p $syspath)"
///     [[ -z "$ID_SERIAL" || "$ID_VENDOR" != "SimTech"* ]] && continue
///     # echo "/dev/$devname $ID_VENDOR"
///     devnum[${#devnum[*]}]=$ID_USB_INTERFACE_NUM
///     num_of_ifaces=$((num_of_ifaces + 1))
///     if [ $ID_USB_INTERFACE_NUM -lt $start_iface ]; then
///         start_iface=$ID_USB_INTERFACE_NUM
///     fi
/// done
/// #echo ${devnum[@]}
/// echo $start_iface $num_of_ifaces
/// if [ $num_of_ifaces -eq 0 ]; then
///    exit 1
/// fi
use std::io;
use std::fs::{self};
use std::path::Path;

struct USBDevs { path: &'static Path, filter: Vec<&'static str>, rec_limit: u32 }

trait SysfsSearch {
    // Static method signature; `Self` refers to the implementor type.
    fn new(path: &'static Path, filter: Vec<&'static str>) -> Self;
    fn path(&self) -> &'static Path;
    fn search(&self);
    fn output(&self) {
    }
}

impl USBDevs {
    fn new(path: &'static Path, filter: Vec<&'static str>, rec_limit: u32) -> USBDevs {
        USBDevs { path: path, filter: filter, rec_limit: rec_limit }
    }

    fn visit_dirs(&mut self, dir: &Path, filter: &str, limit: u32, result: &mut Vec<String>) -> Result<(), io::Error> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                let fname = entry.file_name();
                let name = String::from(fname.to_str().unwrap());
                if path.is_dir() &&
                        !name.contains("subsystem") &&
                        !name.contains("driver") &&
                        !name.contains("firmware_node") &&
                        !name.contains("port") &&
                        limit > 0 {
                    self.visit_dirs(&path.as_path(), filter, limit-1, result)?;
                } else if name == String::from(filter) {
                    let str_path = path.into_os_string();
                    result.push(String::from(str_path.to_str().unwrap()));
                }
            }
        }
        Ok(())
    }

    fn search(&mut self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        if self.filter.len() == 0 {
            return result;
        }
        println!("{:?} {:?}", self.path, self.filter);
        for i in 0..self.filter.len() {
            if i == 0 {
                let _ = self.visit_dirs(self.path, self.filter[i], self.rec_limit, &mut result);
            } else {
                result.retain(|x| x.contains(self.filter[i]));
            }
        }
        result
    }
}

fn main() {
    let path = Path::new("/sys/bus/usb/devices/usb1");
    let filter = vec!["dev", "ttyUSB"];
    let mut usbs: USBDevs = USBDevs::new(path, filter, 7);
    let result = usbs.search();
    println!("\n--------------------------------------------------\n");
    for item in result {
        println!("{:?}", item);
    }
}

