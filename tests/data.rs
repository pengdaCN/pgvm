use pgvm::data::UnstableVersion::RC;
use pgvm::data::Version;

#[test]
fn show_version() {
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
    println!("{}", x)
}
