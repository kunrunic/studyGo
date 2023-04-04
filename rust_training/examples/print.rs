use std::collections::HashMap;

use bytes::Bytes;

/// example : https://webnautes.tistory.com/1726

fn dis_primeval() {
    println!(
        r#"
	=============================
	TEST Info
	=============================
		Message 1 : {}
		M         : {}
		TTTTT     : {}
	"#,
        "MSG1", "MSG2", "MSG3"
    );
}
fn dis_line_multi() {
    println!(
        "
	 =============================
	 TEST Info
	 =============================
		Message 1 : {}
		M         : {}
		TTTTT     : {}
	",
        "MSG1", "MSG2", "MSG3"
    );
}

fn dis_line_multi2() {
    println!(
        "=============================\n\
	 TEST Info\n\
	 =============================\n\
		Message 1 : {}\n\
		M         : {}\n\
		TTTTT     : {}\n\
	",
        "MSG1", "MSG2", "MSG3"
    );
}

fn dis_vec() {
    let vec = vec!["1", "2", "3"];
    println!("{:#?}", vec);
}

fn dis_hash_mpa() {
    let mut hash: HashMap<String, String> = HashMap::new();
    hash.insert("key - 1".into(), "value - 1".into());
    hash.insert("key - 2".into(), "value - 2".into());
    hash.insert("key - 3".into(), "value - 3".into());
    hash.insert("key - 4".into(), "value - 4".into());
    hash.insert("key - 5".into(), "value - 5".into());
    println!(
        "
	Hash : {:?}
			",
        hash
    );
    println!("Hash : {:?}", hash);
}

type Try<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;
fn dis_custom1() -> Try<()> {
    let s = Bytes::from_static(b"{\n\t\"supi\": \"imsi-450080220000720\",\n\t\"gpsi\": \"msisdn-821029771800\",\n\t\"notifUri\": \"http://172.20.100.71:62150/chfservice/spending-limit\",\n\t\"supportedFeatures\": \"F\"\n}");

    let p = serde_json::from_slice::<HashMap<String, String>>(&s)?;
    let mut c = p
        .iter()
        .map(|(k, v)| format!("{:18}: {}", k, v))
        .collect::<Vec<String>>();
    c.sort();

    println!("{}", c.join("\n"));
    Ok(())
}

/// print format : https://doc.rust-lang.org/std/fmt/
fn main() {
    dis_primeval();
    dis_line_multi();
    dis_line_multi2();
    dis_vec();
    dis_hash_mpa();
    let _ = dis_custom1();
}
