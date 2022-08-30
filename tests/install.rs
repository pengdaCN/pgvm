use pgvm::data::Db;
use pgvm::install::install;
use pgvm::online::{open_version, verify_version};
use std::fs::OpenOptions;
use std::io::{copy, Seek, SeekFrom};
use std::path::Path;

#[test]
fn einstall() {
    let db = Db::new("/tmp/versions").unwrap();
    let vers = db.get_versions(Some("linux"), Some("amd64")).unwrap();
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
    copy(&mut online_file, &mut local_file).unwrap();

    local_file.seek(SeekFrom::Start(0)).unwrap();
    verify_version(go_latest, &local_file).unwrap();

    local_file.seek(SeekFrom::Start(0)).unwrap();

    install(&mut local_file, Path::new("/tmp/go1.19")).unwrap();

    println!("install OK")
}
