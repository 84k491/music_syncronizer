extern crate fs_extra;
use fs_extra::dir::get_size;
use fs_extra::dir::get_dir_content;

// struct Pool {
    // origin_type: OriginType,
    // folders: Vec<String>,
// }
// impl Pool {
    // fn new(origin_type: OriginType) -> Pool {
        // Pool{origin_type, {}};
    // }
// }

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum OriginType {
    Source,
    Destination,
}

pub struct PoolBuilder {
    origin_type: OriginType,
    path: String,
}

impl PoolBuilder {
    pub fn new(t: &OriginType, path: &String) -> PoolBuilder {
        PoolBuilder { origin_type: t.clone(), path: path.to_string() }
    }

    fn validate() -> bool {
        // check if path exists

        let folder_size = get_size("dir").unwrap();
        // let dir_content = get_dir_content("dir")?;
        println!("{}", folder_size);

        folder_size != 0
    }

    // fn get_pool() -> Pool {
        // Pool::new()
    // }
}
