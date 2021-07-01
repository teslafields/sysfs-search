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

struct USBDevs { path: &'static Path, filter: Vec<&'static str> }

trait SysfsSearch {
    // Static method signature; `Self` refers to the implementor type.
    fn new(path: &'static Path, filter: Vec<&'static str>) -> Self;
    fn path(&self) -> &'static Path;
    fn search(&self);
    fn output(&self) {
    }
}

impl SysfsSearch for USBDevs {
    // `Self` is the implementor type: `Sheep`.
    fn new(path: &'static Path, filter: Vec<&'static str>) -> USBDevs {
        USBDevs { path: path, filter: filter }
    }

    fn path(&self) -> &'static Path {
        self.path
    }

    fn search(&self) {
    }
}

fn visit_dirs<'a>(dir: &Path, filter: &str, limit: u64, result: &'a mut Vec<String>) -> Result<(), io::Error> {
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
                visit_dirs(&path.as_path(), &filter, limit-1, result)?;
            } else if name == String::from(filter) {
                // println!("{:?}, {:?}", path, path.parent());
                let str_path = path.into_os_string();
                result.push(String::from(str_path.to_str().unwrap()));
            }
        }
    }
    Ok(())
}

fn search(dir: &Path, filter: &Vec<&str>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    if filter.len() == 0 {
        return result;
    }
    for i in 0..filter.len() {
        if i == 0 {
            let _ = visit_dirs(dir, filter[i], 7, &mut result);
        } else {
            result.retain(|x| x.contains(filter[i]));
        }
    }
    result
}

fn main() {
    let path = Path::new("/sys/bus/usb/devices/usb1");
    let filter = vec!["dev", "ttyUSB"];
    let usbs: USBDevs = USBDevs::new(path, filter);
    usbs.output();
    let result = search(usbs.path(), &usbs.filter);
    println!("\n--------------------------------------------------\n");
    for item in result {
        println!("{:?}", item);
    }
}

