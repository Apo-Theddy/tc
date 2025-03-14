use dirs::home_dir;
use lru::LruCache;
use serde::{Deserialize, Serialize};
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::process::Command;
use std::sync::{LazyLock, Mutex};
use std::{env, fs};
use std::{num::NonZeroUsize, path::Path};

const CACHE_SIZE: usize = 10;

#[derive(Deserialize, Serialize, Debug)]
struct CacheData {
    data: Vec<(String, String)>,
}

fn get_cache_file_path() -> PathBuf {
    let home = home_dir().expect("Failed to get home directory");
    let config_dir = home.join(".config/tc");

    if !config_dir.exists() {
        create_dir_all(&config_dir).expect("Failed to create config directory");
    }

    config_dir.join("cache.json")
}

static CACHE: LazyLock<Mutex<LruCache<String, String>>> = LazyLock::new(|| {
    let mut cache = LruCache::new(NonZeroUsize::new(CACHE_SIZE).unwrap());
    let cache_file = get_cache_file_path();

    if let Ok(json_data) = fs::read_to_string(&cache_file) {
        if let Ok(cache_data) = serde_json::from_str::<CacheData>(&json_data) {
            for (key, value) in cache_data.data {
                cache.put(key, value);
            }
        }
    }

    Mutex::new(cache)
});

pub struct Cache {}

impl Cache {
    pub fn save_to_cache(dirpath: &str) {
        if let Some(existing_path) = Cache::get_from_cache(dirpath) {
            println!("Path exists in cache: {}", existing_path);

            if let Err(e) = env::set_current_dir(Path::new(&existing_path)) {
                eprintln!("Failed to change directory: {}", e);
            } else {
                let shell = env::var("SHELL").unwrap_or_default();
                let shell_name = Path::new(&shell)
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy();

                println!("Shell name: {}", shell_name);
                Command::new(shell_name.to_string())
                    .arg("-c")
                    .arg(format!("cd {} && exec bash", existing_path))
                    .status()
                    .expect("Failed to execute command");
                println!("Changed directory to: {}", existing_path);
            }
            return;
        }

        let home_dir = home_dir().expect("Failed to get home directory");
        let complete_path = format!("{}/{}", home_dir.display(), dirpath);
        let path = Path::new(&complete_path);

        if !path.exists() || !path.is_dir() {
            println!("Path does not exist or is not a directory");
            return;
        }

        let name_last_folder = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        if let Ok(mut cache) = CACHE.lock() {
            cache.put(name_last_folder.clone(), complete_path.clone());
            Cache::persist_cache(&cache);
            println!("Saved to cache: {}", name_last_folder);
        } else {
            eprintln!("Failed to lock cache");
        }
    }

    pub fn get_from_cache(dirname: &str) -> Option<String> {
        let mut cache = CACHE.lock().unwrap();
        cache.get(dirname).cloned()
    }

    pub fn persist_cache(cache: &LruCache<String, String>) {
        let cache_file = get_cache_file_path();

        let cache_data = CacheData {
            data: cache.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
        };

        if let Ok(json) = serde_json::to_string_pretty(&cache_data) {
            if let Err(e) = fs::write(&cache_file, json) {
                eprintln!("Failed to write cache to file: {}", e);
            }
        }
    }
}
