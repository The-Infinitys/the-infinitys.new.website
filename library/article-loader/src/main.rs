use std::fs;
use std::io;
use std::path::Path;

fn main() {
    println!("Infinite Article Database");

    // tmpディレクトリのパス
    let tmp_dir = Path::new("tmp");

    // tmpディレクトリの中身を空にする
    if tmp_dir.exists() && tmp_dir.is_dir() {
        match fs::read_dir(tmp_dir) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        if path.is_file() {
                            fs::remove_file(path).unwrap_or_else(|err| {
                                eprintln!("Failed to remove file: {}", err);
                            });
                        } else if path.is_dir() {
                            fs::remove_dir_all(path).unwrap_or_else(|err| {
                                eprintln!("Failed to remove directory: {}", err);
                            });
                        }
                    }
                }
                println!("tmpディレクトリの中身を空にしました。");
            }
            Err(err) => {
                eprintln!("Failed to read tmp directory: {}", err);
            }
        }
    } else {
        println!("tmpディレクトリが存在しません。");
    }
}
