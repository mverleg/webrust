use ::std::env;
use ::std::fs;
use ::std::io;
use ::std::path::PathBuf;
use ::std::sync::Arc;
use ::std::sync::LazyLock;

use ::dashmap::DashMap;
use ::sha2::Sha256;
use base64::{Engine as _, engine::general_purpose};
use sha2::Digest;

pub static DOMAIN: LazyLock<String> = LazyLock::new(||
    env::var("WEBRUST_DOMAIN").unwrap_or_else(|_| "localhost:8080".to_owned()));

//static RESOURCE_HASHES: Arc<DashMap<String, String>> = Arc::new(DashMap::new());

pub static CSS_PATHS: LazyLock<Vec<String>> = LazyLock::new(|| {
    ["style.css"].into_iter()
        .map(|name| {
            let mut pth = PathBuf::from("resources");
            pth.push(name);
            let mut hasher = Sha256::new();
            let mut file = fs::File::open(pth).unwrap_or_else(|_| panic!("cannot find css file {name}"));
            let _bytes_written = io::copy(&mut file, &mut hasher).unwrap();
            let hash_bytes = hasher.finalize();
            general_purpose::URL_SAFE_NO_PAD.encode(hash_bytes)[..16].to_owned()
        })
        .collect()
});

