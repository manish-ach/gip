use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue},
};
use serde::Deserialize;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

#[derive(Debug, Deserialize)]
struct PackageDB {
    packages: Vec<Package>,
}
#[derive(Debug, Deserialize)]
struct Package {
    name: String,
    versions: Vec<Version>,
}
#[derive(Debug, Deserialize)]
struct Version {
    version: String,
    latest: String,
    url: String,
    checksum: String,
    manifest: String,
}

struct EnvVars {
    pat: String,
}
impl EnvVars {
    fn from_map(map: HashMap<String, String>) -> Result<Self, String> {
        let pat = map
            .get("PAT")
            .ok_or("Missing Personal Acess Token")?
            .to_string();
        Ok(Self { pat })
    }
}

fn intro() {
    println!("Usage: program <command>");
    println!("Available commands: install, update, list, uninstall");
}

fn config_dir() -> Result<PathBuf, String> {
    let home = dirs::home_dir().ok_or("failed to read home dir")?;
    let path: PathBuf = home.join(".gip");
    Ok(path)
}

fn load_packages() -> Result<PackageDB, String> {
    let path = config_dir()?.join("packages.json");
    let data = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let parsed: PackageDB = serde_json::from_str(&data).map_err(|e| e.to_string())?;
    Ok(parsed)
}

pub fn load_env(path: &str) -> io::Result<HashMap<String, String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut map = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        if let Some((key, value)) = trimmed.split_once('=') {
            map.insert(key.trim().to_string(), value.trim().to_string());
        }
    }

    Ok(map)
}

fn refresh_packages_db(env_config: &EnvVars) -> Result<(), String> {
    let client = Client::new();
    let url =
        "https://raw.githubusercontent.com/manish-ach/gip/refs/heads/main/packages/packages.json";
    let mut headers = HeaderMap::new();
    let header_value = HeaderValue::from_str(&format!("token {}", env_config.pat)).unwrap();
    headers.insert(reqwest::header::AUTHORIZATION, header_value);

    let res = client
        .get(url)
        .headers(headers)
        .send()
        .map_err(|e| e.to_string())?;
    let bytes = res.bytes().map_err(|e| e.to_string())?;

    let path = config_dir()?;
    let package_file = path.join("packages.json");

    std::fs::create_dir_all(package_file.parent().unwrap()).map_err(|e| e.to_string())?;
    std::fs::write(package_file, bytes).map_err(|e| e.to_string())?;

    Ok(())
}

fn install(args: &[String], env_config: EnvVars) -> Result<(), String> {
    refresh_packages_db(&env_config)?;
    let db = load_packages()?;
    let pkg_name = args.get(2).ok_or("no file name specified")?;
    let version_req = args.get(3);

    let pkg = db
        .packages
        .iter()
        .find(|p| p.name == *pkg_name)
        .ok_or("package not found")?;

    let ver = match version_req {
        Some(v) => pkg
            .versions
            .iter()
            .find(|x| x.version == *v)
            .ok_or("version not found")?,
        None => pkg
            .versions
            .iter()
            .find(|x| x.latest == "true")
            .ok_or("latest version not found")?,
    };

    Ok(())
}

fn process(args: &[String]) -> Result<(), String> {
    let path = config_dir()?;
    let pat_file = path.join("pat.key");
    let vars = load_env(pat_file.to_str().unwrap()).map_err(|e| e.to_string())?;
    let env_config = EnvVars::from_map(vars)?;

    match args.get(1).map(String::as_str) {
        Some("install") => install(args, env_config)?,
        Some("update") => {}
        Some("list") => {}
        Some("uninstall") => {}
        Some(x) => return Err(format!("Unidentified parameter found {x}")),
        None => return Err(String::from("no params found")),
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        intro();
    } else if let Err(e) = process(&args) {
        eprintln!("Error: {e}");
    }
}
