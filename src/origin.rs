extern crate fs_extra;

use crate::object::Object;
use fs_extra::dir::{DirEntryValue, get_size, ls, get_dir_content, DirEntryAttr, LsResult, DirEntryValue::U64};
use std::collections::HashSet;
use std::path::Path;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum OriginType {
    Source,
    Destination,
}

pub struct Pool {
    path: String,
    origin_type: OriginType,
    top_level_objects: Vec<Object>,
    total_size: u32,
}

impl Pool {
    pub fn new(t: &OriginType, path: &String) -> Pool {
        if !Path::new(&path).exists() {
            panic!("No directory: {}", &path);
        }

        let mut config = HashSet::new();
        config.insert(DirEntryAttr::Name);
        config.insert(DirEntryAttr::Size);
        config.insert(DirEntryAttr::BaseInfo);
        let ls_res = ls(&path, &config).unwrap();

        let mut objects = Vec::<Object>::new();
        for map in ls_res.items {
            let size_value = map.get(&DirEntryAttr::Size).unwrap();
            let name = {
                match map.get(&DirEntryAttr::Name).unwrap() {
                    DirEntryValue::String(s) => s.as_str(),
                    _ => panic!(),
                }
            };
            let size = {
                match map.get(&DirEntryAttr::Size).unwrap() {
                    DirEntryValue::U64(v) => v,
                    _ => panic!(),
                }
            };

            objects.push(Object::new(&name.to_string(), *size as u32));
        }

        let pool = Pool {
            path: path.to_string(),
            origin_type: *t,
            top_level_objects: objects,
            total_size: 1,};

        for tlo in &pool.top_level_objects {
            println!("There is an object {}", tlo.path);
        }
        pool
    }
}
