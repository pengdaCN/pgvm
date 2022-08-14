use pgvm::data::Version;

#[test]
fn insert() {
    let x = Version {
        name: "go".to_string(),
        arch: "amd64".to_string(),
        os: "linux".to_string(),
        v1: 1,
        v2: 19.into(),
        v3: None,
        unstable_v4: None,
        size: 121323,
        sha256: "1".to_string(),
    };

    let db = sled::open("./data.db").unwrap();

    db.insert("s1", bincode::serialize(&x).unwrap()).unwrap();
}

#[test]
fn get() {
    let db = sled::open("./data.db").unwrap();

    let value = db.get("s1").unwrap().unwrap();

    let v: Version = bincode::deserialize(value.as_ref()).unwrap();

    println!("{v:?}")
}
