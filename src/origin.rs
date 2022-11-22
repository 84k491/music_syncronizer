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
    pub path: String,
    origin_type: OriginType,
    pub available_space: u64,
    top_level_objects: Vec<Object>,
    total_size: u64,
    actions: HashMap<ActionType, Vec::<Object>>,
}

impl Pool {
    pub fn new(t: OriginType) -> Pool {
        Pool {
            path: ("").to_string(),
            origin_type: t,
            available_space: 0,
            top_level_objects: Vec::<Object>::new(),
            total_size: 0,
            actions: HashMap::new(),
        }
    }

    pub fn inflate_from_path(&mut self, path: &String) {
        if !Path::new(&path).exists() {
            panic!("No directory: {}", &path);
        }
        self.path = path.to_string(); // TODO remove for Source
        self.available_space = fs2::available_space(self.path.to_string()).unwrap();

        let mut config = HashSet::new();
        config.insert(DirEntryAttr::FullName);
        config.insert(DirEntryAttr::Size);
        config.insert(DirEntryAttr::BaseInfo);
        let ls_res = ls(&path, &config).unwrap();

        let mut objects = Vec::<Object>::new();
        let mut total_size_local: u64 = 0;
        for res_map in &ls_res.items {
            let obj = Object::from_ls_result(self.origin_type, path, &res_map);
            total_size_local += obj.size;
            // println!("Entry {:?}: {}=>{}; Size: {}; total: {}", self.origin_type, path, obj.path, obj.size, total_size_local);
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
            ActionType::MoveOut | ActionType::Remove => {
                self.total_size -= object.size;
                self.available_space += object.size;
            },
            ActionType::MoveIn | ActionType::CopyIn => {
                self.total_size += object.size;
                self.available_space -= object.size;
            },
            _ => {},
        };
    }

    pub fn has_space_for_object(&self, obj: &Object) -> bool {
        self.available_space > obj.size
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

    fn extract_biggest_object_smaller_than(&mut self, required_size: u64) -> Option<Object> {
        let mut max_size: u64 = 0;
        let mut iter: i64 = -1; // TODO use option
        for (i, obj) in self.top_level_objects.iter().enumerate() {
            if obj.size < required_size && obj.size > max_size {
                iter = i as i64;
                max_size = obj.size;
            }
        }

        if iter < 0 {
            return None;
        }

        Some(self.top_level_objects.remove(iter as usize))
    }

    pub fn extract_for_free_space(&mut self, additional_space_needed: u64) -> Vec::<Object> {
        let mut result = Vec::<Object>::new();
        let mut space_to_free = additional_space_needed as i64;
        while space_to_free > 0 {
            match self.extract_biggest_object_smaller_than(space_to_free as u64) {
                Some(obj) => {
                    space_to_free -= obj.size as i64;
                    result.push(obj);
                },
                None => panic!("No object to cleanup space ({})", additional_space_needed),
            }
        }
        result
    }

    pub fn invoke_actions_with_type<F: Fn(&Object, &String)>(&self, t: ActionType, callback: F) {
        match self.actions.get(&t) {
            Some(vec) => {
                for obj in vec {
                    callback(obj, &self.path);
                }
            },
            None => {},
        }
    }
}
