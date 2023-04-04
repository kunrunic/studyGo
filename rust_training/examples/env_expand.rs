use std::env;

fn find_env(i: usize, x: &str) -> Option<String> {
    if Some('$') != x.chars().next() {
        return Some(x.to_string());
    }
    let cv = x.replace("$", "").replace("{", "").replace("}", "");

    let f = env::vars_os()
        .find(|(k, _v)| {
            if let Some(ck) = k.to_str() {
                ck.eq(cv.as_str())
            } else {
                false
            }
        })
        .map(|(_k, v)| v);

    let v = f?;
    let vc = v.to_str()?;
    let mut res = x.replace(x, vc);
    // path keyword(/) audit
    if i != 0 {
        if res.find('/').unwrap_or(9999 as usize) == 0 {
            res.remove(0);
        }
        if res.rfind('/').unwrap_or(9999 as usize) == res.len() {
            res.remove(res.len());
        }
    }
    Some(res)
}

fn expand_env(text: String) -> String {
    let vec: Vec<&str> = text.split('/').collect();
    let r = vec
        .into_iter()
        .enumerate()
        .map(|(i, x)| find_env(i, x).unwrap_or_else(|| x.into()))
        .collect::<Vec<String>>();

    r.join("/")
}

fn main() {
    // {} 있는 유형
    let tlist = vec![
        "${HOME}",
        "${HOME}/test",
        "/test/${HOME}",
        "/t/${HOME}/test",
        "/t/${HOME}/test/$HOME",
    ];
    tlist
        .into_iter()
        .for_each(|v| println!("{} => {:?}", v, expand_env(v.to_string())));

    // {} 없는 유형
    let tlist = vec![
        "$HOME",
        "$HOME/test",
        "/test/$HOME",
        "/t/$HOME/test",
        "/t/$HOME/test/$HOME",
    ];
    tlist
        .into_iter()
        .for_each(|v| println!("{} => {:?}", v, expand_env(v.to_string())));

    // 예외.. $, {, } 잘못 기입 되는케이스, $로 시작하면 환경 변수로 치환한다.
    let tlist = vec![
        "$$HOME",
        "$HOME/test",
        "/test/$$HOME",
        "/t/$${{HOME}}/test",
        "/t/$HOME}/test/${HOME",
    ];
    tlist
        .into_iter()
        .for_each(|v| println!("{} => {:?}", v, expand_env(v.to_string())));
}
