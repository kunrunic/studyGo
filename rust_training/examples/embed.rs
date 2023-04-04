use std::ops::{Deref, DerefMut};
use test_mod::emb::EmbStruct;
mod test_mod;

#[derive(Debug)]
struct Embed(EmbStruct);

impl Deref for Embed {
    type Target = EmbStruct;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Embed {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Embed {
    fn new(name: String) -> Embed {
        Embed(EmbStruct { name })
    }
    fn update(&mut self, name: String) {
        self.name = name;
    }
}

fn main() {
    let mut n = Embed::new("test".into());
    println!("{:?}", n);
    n.dis(); // Deref 구현 해야 이런식으로 접근이 가능함
    n.0.dis(); // Deref 구현이 없다면 이렇게 접근이 가능함, embed는 아니겠지..?
    n.update("update name".into());
    n.dis();
}
