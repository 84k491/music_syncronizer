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
    total_size: u64,
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
        let mut total_size: u64 = 0;
        for res_map in &ls_res.items {
            let obj = Object::from_ls_result(&res_map);
            println!("Entry: {}; Size: {}; total: {}", obj.path, obj.size, total_size);
            total_size += obj.size;
            objects.push(obj);
        }

        let pool = Pool {
            path: path.to_string(),
            origin_type: *t,
            top_level_objects: objects,
            total_size: total_size,};

        for tlo in &pool.top_level_objects {
            println!("There is an object {}", tlo.path);
        }
        pool
    }

    pub fn contains(self, obj: &Object) -> bool {
        for local_obj in &self.top_level_objects {
            if obj.path == local_obj.path {
                return true;
            }
        }
        return false;
    }
}
