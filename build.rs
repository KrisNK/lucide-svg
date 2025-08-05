#![allow(unused)]

use std::path::PathBuf;
use std::sync::Arc;

use serde::Deserialize;
use tokio::fs;
use tokio::sync::Mutex;
use tokio::task::JoinSet;

#[tokio::main]
async fn main() {
    build_icons().await;
    println!("cargo::rerun-if-changed=build.rs");
}

const FD_LIMIT: usize = 1024;
const LUCIDE_UNPKG: &str = "https://unpkg.com/lucide-static@latest";
const LUCIDE_UNPKG_META: &str = "https://unpkg.com/lucide-static@latest/icons/?meta";
const LUCIDE_CACHE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/target/lucide");

async fn build_icons() {
    fs::create_dir_all(LUCIDE_CACHE)
        .await
        .expect("failed to create cache");

    let meta_list = fetch_metadata().await;

    let mut code = String::new();
    for meta_chunk in meta_list.chunks(FD_LIMIT) {
        let mut jobs = JoinSet::new();
        for meta in meta_chunk {
            jobs.spawn(process_icon(meta.clone()));
        }
        let code_snippets = jobs.join_all().await.join("\n");
        code.push_str(&code_snippets);
    }

    fs::write(
        format!("{}/icons.rs", std::env::var("OUT_DIR").unwrap()),
        code,
    )
    .await
    .expect("failed to write code fragment");
}

#[derive(Deserialize, Clone)]
struct FileMeta {
    path: String,
    integrity: String,
}

async fn fetch_metadata() -> Vec<FileMeta> {
    #[derive(Deserialize)]
    struct Meta {
        files: Vec<FileMeta>,
    }

    let meta = reqwest::get(LUCIDE_UNPKG_META)
        .await
        .expect("failed to fetch metadata")
        .text()
        .await
        .expect("failed to read metadata");
    let meta: Meta = serde_json::from_str(&meta).expect("failed to parse metadata");

    meta.files
}

async fn process_icon(meta: FileMeta) -> String {
    let cached_path = format!(
        "{LUCIDE_CACHE}{}",
        meta.path.strip_prefix("/icons").unwrap()
    );

    let (svg, is_cached): (String, bool) = match fs::try_exists(&cached_path).await {
        Err(err) => panic!("failed to check if icon is cached; {err}"),
        Ok(true) => {
            let svg = fs::read_to_string(&cached_path)
                .await
                .expect("failed to read cached icon");

            if ssri::Integrity::from(&svg).to_string() == meta.integrity {
                (svg, true)
            } else {
                let url = format!("{LUCIDE_UNPKG}{}", &meta.path);
                let svg = reqwest::get(&url)
                    .await
                    .expect("failed to fetch icon")
                    .text()
                    .await
                    .expect("failed to read icon");
                (svg, false)
            }
        }
        _ => {
            let url = format!("{LUCIDE_UNPKG}{}", &meta.path);
            let svg = reqwest::get(&url)
                .await
                .expect("failed to fetch icon")
                .text()
                .await
                .expect("failed to read icon");
            (svg, false)
        }
    };

    if !is_cached {
        fs::write(&cached_path, &svg)
            .await
            .expect("failed to cache icon");
    }

    let ident = gen_ident(&meta.path);
    gen_snippet(ident, svg)
}

fn gen_ident(path: &str) -> String {
    let mut path = path
        .strip_prefix("/icons/")
        .unwrap()
        .strip_suffix(".svg")
        .unwrap()
        .chars()
        .peekable();

    let mut ident = String::new();
    let mut at_hyphen = false;
    let mut at_number = false;

    while let Some(c) = path.next() {
        match c {
            _ if ident.is_empty() => ident.push(c.to_ascii_uppercase()),
            '-' => {
                if let Some(nc) = path.peek() {
                    if nc.is_ascii_digit() && at_number {
                        ident.push('_');
                    }
                }
                at_hyphen = true;
            }
            _ if at_hyphen => {
                at_hyphen = false;
                at_number = c.is_ascii_digit();
                ident.push(c.to_ascii_uppercase());
            }
            _ => {
                at_number = c.is_ascii_digit();
                ident.push(c);
            }
        }
    }

    ident
}

fn gen_snippet(ident: String, svg: String) -> String {
    let mut snippet = String::new();

    snippet.extend(format!("#[derive(Debug, Clone, Copy)]\npub struct {ident};\n\n").chars());
    snippet.extend(format!("impl std::fmt::Display for {ident} {{\n").chars());
    snippet.push_str(
        "\tfn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {\n",
    );
    snippet.extend(format!("\t\twrite!(f, r#\"{svg}\"#)\n").chars());
    snippet.push_str("\t}\n}\n\n");
    snippet.extend(format!("impl LucideIcon for {ident} {{}}\n").chars());

    snippet
}
