pub struct Object {
    pub path: String,
    pub size_bytes: u32,
}
impl Object {
    pub fn new(path: &String, size: u32) -> Object {
        Object { path: path.to_string(), size_bytes: size }
    }
}
