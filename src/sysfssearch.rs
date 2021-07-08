// trait SysfsSearch {
//     fn new(path: &'static Path, filter: Vec<&'static str>) -> Self;
//     fn path(&self) -> &'static Path;
//     fn search(&self);
//     fn output(&self) {
//     }
// }
use crate::utils;
use std::io;
use std::fs::{self};
use std::path::Path;
use std::default::Default;

#[derive(Default)]
pub struct SysfsSearch<'a> {
    path: &'a str,
    filter: Option<Vec<&'a str>>,
    search_limit: Option<u32>,
    buffer: Vec<String>,
}

impl SysfsSearch<'_> {
    pub fn new<'a>(
            path: &'a str,
            filter: Option<Vec<&'a str>>,
            search_limit: Option<u32>,
            ) -> SysfsSearch<'a> {
        SysfsSearch {
            path: path,
            filter: filter,
            search_limit: search_limit,
            ..Default::default()
        }
    }

    fn visit_dirs(dir: &Path, filter: &str, limit: u32, result: &mut Vec<String>) -> Result<(), io::Error> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                // Here we need to consume the filename in order to skip dirs
                let fname = entry.file_name();
                let str_fname = fname.to_str().unwrap();
                let condition = !str_fname.contains("subsystem") &&
                    !str_fname.contains("driver") &&
                    !str_fname.contains("firmware_node") &&
                    !str_fname.contains("port") && limit > 0;
                if path.is_dir() && condition {
                    Self::visit_dirs(&path.as_path(), filter, limit-1, result)?;
                } else if str_fname == filter {
                    let parent = path.parent().unwrap();
                    let str_parent = parent.to_str().unwrap();
                    result.push(String::from(str_parent)); // save the parent
                }
            }
        }
        Ok(())
    }

    fn get_property(base_str: &str, key: &str) -> Option<String> {
        let start_idx = base_str.find(key);
        if start_idx.is_none() {
            return None;
        }
        let start_idx = start_idx.unwrap();
        let offset: usize = key.len() + start_idx + 1;
        let end_idx = &base_str[offset..].find('\n');
        if end_idx.is_none() {
            return None;
        }
        let end_idx = end_idx.unwrap();
        let value_sliced: &str = &base_str[offset..offset + end_idx];
        // let value: u32 = u32::from_str_radix(value_sliced, 16).unwrap();
        println!("str[{}..{}] = {}", offset, offset + end_idx, value_sliced);
        Some(String::from(value_sliced))
    }

    fn run_udevadm(&self, path_vec: &Vec<String>) -> Vec<utils::CommandResult> {
        let mut stdout_vec = Vec::new();
        for path in path_vec {
            let opts = ["info", "-q", "property", "-p", path.as_str()];
            stdout_vec.push(utils::execute_command("udevadm", Some(&opts)));
        }
        stdout_vec
    }

    pub fn search(&mut self) -> Option<&Vec<String>> {
        let mut path_list: Vec<String> = Vec::new();
        println!("{:?} {:?}", self.path, self.filter);
        if self.filter.is_none() {
            path_list.push(String::from(self.path));
        } else {
            let filter = self.filter.as_deref();
            let filter = filter.unwrap();
            let limit: u32;
            if self.search_limit.is_none() {
                limit = u32::MAX;
            } else {
                limit = self.search_limit.unwrap();
            }
            if filter.len() == 0 {
                return None;
            }
            for i in 0..filter.len() {
                if i == 0 {
                    let _ = Self::visit_dirs(Path::new(self.path), filter[i], limit, &mut path_list);
                } else {
                    path_list.retain(|x| x.contains(filter[i]));
                }
            }
        }
        let udev_result = self.run_udevadm(&path_list);
        for item in udev_result {
            if item.status != Some(0) {
                continue;
            }
            if item.stdout.is_none() {
                continue;
            }
            let out = item.stdout.unwrap();
            self.buffer.push(out);
        }
        for i in 0..self.buffer.len() {
            println!("{}", self.buffer[i]);
        }
        Some(&self.buffer)
    }
}

