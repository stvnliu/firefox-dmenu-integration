mod cmdline;
use clap::Parser;
use cmdline::Args;
use rusqlite::{params, Connection, Result};
use std::{
    collections::HashSet,
    fmt::Display,
    fs,
    path::{Path, PathBuf},
};
use url;
#[derive(Debug)]
struct FirefoxPlace {
    id: u64,
    url: String,
}
struct NoProfileFoundError {}
impl Display for NoProfileFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "No suitable profile was found")
    }
}
fn match_firefox_profile() -> Result<PathBuf, NoProfileFoundError> {
    todo!()
}
fn copy_db(root_path: Option<&PathBuf>) -> Option<PathBuf> {
    let mut root = if let Some(p) = root_path {
        p.clone()
    } else {
        if let Ok(found_profile_path) = match_firefox_profile() {
            found_profile_path
        } else {
            panic!("thing");
        }
    };
    root.push("places.sqlite");
    let tmp_path = PathBuf::from("/tmp/firefox-dmenu-places.db");
    match fs::exists(&root) {
        Ok(existence) => {
            if !existence {
                panic!("[FS] DB does not exist")
            } else {
                fs::copy(root, &tmp_path);
                return Some(tmp_path);
            }
        }
        Err(_) => {
            panic!("[FS] something wrong...")
        }
    };
}
fn main() -> Result<()> {
    let args = Args::parse();
    let path = if let Some(p) = copy_db(Some(&args.profile)) {
        p
    } else {
        panic!("baddddd")
    };
    let conn = Connection::open(&path).unwrap();
    let query = format!("SELECT id, url FROM moz_places ORDER BY last_visit_date DESC LIMIT {}", args.limit);
    let mut stmt = conn.prepare(&query).unwrap();
    let urls_iter = stmt.query_map([], |row| {
        Ok(FirefoxPlace {
            id: row.get(0)?,
            url: row.get(1)?,
        })
    })?;
    let mut hosts = HashSet::new();
    for place in urls_iter {
        if let Ok(p) = place {
            let url = url::Url::parse(&p.url);
            if let Ok(u) = url {
                if let Some(s) = u.host_str() {
                    hosts.insert(s.to_string());
                }
            }
        }
    }
    for h in hosts {
        println!("{}", h);
    }
    fs::remove_file(&path);
    Ok(())
}
