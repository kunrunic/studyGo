use std::collections::HashMap;

fn basic() -> Result<(), String> {
    let vec: Vec<String> = Vec::new();

    // let m은 impl Iterator<Item = String> 형식으로 반환됨.
    let m = vec.into_iter().map(|v| v);
    println!("{:?}", m);
    let vec: Vec<String> = Vec::new();
    // 이걸 String<Iterator> 로 변경하기 위해서 collect를 사용한다.
    // 이건 traverse 처럼 impl Iterator<Item = String>  -> String<Iterator> 로 변경 해준다.
    // 형식은 지정 해야됨.
    let m2: Vec<String> = vec.into_iter().map(|v| v).collect();
    //collect는 처리 과정중 1개가 실패하면 모두 실패로 처리된다.
    println!("{:?}", m2);
    // 다른 모양으로 가공.
    let vec: Vec<String> = Vec::new();
    let m2: HashMap<String, String> = vec.into_iter().map(|v| (v, "val".into())).collect();
    println!("{:?}", m2);
    Ok(())
}

fn basic_applie() -> Result<(), String> {
    fn convert(s: &str) -> Result<(String, String), String> {
        let k = String::from(s);
        let v = "values".to_string();
        Ok((k, v))
    }

    let s = "test;test1;test2;test3";
    let sp: Vec<&str> = s.split(";").collect();
    let c: Result<HashMap<String, String>, String> = sp.iter().map(|v| convert(*v)).collect();
    println!("{:?}", c?);
    Ok(())
}

fn main() -> Result<(), String> {
    let _ = basic();
    let _ = basic_applie();
    Ok(())
}
