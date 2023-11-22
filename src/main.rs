use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::io::Read;
use std::path::PathBuf;

fn main() {
    let cur_dir;
    // カレントディレクトリを取得
    match env::current_dir() {
        Ok(dir) => {
            if let Ok(abs_dir) = fs::canonicalize(&dir) {
                cur_dir = abs_dir;
            } else {
                println!("Failed to get absolute path of current directory");
                return;
            }
        }
        Err(e) => {
            println!("Failed to get current directory: {}", e);
            return;
        }
    }
    println!("Current directory: {}", cur_dir.display());

    // カレントディレクトリ内のファイルを走査
    let mut file_paths: Vec<PathBuf> = Vec::new();

    // カレントディレクトリ内のファイルを走査
    match fs::read_dir(cur_dir.clone()) {
        Ok(entries) => {
            // エラーの場合、Noneにフィルター変換する
            for entry in entries.filter_map(|e| e.ok()) {
                // エラーがないエントリに対してのみ処理を行う
                let path = entry.path();
                if let Some(extension) = path.extension().and_then(|s| s.to_str()) {
                    // 指定された拡張子を持つファイルの絶対パスをリストに追加
                    match extension {
                        "jpeg" | "jpg" | "png" | "webp" => {
                            if let Ok(abs_path) = fs::canonicalize(&path) {
                                file_paths.push(abs_path);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        Err(e) => {
            println!("Failed to read directory: {}", e);
            return;
        }
    }

    // 取得したファイルパスを表示
    for path in file_paths {
        match fs::File::open(&path) {
            Ok(mut file) => {
                let mut buffer = Vec::new();
                // ファイルの内容を読み込む
                match file.read_to_end(&mut buffer) {
                    Ok(_) => {
                        //drop(file);
                        let mut hasher = Sha256::new();
                        hasher.update(&buffer);
                        let hash = hasher.finalize();
                        match path.extension().and_then(|s| s.to_str()) {
                            Some(extension) => {
                                let new_name = format!("image-{:x}.{}", hash, extension);
                                // 新しいファイル名でリネームする
                                let new_path = cur_dir.clone().join(&new_name);
                                match fs::rename(&path, &new_path) {
                                    Ok(_) => {
                                        println!(
                                            "Renamed {} to {}",
                                            path.display(),
                                            new_path.display()
                                        );
                                    }
                                    Err(e) => {
                                        eprintln!("Failed to rename {}: {}", path.display(), e);
                                    }
                                }
                            }
                            None => {}
                        }
                    }
                    Err(e) => {
                        eprintln!("エラーが発生しました: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("エラーが発生しました: {}", e);
            }
        }
    }
}
