use pgvm::data::UnstableVersion::RC;
use pgvm::data::{Compress, Db, Version};
use pgvm::online::get_versions;

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
        compress: Compress::TarGz,
    };
    println!("{}", x);
    println!("{}", x.short_version())
}

#[test]
fn store() {
    let db = Db::new("/tmp/versions").unwrap();

    let versions = get_versions().unwrap();

    db.store(versions).unwrap();

    println!("OK")
}

#[test]
fn versions() {
    let db = Db::new("/tmp/versions").unwrap();

    for x in db.get_versions(Some("linux"), Some("amd64")).unwrap() {
        println!("{x}")
    }
}
