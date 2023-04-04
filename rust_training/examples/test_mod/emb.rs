#[derive(Debug)]
pub struct EmbStruct {
    pub name: String,
}

impl EmbStruct {
    pub fn new() -> Self {
        Self { name: "new".into() }
    }
    pub fn dis(&self) {
        println!("name : {}", self.name);
    }
}
