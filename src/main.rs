mod cmdline;
use clap::Parser;
use cmdline::Args;
use rusqlite::{Connection, Result};
use std::io::Write;
use std::{
    collections::HashSet,
    fmt::Display,
    fs,
    path::PathBuf,
    process::{Command, Stdio},
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
            panic!("[Profile] Cannot find any profiles!");
        }
    };
    root.push("places.sqlite");
    let tmp_path = PathBuf::from("/tmp/firefox-dmenu-places.db");
    match fs::exists(&root) {
        Ok(existence) => {
            if !existence {
                panic!("[Filesystem] Database does not exist!")
            } else {
                let _ = fs::copy(root, &tmp_path);
                return Some(tmp_path);
            }
        }
        Err(_) => {
            panic!("[Filesystem] Nondescript error.")
        }
    };
}
fn main() -> Result<()> {
    let args = Args::parse();
    let path = if let Some(p) = copy_db(Some(&args.profile)) {
        p
    } else {
        panic!("[main] Database copying failed.")
    };
    let conn = Connection::open(&path).unwrap();
    let query = format!(
        "SELECT id, url FROM moz_places ORDER BY last_visit_date DESC LIMIT {}",
        args.limit
    );
    let mut stmt = conn.prepare(&query).unwrap();
    let urls_iter = stmt.query_map([], |row| {
        Ok(FirefoxPlace {
            id: row.get(0)?,
            url: row.get(1)?,
        })
    })?;
    let mut hosts: HashSet<String> = HashSet::new();
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
    /*for h in &hosts {
        println!("{}", h);
    }*/
    let tmp_urls_path = PathBuf::from("/tmp/firefox-dmenu-urls.tmp");
    let _ = fs::remove_file(&path);
    let dmenu_opts = hosts.into_iter().collect::<Vec<String>>().join("\n");
    /*println!(
        "cat {} | {}",
        tmp_urls_path.to_str().unwrap(),
        args.dmenu.to_str().unwrap()
    );*/
    let _ = fs::write(&tmp_urls_path, &dmenu_opts);
    let source_command = Command::new("cat")
        .arg(&tmp_urls_path)
        .stdout(Stdio::piped())
        .output()
        .expect("[cat] Failed to cat source file. Is it readable?");
    let mut dmenu_command = Command::new(args.dmenu)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("[dmenu] Failed to launch dmenu. Is it executable?");
    let mut dmenu_stdin = dmenu_command
        .stdin
        .take()
        .expect("[dmenu] Failed to take standard input from dmenu.");
    std::thread::spawn(move || {
        dmenu_stdin
            .write_all(&source_command.stdout)
            .expect("[dmenu] Failed to write to stdin of dmenu.");
    });
    let dmenu_out = dmenu_command
        .wait_with_output()
        .expect("[dmenu] Expected dmenu to successfully execute.");
    let dmenu_sel = String::from_utf8_lossy(&dmenu_out.stdout);
    /*println!("Captured selection: {}", dmenu_sel);
    println!("{}", dmenu_sel);*/
    if dmenu_sel.to_string() == String::from("") {
        println!("dmenu did not produce any output.");
        return Ok(());
    }
    let _ = Command::new(args.browser)
        .arg(dmenu_sel.to_string())
        .spawn();
    Ok(())
}
