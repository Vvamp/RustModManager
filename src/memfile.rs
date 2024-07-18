pub struct Memfile {
    pub filename: String,
    pub content: String,
}

impl Memfile {
    pub fn new(filename: String, content: String) -> Memfile {
        Memfile { filename, content }
    }
}
