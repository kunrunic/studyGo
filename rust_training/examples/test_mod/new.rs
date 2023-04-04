#[derive(Debug)]
pub struct NewStruct {
    pub name: String,
}

impl NewStruct {
    pub fn new() -> Self {
        Self { name: "new".into() }
    }
    pub fn dis(&self) {
        println!("name : {}", self.name);
    }
}
