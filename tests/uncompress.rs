use compress_tools::{uncompress_archive, Ownership};
use std::fs;
use std::fs::OpenOptions;
use std::path::Path;

#[test]
fn uncompress() {
    fs::create_dir_all("/tmp/t_r_un_c").unwrap();
    let tar_gz_archive = OpenOptions::new().read(true).open("/tmp/1.zip").unwrap();

    let dest = Path::new("/tmp/t_r_un_c");
    uncompress_archive(&tar_gz_archive, dest, Ownership::Preserve).unwrap()
}
