use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct Data {
    d: Option<String>,
}

fn get(ar: Arc<Mutex<Data>>) {
    let mut m = ar.lock().unwrap();
    *m = Data {
        d: Some("test".into()),
    };
}

fn get2(ar: Arc<Mutex<Data>>) {
    let mut m = ar.lock().unwrap();
    *m = Data {
        d: Some("test222".into()),
    };
}

fn main() {
    {
        let ar: Arc<Mutex<Data>> = Arc::new(Mutex::new(Data { d: None }));
        let arc = ar.clone();
        {
            get(ar);
        }
        get2(arc.clone());
        println!("{:?}", arc);
    }
}
