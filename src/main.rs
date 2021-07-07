use std::io;
use std::env;
use std::fs::{self};
use std::path::Path;
use std::process::Command;
use std::str::from_utf8;

const USB_VENDOR_ID: (&str, usize) = ("ID_VENDOR_ID", 4);
const USB_MODEL_ID: (&str, usize) = ("ID_MODEL_ID", 4);
const USB_TTY_NUM: (&str, usize) = ("ID_USB_INTERFACE_NUM", 2);

const SIMCOM: u32 = 7694; // 0x1e0e
const SIM7600: u32 = 36865; // 0x9001


struct USBDevs<'a> {
    path: &'a Path,
    filter: Vec<&'static str>,
    rec_limit: u32
}

struct CommandResult {
    status: Option<i32>,
    stdout: Option<String>,
    stderr: Option<String>
}

trait SysfsSearch {
    // Static method signature; `Self` refers to the implementor type.
    fn new(path: &'static Path, filter: Vec<&'static str>) -> Self;
    fn path(&self) -> &'static Path;
    fn search(&self);
    fn output(&self) {
    }
}

impl USBDevs<'_> {
    fn new<'a>(path: &'a Path, filter: Vec<&'static str>, rec_limit: u32) -> USBDevs<'a> {
        USBDevs { path: path, filter: filter, rec_limit: rec_limit }
    }

    fn visit_dirs(dir: &Path, filter: &str, limit: u32, result: &mut Vec<String>) -> Result<(), io::Error> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                // Here we need to consume the filename in order to skip dirs
                let fname = entry.file_name();
                let str_fname = String::from(fname.to_str().unwrap());
                let condition = !str_fname.contains("subsystem") &&
                    !str_fname.contains("driver") &&
                    !str_fname.contains("firmware_node") &&
                    !str_fname.contains("port") && limit > 0;
                if path.is_dir() && condition {
                    Self::visit_dirs(&path.as_path(), filter, limit-1, result)?;
                } else if str_fname.as_str() == filter {
                    let parent = path.parent().unwrap();
                    let str_parent = parent.to_str().unwrap();
                    result.push(String::from(str_parent)); // save the parent
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
                let _ = Self::visit_dirs(self.path, self.filter[i], self.rec_limit, &mut result);
            } else {
                result.retain(|x| x.contains(self.filter[i]));
            }
        }
        result
    }

    fn get_property(base_str: &str, key: (&str, usize)) -> Option<u32> {
        let base_idx = base_str.find(key.0);
        if base_idx.is_none() {
            return None;
        }
        let base_idx = base_idx.unwrap();
        let offset = key.0.len() + base_idx + 1;
        if base_str.len() < offset + key.1 {
            return None;
        }
        let value_sliced: &str = &base_str[offset..offset+key.1];
        let value: u32 = u32::from_str_radix(value_sliced, 16).unwrap();
        println!("{}, {}, {}", base_idx, value_sliced, value);
        Some(value)
    }

    fn run_udevadm(&self, path_vec: &Vec<String>) {
        let mut stdout_vec: Vec<CommandResult> = Vec::new();
        for path in path_vec {
            let opts = ["info", "-q", "property", "-p", path.as_str()];
            stdout_vec.push(execute_command("udevadm", Some(&opts)));
        }
        let mut tty_vec: Vec<u32> = Vec::new();
        for item in stdout_vec {
            let stdout_str = item.stdout.unwrap();
            if Self::get_property(&stdout_str, USB_VENDOR_ID) != Some(SIMCOM) {
                continue;
            }
            if Self::get_property(&stdout_str, USB_MODEL_ID) != Some(SIM7600) {
                continue;
            }
            let value = Self::get_property(&stdout_str, USB_TTY_NUM);
            if value.is_none() {
                continue;
            };
            tty_vec.push(value.unwrap());
        }
        let mut cloned_vec = tty_vec.clone();
        let tty_vec_sorted = cloned_vec.as_mut_slice();
        println!("TTY: {:?} {:?}", tty_vec, tty_vec_sorted);
    }
}

fn execute_command(cmd: &str, args: Option<&[&str]>) -> CommandResult {
    println!("DEB: {} {:?}", cmd, args);
    let mut command = Command::new(cmd);
    if args.is_some() {
        command.args(args.unwrap());
    }
    let mut result = CommandResult{status: None, stdout: None, stderr: None};
    let output = command.output().expect("ERR: Command failed!");
    result.status = output.status.code();
    if output.stderr.len() > 0 {
        result.stderr = match from_utf8(output.stdout.as_slice()) {
            Ok(v) => Some(v.to_string()),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
    }
    if output.stdout.len() > 0 {
        result.stdout = match from_utf8(output.stdout.as_slice()) {
            Ok(v) => Some(v.to_string()),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
    }
    result
}

fn main() {
    let argv: Vec<String> = env::args().collect();
    if argv.len() < 2 {
        println!("missing path argument");
        return;
    }
    let path = Path::new(argv[1].as_str());
    let filter = vec!["dev", "ttyUSB"];
    let mut usbs: USBDevs = USBDevs::new(path, filter, 7);
    let result = usbs.search();
    usbs.run_udevadm(&result);
}

