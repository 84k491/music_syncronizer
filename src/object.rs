use fs_extra::dir::{DirEntryValue, get_size, ls, get_dir_content, DirEntryAttr, LsResult, DirEntryValue::U64};
use std::collections::HashMap;

pub struct Object {
    pub path: String,
    pub size: u64,
}
impl Object {
    pub fn new(path: &String, size: u64) -> Object {
        Object { path: path.to_string(), size: size }
    }
    pub fn from_ls_result(ls_res_map: &HashMap<DirEntryAttr, DirEntryValue>) -> Object {
        let name = {
            match ls_res_map.get(&DirEntryAttr::Name).unwrap() {
                DirEntryValue::String(s) => s.as_str(),
                _ => panic!(),
            }
        };
        let size = {
            match ls_res_map.get(&DirEntryAttr::Size).unwrap() {
                DirEntryValue::U64(v) => v,
                _ => panic!(),
            }
        };
        Object::new(&name.to_string(), *size as u64)
    }
}
