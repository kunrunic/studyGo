use test_mod::new::NewStruct;
mod test_mod;

#[derive(Debug)]
struct NewType(NewStruct);

impl NewType {
    fn new(name: String) -> NewType {
        NewType(NewStruct { name })
    }
}

impl From<NewStruct> for NewType {
    fn from(v: NewStruct) -> NewType {
        NewType(v)
    }
}

fn main() {
    let n = NewType::new("test".into());

    // NewType 으로 생성되고 NewStruct의 접근은 메서드를 별도로 구현할 필요가 있다.
    // 이 pattern 은 비용없는 추상화에 주요 목적이 있다.
    println!("{:?}", n);
    n.0.dis(); // NewStruct impl에 대해서 접근하려면 이렇게 할수 있다.

    // typecast을 제공하려면 from 함수를 구현해서 제공해야함
    let ns = NewType::from(NewStruct::new());
    println!("{:?}", ns);
}
