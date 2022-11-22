use fs_extra::dir::{DirEntryValue, DirEntryAttr};
use crate::origin::OriginType;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Object {
    pub origin_type: OriginType,
    // pub origin_path: String,
    pub path: String,
    pub size: u64,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)] // TODO remove clone?
pub enum ActionType {
    Remove,
    MoveOut,
    MoveIn,
    CopyIn,
}


impl Object {
    pub fn from_ls_result(
        _origin_type: OriginType,
        ls_res_map: &HashMap<DirEntryAttr, DirEntryValue>) -> Object {
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
        Object{
            origin_type: _origin_type,
            path: name.to_string(),
            size: *size as u64,
        }
    }
}
