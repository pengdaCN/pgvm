use pgvm::data::Db;
use pgvm::online::{get_versions, open_version, verify_version};
use std::fs::OpenOptions;
use std::io::{copy, Seek, SeekFrom};

#[test]
fn versions() {
    let x = get_versions().unwrap();
    println!("数量 {}", x.len());
    for ver in x {
        println!("{ver}")
    }
}

#[test]
fn verify() {
    let db = Db::new("/tmp/versions").unwrap();
    let vers = db.versions(Some("linux"), Some("amd64")).unwrap();
    let go_latest = vers.first().unwrap();
    println!("go latest {go_latest}");

    let mut online_file = open_version(go_latest).unwrap();
    let mut local_file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .truncate(true)
        .open("/tmp/go_latest.tar.gz")
        .unwrap();
    copy(&mut online_file.0, &mut local_file).unwrap();

    local_file.seek(SeekFrom::Start(0)).unwrap();
    verify_version(go_latest, &local_file).unwrap();
}
