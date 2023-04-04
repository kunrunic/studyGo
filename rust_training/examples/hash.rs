use std::collections::HashMap;

fn main() {
    //  let mut map = HashMap::from([
    //      ("b", (1usize, true)),
    //      ("c", (2, false)),
    //      ("d", (3, true)),
    //      ("e", (4, false)),
    //  ]);
    let mut map: HashMap<&str, (usize, bool)> = HashMap::new();
    for ele in map.iter().enumerate() {
        println!("{:?}", ele);
    }
    for val in map.values() {
        println!("{:?}", val);
    }

    // Get
    let m = map.get(&"c");
    println!("get : {:?}", m);

    // Get map or
    let m = map.get(&"c").map_or(false, |x| (*x).1);
    println!("get find return val : {:?}", m);

    // Restructure condition true
    let m = map
        .iter()
        .flat_map(|(k, (i, v))| v.eq(&true).then(|| (*k, (*i, *v))))
        .collect::<HashMap<&str, (usize, bool)>>();
    println!("restructure is true : {:?}", m);

    // Added New Item
    map.entry("a")
        .and_modify(|(i, v)| *v = true)
        .or_insert((5usize, true));
    for ele in map.iter().enumerate() {
        println!("{:?}", ele);
    }

    //  Restructre
    let m = map
        .iter()
        .flat_map(|(k, (i, v))| v.eq(&true).then(|| (*i, (*k, *v))))
        .collect::<HashMap<usize, (&str, bool)>>();
    println!("restructure is true : {:?}", m);

    let i = 2;
    let r = m.get(&i).and_then(|(k, v)| (*v).eq(&true).then(|| *k));
    if let None = r {
        println!("IS NONE Case");
    }
    println!("{:?}", r);
}
