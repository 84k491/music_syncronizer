extern crate fs_extra;

use crate::object::Object;
use fs_extra::dir::{ls, DirEntryAttr};
use std::collections::HashSet;
use std::path::Path;
use crate::object::ActionType;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)] // TODO remove clone?
pub enum OriginType {
    Source,
    Destination,
}

pub struct Pool {
    // origin_path: String, // TODO remove??
    origin_type: OriginType,
    top_level_objects: Vec<Object>,
    total_size: u64,
    actions: HashMap<ActionType, Vec::<Object>>,
}

impl Pool {
    pub fn new(t: OriginType) -> Pool {
        Pool {
            origin_type: t,
            top_level_objects: Vec::<Object>::new(),
            total_size: 0,
            actions: HashMap::new(),
        }
    }

    pub fn inflate_from_path(&mut self, path: &String) {
        if !Path::new(&path).exists() {
            panic!("No directory: {}", &path);
        }

        let mut config = HashSet::new();
        config.insert(DirEntryAttr::Name);
        config.insert(DirEntryAttr::Size);
        config.insert(DirEntryAttr::BaseInfo);
        let ls_res = ls(&path, &config).unwrap();

        let mut objects = Vec::<Object>::new();
        let mut total_size_local: u64 = 0;
        for res_map in &ls_res.items {
            let obj = Object::from_ls_result(self.origin_type, &res_map);
            println!("Entry: {}; Size: {}; total: {}", obj.path, obj.size, total_size_local);
            total_size_local += obj.size;
            objects.push(obj);
        }

        self.top_level_objects.append(&mut objects);
        self.total_size += total_size_local;
    }

    pub fn contains(&self, obj: &Object) -> bool {
        for local_obj in &self.top_level_objects {
            if obj.path == local_obj.path {
                return true;
            }
        }
        return false;
    }

    fn plan_action(&mut self, action_type: ActionType, object: &Object) {
        self.actions.entry(action_type).or_default().push((*object).clone());
        match action_type {
            ActionType::Remove => self.total_size -= object.size,
            ActionType::MoveOut => self.total_size -= object.size,
            _ => self.total_size += object.size,
           
        };
    }

    pub fn has_available_space(&self, space: u64) -> bool {
        space > 0 // TODO implement
    }

    pub fn extract_difference_with_single_pool(&mut self, pool: &Pool) -> Vec::<Object> {
        let mut result = Vec::<Object>::new();
        self.top_level_objects.retain(|obj_ref| {
            let do_keep = pool.contains(obj_ref);
            if !do_keep {
                result.push(obj_ref.clone());
            }
            do_keep
        });
        result
    }

    pub fn extract_difference_with_multiple_pools(&mut self, other_pools: &Vec::<Pool>) -> Vec::<Object> {
        let mut result = Vec::<Object>::new();
        self.top_level_objects.retain(|obj_ref| {
            let mut is_diff = true;
            for pool in other_pools {
                if pool.contains(&obj_ref) {
                    is_diff = false;
                    break;
                }
            }
            if is_diff {
                result.push(obj_ref.clone());
            }
            !is_diff
        });
        result
    }

    pub fn remove_difference(&mut self, other: &Pool) {
        let diff = self.extract_difference_with_single_pool(other);
        for obj in &diff {
            self.plan_action(ActionType::Remove, obj);
        }
    }

    pub fn push(&mut self, obj: Object) {
        let action_type = {
            match obj.origin_type {
                OriginType::Destination => ActionType::MoveIn,
                OriginType::Source => ActionType::CopyIn,
            }
        };
        self.plan_action(action_type, &obj);
        self.top_level_objects.push(obj);
    }

    fn extract_biggest_object_smaller_than(&mut self, _size: u64) -> Object {
        // TODO implement
        let obj = self.top_level_objects.remove(0);
        self.plan_action(ActionType::MoveOut, &obj);
        obj
    }

    pub fn extract_for_free_space(&mut self, additional_space_needed: u64) -> Vec::<Object> {
        let mut result = Vec::<Object>::new();
        let mut space_to_free = additional_space_needed as i64;
        while space_to_free > 0 {
            let obj = self.extract_biggest_object_smaller_than(space_to_free as u64);
            space_to_free -= obj.size as i64;
            result.push(obj);
        }
        result
    }

    pub fn invoke_actions_with_type(&self, _t: ActionType) {
        println!("Invoking some");
    }
}
