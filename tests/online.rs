use pgvm::online::get_versions;

#[test]
fn versions() {
    let x = get_versions().unwrap();
    for ver in x {
        println!("{ver}")
    }
}
