use std::fs;
use crate::errors::{Error, Reason, Result};
use compress_tools::{uncompress_archive, Ownership};
use std::fs::rename;
use std::io::{BufRead, BufReader, Read, Seek, Write};
use std::path::Path;

// 插入一行到文件中，如果该行不存在
fn exits_line_or_install<P: AsRef<Path>>(p: P, line: &str) -> Result<()> {
    let mut f = fs::OpenOptions::new().read(true).write(true).append(true).open(p)?;
    if BufReader::new(&f).lines().any(|x| {
        if let Ok(t) = x {
            if t == line {
                return true;
            }
        }

        false
    }) {
        return Ok(());
    }

    f.write_all(line.as_bytes())?;

    Ok(())
}

pub fn install<R: Read + Seek, D: AsRef<Path>>(r: &mut R, dest: D) -> Result<()> {
    let path = if let Some(name) = dest.as_ref().file_name() {
        dest.as_ref()
            .with_file_name(&format!("{}.bak", name.to_str().unwrap()))
    } else {
        return Err(Error {
            kind: Reason::InvalidInstallPath,
            msg: String::from("无效的安装路径"),
        });
    };

    // 解压
    uncompress_archive(r, &path, Ownership::Preserve)?;
    // 重命到指定位置
    rename(&path, dest)?;

    Ok(())
}


#[cfg(test)]
mod tests {
    use crate::install::exits_line_or_install;

    #[test]
    fn test_exits_line_or_install() {
        exits_line_or_install("/tmp/1.txt", "line 2").unwrap();
    }
}