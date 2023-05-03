use ::std::env;
use ::std::fs;
use ::std::io;
use ::std::path::PathBuf;
use ::std::sync::Arc;
use ::std::sync::LazyLock;

use ::base64::Engine;
use ::base64::engine::general_purpose;
use ::dashmap::DashMap;
use ::sha2::Digest;
use ::sha2::Sha256;

pub static DOMAIN: LazyLock<String> = LazyLock::new(||
    env::var("WEBRUST_DOMAIN").unwrap_or_else(|_| "localhost:8080".to_owned()));

//static RESOURCE_HASHES: Arc<DashMap<String, String>> = Arc::new(DashMap::new());

pub static CSS_PATHS: LazyLock<Vec<String>> = LazyLock::new(|| {
    ["style.css"].into_iter()
        .map(|name| {
            let mut pth = PathBuf::from("static");
            pth.push(name);
            eprintln!("{:?}", &pth);  //TODO @mark: TEMPORARY! REMOVE THIS!
            let mut hasher = Sha256::new();
            let mut file = fs::File::open(&pth).unwrap_or_else(|_| panic!("cannot find css file {:?}", &pth));
            let _bytes_written = io::copy(&mut file, &mut hasher).unwrap();
            let hash_bytes = hasher.finalize();
            let hash = general_purpose::URL_SAFE_NO_PAD.encode(hash_bytes)[..16].to_owned();
            format!("{}?v={hash}", pth.to_str().expect("css file path not utf safe"))
        })
        .collect()
});

