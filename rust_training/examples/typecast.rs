fn main() {
    let u: u64 = 1;
    let uu: usize = u as usize; // 기본 형은 as 가 거의 구현되어있음.
    println!("{}, {}", u, uu);
    let b: Vec<u8> = Vec::new();

    let bb: bytes::Bytes = b.clone().into(); // into 는 into, try_into 둘중 하나만 구현이 가능하다.
    println!("{:?}", bb);
    let bb: bytes::Bytes = bytes::Bytes::from(b); // as 가 구현되지 않는 경우 관습적으로 from이 구현되어있음

    let b: Vec<u8> = Vec::from(bb.as_ref()); //  Vec의 from은 다음과 같다 fn from(s: &[T]) -> Vec<T> , ref를 사용하여 재할당하지 않고 형을 변환한다.
    println!("{:?}", b);
}
