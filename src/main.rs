use feed_rs::parser;
use reqwest;
use std::{process::Command, time::Duration};

use std::fs;
use std::fs::File;
use std::io::prelude::*;

use std::env;

use clokwerk::{AsyncScheduler, TimeUnits};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct Config {
    feeds: Vec<String>,
    download_path: String,
}

#[derive(Serialize, Deserialize)]
struct Downloaded {
    downloaded: Vec<String>,
}

fn get_config(path: String) -> Config {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    let config: Config = toml::from_str(&contents).unwrap();

    config
}

fn get_downloaded(path: &Option<String>) -> Downloaded {
    let path = match path {
        Some(path) => path,
        None => "downloaded.toml",
    };

    let contents = fs::read_to_string(path);

    match contents {
        Ok(contents) => {
            let downloaded: Downloaded = toml::from_str(&contents).unwrap();

            return downloaded;
        }
        Err(_) => {
            return Downloaded {
                downloaded: Vec::new(),
            }
        }
    }
}

fn store_downloaded(downloaded: Downloaded, path: &Option<String>) {
    let path = match path {
        Some(path) => path,
        None => "downloaded.toml",
    };

    let contents = toml::to_string(&downloaded).unwrap();

    let mut file = File::create(path).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}

async fn download(
    config: Config,
    downloaded_path: &Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut downloaded = get_downloaded(&downloaded_path);

    for feed_url in &config.feeds {
        let body = reqwest::get(feed_url).await?.text().await?;

        let feed = parser::parse(body.as_bytes());

        match feed {
            Ok(feed) => {
                let first_entry = feed.entries.first().unwrap();
                let first = first_entry.links.first().unwrap().href.to_string();

                let video_id = first_entry.id.to_string();

                if downloaded.downloaded.contains(&video_id) {
                    println!("Already downloaded: {:?}", first_entry.title);
                    continue;
                }

                let download_result = Command::new("youtube-dl")
                    .arg(first)
                    .arg("-o")
                    .arg(&config.download_path)
                    .output();

                match download_result {
                    Ok(_) => {
                        println!("Downloaded: {:?}", first_entry.title);
                        downloaded.downloaded.push(video_id);
                    }
                    Err(_) => {
                        println!("Error: {:?}", first_entry.title);
                    }
                };
            }
            Err(_) => {
                println!("Error download feed: {:?}", feed_url);
            }
        };
    }

    store_downloaded(downloaded, &downloaded_path);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut scheduler = AsyncScheduler::new();

    scheduler.every(60.minutes()).run(|| async {
        println!("Run task:");

        let args: Vec<String> = env::args().collect();
        let config_file_path = args[1].to_string();
        let downloaded_path: Option<String>;

        if args.len() > 1 {
            downloaded_path = Some(args[2].to_string());
        } else {
            downloaded_path = None;
        }

        let _result = download(get_config(config_file_path), &downloaded_path).await;
    });

    loop {
        scheduler.run_pending().await;
        tokio::time::sleep(Duration::from_secs(20)).await;
    }
}
