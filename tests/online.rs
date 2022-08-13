use pgvm::online::get_versions;

#[test]
fn versions() {
    let x = get_versions().unwrap();
    println!("数量 {}", x.len());
    for ver in x {
        println!("{ver}")
    }
}
