use flate2::read::GzDecoder;
use reqwest::{
    blocking::Client,
    header::{AUTHORIZATION, HeaderMap, HeaderValue},
};
use serde::Deserialize;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, BufRead, BufReader},
    os::unix::fs::symlink,
    path::PathBuf,
};
use tar::Archive;

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

#[derive(Debug, Deserialize)]
struct Manifest {
    bin: String,
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

fn create_auth_client(pat: &str) -> Result<(Client, HeaderMap), String> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    let token = format!("token {}", pat);
    let header_value =
        HeaderValue::from_str(&token).map_err(|e| format!("Invalid header value: {}", e))?;
    headers.insert(AUTHORIZATION, header_value);
    Ok((client, headers))
}

fn fetch_bytes(url: &str, pat: &str) -> Result<Vec<u8>, String> {
    let (client, headers) = create_auth_client(pat)?;
    let res = client
        .get(url)
        .headers(headers)
        .send()
        .map_err(|e| format!("Request Failes: {}", e))?;
    res.bytes()
        .map(|b| b.to_vec())
        .map_err(|e| format!("Failed to read response: {}", e))
}

fn fetch_json<T: serde::de::DeserializeOwned>(url: &str, pat: &str) -> Result<T, String> {
    let bytes = fetch_bytes(url, pat)?;
    serde_json::from_slice(&bytes).map_err(|e| format!("JSON parse error: {}", e))
}

fn intro() {
    println!("Usage: program <command>");
    println!("Available commands: install, update, list, uninstall");
}

fn config_dir() -> Result<PathBuf, String> {
    let path = dirs::home_dir()
        .ok_or("failed to read home dir")?
        .join(".gip");
    Ok(path)
}

fn load_packages() -> Result<PackageDB, String> {
    let path = config_dir()?.join("packages.json");
    let data = fs::read_to_string(path).map_err(|e| format!("Failed to read packages: {}", e))?;
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
    let url = "https://raw.githubusercontent.com/manish-ach/packages/refs/heads/main/packages.json";
    let bytes = fetch_bytes(url, &env_config.pat)?;

    let package_file = config_dir()?.join("packages.json");
    fs::create_dir_all(package_file.parent().unwrap())
        .map_err(|e| format!("Failed to create directory: {}", e))?;
    fs::write(package_file, bytes).map_err(|e| format!("Failed to write packages.json: {}", e))?;

    Ok(())
}

fn download_bin(name: &str, url: &str, pat: &str) -> Result<(), String> {
    let bytes = fetch_bytes(url, pat)?;
    let gip_dir = config_dir()?;
    let extract_dir = gip_dir.join(name);

    if extract_dir.exists() {
        fs::remove_dir_all(&extract_dir).map_err(|e| format!("Failed to remove old dir: {}", e))?;
    }
    fs::create_dir_all(&extract_dir).map_err(|e| format!("Failed to create dir: {}", e))?;

    let gz = GzDecoder::new(&bytes[..]);
    let mut archive = Archive::new(gz);
    archive.unpack(&extract_dir).map_err(|e| e.to_string())?;

    let bin_path = extract_dir.join(name);

    if !bin_path.exists() {
        return Err(format!("binary {} not found after extraction", name));
    }

    let local_bin = dirs::home_dir()
        .ok_or("cannot read home dir")?
        .join(".local/bin");
    fs::create_dir_all(&local_bin).map_err(|e| format!("Failed to create .local/bin: {}", e))?;

    let link_path = local_bin.join(name);
    if link_path.exists() {
        fs::remove_file(&link_path).map_err(|e| format!("Failed to remove old symlink: {}", e))?;
    }

    symlink(&bin_path, &link_path).map_err(|e| format!("Failed to create symlink: {}", e))?;
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

    let file_url = format!(
        "https://raw.githubusercontent.com/manish-ach/packages/refs/heads/main/{}/{}",
        pkg.name, ver.version
    );
    let manifest_url = format!("{}/manifest.json", file_url);
    let manifest: Manifest = fetch_json(&manifest_url, &env_config.pat)?;
    let bin_url = format!("{}/{}", file_url, manifest.bin);
    download_bin(&pkg.name, &bin_url, &env_config.pat)?;

    Ok(())
}

fn process(args: &[String]) -> Result<(), String> {
    let pat_file = config_dir()?.join("pat.key");
    let vars =
        load_env(pat_file.to_str().unwrap()).map_err(|e| format!("failed to load PAT: {}", e))?;
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
        std::process::exit(1);
    }
}
