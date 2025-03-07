use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::Error as ReqwestError;
use serde::Deserialize;
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

lazy_static! {
    static ref LOG_FILE: Mutex<Option<File>> = Mutex::new(None);
    static ref LOG_TXT_FILE: Mutex<Option<File>> = Mutex::new(None);
}

#[derive(Deserialize, Debug)]
struct Repo {
    name: String,
    html_url: String,
}

// JSONファイルを読み取り、データを結合する関数
fn read_json_files(
    dir: &Path,
    combined_data: &mut BTreeMap<String, BTreeMap<String, Vec<Value>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    for entry_result in fs::read_dir(dir)? {
        let entry: fs::DirEntry = entry_result?;
        let entry_path: PathBuf = entry.path();
        if entry_path.is_file() {
            let file: File = File::open(&entry_path)?;
            let data: serde_json::Value = serde_json::from_reader(file)?;
            let formatted_data: String = serde_json::to_string_pretty(&data)?;
            log_writeline(&format!(
                "Read data from {}: {}",
                entry_path.display(),
                formatted_data
            ));

            if let Some(map) = data.as_object() {
                if let Some(articles) = map.get("articles") {
                    if let Some(articles_array) = articles.as_array() {
                        // ファイルパスから年度と月を抽出
                        if let Some(grandparent_dir) = entry_path
                            .parent()
                            .and_then(|p| p.parent())
                            .and_then(|p| p.file_name())
                            .and_then(|name| name.to_str())
                        {
                            let year: String = grandparent_dir.replace("article-", "");
                            if let Some(parent_dir) = entry_path
                                .parent()
                                .and_then(|p| p.file_name())
                                .and_then(|name| name.to_str())
                            {
                                let month: String = parent_dir.to_string();
                                log_writeline(&format!("Year: {}, Month: {}", year, month));
                                let year_entry: &mut BTreeMap<String, Vec<Value>> =
                                    combined_data.entry(year).or_insert_with(BTreeMap::new);
                                let month_entry: &mut Vec<Value> =
                                    year_entry.entry(month).or_insert_with(Vec::new);
                                month_entry.extend(articles_array.clone());
                            } else {
                                log_writeline(&format!(
                                    "Invalid directory name format: {}",
                                    entry_path.parent().unwrap().display()
                                ));
                            }
                        } else {
                            log_writeline(&format!(
                                "Invalid directory name format: {}",
                                entry_path.parent().unwrap().parent().unwrap().display()
                            ));
                        }
                    }
                }
            }
        } else if entry_path.is_dir() {
            read_json_files(&entry_path, combined_data)?;
        } else {
            log_writeline(&format!("Skipping non-file entry: {:?}", entry_path));
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    log_writeline("Infinite Article Database");

    // tmpディレクトリのパス
    let tmp_dir: &Path = Path::new("tmp");

    // tmpディレクトリの中身を空にする
    if tmp_dir.exists() && tmp_dir.is_dir() {
        match fs::read_dir(tmp_dir) {
            Ok(entries) => {
                for entry_result in entries {
                    if let Ok(entry) = entry_result {
                        let path: PathBuf = entry.path();
                        if path.is_file() {
                            fs::remove_file(path)?;
                        } else if path.is_dir() {
                            fs::remove_dir_all(path)?;
                        }
                    }
                }
            }
            Err(err) => {
                log_writeline(&format!("Failed to read tmp directory: {}", err));
                return Ok(());
            }
        }
    } else {
        log_writeline("Couldn't find tmp directory.");
    }

    // logディレクトリのパス
    let log_dir: &Path = Path::new("log");

    // logディレクトリが存在しない場合は作成する
    if !log_dir.exists() {
        fs::create_dir_all(log_dir).unwrap_or_else(|err| {
            log_writeline(&format!("Failed to create log directory: {}", err));
            std::process::exit(1);
        });
    }

    // ログファイルが1個以上ある場合、古いログファイルを削除する
    let log_files: Vec<fs::DirEntry> = fs::read_dir(log_dir)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .collect();
    let max_log_files: usize = 1;
    if log_files.len() >= max_log_files {
        let mut log_files = log_files;
        log_files.sort_by_key(|entry| entry.metadata().unwrap().created().unwrap());
        for entry in log_files.iter().take(log_files.len() - max_log_files + 1) {
            fs::remove_file(entry.path()).unwrap_or_else(|err| {
                log_writeline(&format!("Failed to remove old log file: {}", err));
            });
        }
    }

    // 現在のUTC日付と時刻を取得
    let now: DateTime<Utc> = Utc::now();
    let formatted_now: String = now.format("%Y-%m-%d_%H-%M-%S").to_string();
    let log_file_name: String = format!("{}.log", formatted_now);
    let log_file_path: PathBuf = log_dir.join(log_file_name);

    // ログファイルを作成する
    let log_file: File = File::create(&log_file_path).unwrap_or_else(|err| {
        log_writeline(&format!("Failed to create log file: {}", err));
        std::process::exit(1);
    });

    {
        let mut log_file_guard: std::sync::MutexGuard<Option<File>> = LOG_FILE.lock().unwrap();
        *log_file_guard = Some(log_file);
    }

    // log.txtファイルを作成する（既存のファイルを上書きする）
    let log_txt_file: File = File::create("log.txt").unwrap_or_else(|err| {
        log_writeline(&format!("Failed to create log.txt file: {}", err));
        std::process::exit(1);
    });

    {
        let mut log_txt_file_guard: std::sync::MutexGuard<Option<File>> =
            LOG_TXT_FILE.lock().unwrap();
        *log_txt_file_guard = Some(log_txt_file);
    }

    let log_msg: String = format!("Log created at: {}", now);
    log_writeline(&log_msg);

    // GitHubのThe-Infinitysユーザーが公開しているリポジトリを取得
    log_writeline("Fetching GitHub repos...");
    let repos: Vec<Repo> = match get_github_repos("The-Infinitys").await {
        Ok(repos) => repos,
        Err(err) => {
            log_writeline(&format!("Failed to fetch GitHub repos: {}", err));
            return Ok(()); // エラーを返さずに処理を終了
        }
    };
    for repo in &repos {
        log_writeline(&format!("Repo: {} - URL: {}", repo.name, repo.html_url));
    }
    log_writeline(&format!("Fetched {} repos", repos.len()));
    // リポジトリ名が "article-" で始まり、その後に数字が続くリポジトリをフィルタリング
    let article_repos: Vec<Repo> = repos
        .into_iter()
        .filter(|repo| {
            let re: Regex = Regex::new(r"^article-\d+$").unwrap();
            re.is_match(&repo.name)
        })
        .collect();

    log_writeline(&format!("Found {} article repos", article_repos.len()));

    // 各リポジトリのアーカイブをダウンロード
    for repo in article_repos {
        log_writeline(&format!("Repo: {} - URL: {}", repo.name, repo.html_url));
        if let Err(err) = download_repo_archive(&repo, tmp_dir).await {
            log_writeline(&format!("Failed to download repo archive: {}", err));
            return Ok(()); // エラーを返さずに処理を終了
        }
    }

    // すべてのJSONファイルを読み取り、データを一つにまとめる
    let mut combined_data: BTreeMap<String, BTreeMap<String, Vec<Value>>> = BTreeMap::new();
    read_json_files(tmp_dir, &mut combined_data)?;

    // 結合されたデータをJSONファイルに書き込む
    let export_path: &Path = Path::new("export/article-data.json");
    let export_file: File = File::create(export_path)?;
    serde_json::to_writer_pretty(export_file, &combined_data)?;

    log_writeline("Exported combined data to export/article-data.json");

    Ok(())
}

// GitHub APIを使用してユーザーのリポジトリを取得
async fn get_github_repos(user: &str) -> Result<Vec<Repo>, ReqwestError> {
    let url: String = format!("https://api.github.com/users/{}/repos", user);
    let client: reqwest::Client = reqwest::Client::new();
    let repos: Vec<Repo> = client
        .get(&url)
        .header("User-Agent", "request")
        .send()
        .await?
        .json::<Vec<Repo>>()
        .await?;
    Ok(repos)
}

// 新しい関数を追加
async fn download_repo_archive(
    repo: &Repo,
    tmp_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    // 一時ディレクトリが存在しない場合は作成する
    if !tmp_dir.exists() {
        fs::create_dir_all(tmp_dir)?;
    }

    let client = reqwest::Client::new();
    let url = format!(
        "https://github.com/The-Infinitys/{}/archive/refs/heads/main.zip",
        repo.name
    );
    let response = client.get(&url).send().await?;
    if response.status().is_success() {
        let archive_content = response.bytes().await?;
        let archive_path = tmp_dir.join(format!("{}.zip", repo.name));
        let mut file = File::create(&archive_path)?;
        file.write_all(&archive_content)?;
        log_writeline(&format!(
            "Downloaded archive {} to {:?}",
            repo.name,
            archive_path
        ));
        // アーカイブを解凍する
        let archive_file = File::open(&archive_path)?;
        let mut archive = zip::ZipArchive::new(archive_file)?;
        let repo_dir = tmp_dir.join(&repo.name);
        fs::create_dir_all(&repo_dir)?;
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let outpath = repo_dir.join(
                file.mangled_name()
                    .components()
                    .skip(1)
                    .collect::<PathBuf>(),
            );
            if file.name().ends_with('/') {
                fs::create_dir_all(&outpath)?;
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        fs::create_dir_all(&p)?;
                    }
                }
                let mut outfile = File::create(&outpath)?;
                std::io::copy(&mut file, &mut outfile)?;
            }
        }
        // 解凍後にzipファイルを削除
        fs::remove_file(&archive_path)?;
        log_writeline(&format!("Deleted archive file {:?}", archive_path));

        // 解凍されたフォルダを参照し、画像ファイルとJSONファイル以外を削除
        clean_up_extracted_files(&repo_dir)?;

    } else {
        log_writeline(&format!(
            "Failed to download archive {}: HTTP {}",
            repo.name,
            response.status()
        ));
    }
    Ok(())
}

// 解凍されたフォルダを参照し、画像ファイルとJSONファイル以外を削除する関数
fn clean_up_extracted_files(dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    for entry_result in fs::read_dir(dir)? {
        let entry: fs::DirEntry = entry_result?;
        let entry_path: PathBuf = entry.path();
        if entry_path.is_file() {
            if !is_image_or_json_file(&entry_path) || !is_thumbnail_file(&entry_path) {
                fs::remove_file(&entry_path)?;
                log_writeline(&format!("Deleted file {:?}", entry_path));
            }
        } else if entry_path.is_dir() {
            clean_up_extracted_files(&entry_path)?;
        }
    }
    Ok(())
}

// 画像ファイルとJSONファイルかどうかを判定する関数
fn is_image_or_json_file(path: &Path) -> bool {
    if let Some(extension) = path.extension() {
        match extension.to_str().unwrap_or("").to_lowercase().as_str() {
            "png" | "jpg" | "jpeg" | "gif" | "bmp" | "json" => true,
            _ => false,
        }
    } else {
        false
    }
}

// ファイル名がthumbnailかどうかを判定する関数
fn is_thumbnail_file(path: &Path) -> bool {
    if let Some(file_name) = path.file_stem() {
        file_name == "thumbnail"
    } else {
        false
    }
}

// ログファイルにメッセージを書き込む
fn log_writeline(message: &str) {
    let mut log_file_guard: std::sync::MutexGuard<Option<File>> = LOG_FILE.lock().unwrap();
    if let Some(log_file) = log_file_guard.as_mut() {
        println!("{}", message);
        writeln!(log_file, "{}", message).unwrap_or_else(|err| {
            eprintln!("Failed to write to log file: {}", err);
        });
    }

    let mut log_txt_file_guard: std::sync::MutexGuard<Option<File>> = LOG_TXT_FILE.lock().unwrap();
    if let Some(log_txt_file) = log_txt_file_guard.as_mut() {
        writeln!(log_txt_file, "{}", message).unwrap_or_else(|err| {
            eprintln!("Failed to write to log.txt file: {}", err);
        });
    }
}
