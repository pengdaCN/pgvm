use pgvm::data::UnstableVersion::RC;
use pgvm::data::Version;
use pgvm::db::ExtKv;
use crate::db::ExtKv;

#[test]
fn store() {
    let db = sled::open("./data.db").unwrap();
    let x = Version {
        name: "x".to_string(),
        arch: "amd64".to_string(),
        os: "linux".to_string(),
        v1: 1,
        v2: Option::from(19),
        v3: None,
        unstable_v4: Some(RC(1)),
        size: 0,
        sha256: "".to_string(),
    };

    db.store("s2", &x).unwrap();

    let v1: Version = db.load("s1").unwrap().unwrap();

    println!("{v1:?}")
}