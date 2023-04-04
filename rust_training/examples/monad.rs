type Try<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn hello() -> Try<i32> {
    Ok(10)
}

fn parse(s: String) -> Try<i32> {
    Ok(s.parse()?)
}

fn flatten<T>(v: Try<Try<T>>) -> Try<T> {
    match v {
        Ok(v) => v,
        Err(err) => Err(err),
    }
}

fn flat_map<T, U>(t: Try<T>, f: impl Fn(T) -> Try<U>) -> Try<U> {
    flatten(t.map(f))
}

fn map_vector<T, U>(v: Vec<T>, f: impl Fn(T) -> U) -> Vec<U> {
    v.into_iter().map(f).collect()
}

fn flatten_vec<T>(v: Vec<Vec<T>>) -> Vec<T> {
    let ret = v.into_iter().flat_map(|l| l.into_iter());
    ret.collect()
}

fn flatmap_vec<T, U>(v: Vec<T>, f: impl Fn(T) -> Vec<U>) -> Vec<U> {
    flatten_vec(map_vector(v, f))
}

fn main() {
    let v = hello();
    let t = v.map(|x| format!("{}", x));

    //let t2 = flat_map(t, |x| parse(x));

    let t2 = t.and_then(|x| parse(x));

    let l = vec![1, 2, 3];

    let vv = vec![vec![1, 2, 3], vec![4, 5, 6]];

    let res = flatmap_vec(l, |x| vec![x, x]);

    println!("{:?}", res);
}
