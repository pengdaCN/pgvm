use pgvm::data::Version;

#[test]
fn dec() {
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

    let v = bincode::serialize(&x).unwrap();
    println!("{v:?}")
}
