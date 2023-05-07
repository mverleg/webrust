use ::std::env;
use ::std::fs;
use ::std::io;
use ::std::path::Path;
use ::std::path::PathBuf;
use ::std::sync::LazyLock;

use ::base64::Engine;
use ::base64::engine::general_purpose;
use ::sha2::Digest;
use ::sha2::Sha256;

pub static DOMAIN: LazyLock<String> = LazyLock::new(||
    env::var("WEBRUST_DOMAIN").unwrap_or_else(|_| "http://localhost:8080".to_owned()));

//static RESOURCE_HASHES: Arc<DashMap<String, String>> = Arc::new(DashMap::new());

fn hash(pth: &Path) -> String {
    let mut hasher = Sha256::new();
    let mut file = fs::File::open(pth).unwrap_or_else(|_| panic!("cannot find image file {:?}", &pth));
    let _bytes_written = io::copy(&mut file, &mut hasher).unwrap();
    let hash_bytes = hasher.finalize();
    general_purpose::URL_SAFE_NO_PAD.encode(hash_bytes)[..10].to_owned()
}

pub static CSS_PATHS: LazyLock<Vec<String>> = LazyLock::new(|| {
    ["style.css", "pico.css"].into_iter()
        .map(|name| {
            let mut pth = PathBuf::from("static");
            pth.push(name);
            let mut url = PathBuf::from("/s");
            url.push(name);
            format!("{}?v={}",
                    url.to_str().expect("css file path not utf safe"),
                    hash(&pth))
        })
        .collect()
});

pub static LOGO_PATH: LazyLock<String> = LazyLock::new(|| {
    let name = "logo.png";
    let pth = PathBuf::from("static").join(name);
    let url = PathBuf::from("/s").join(name);
    format!("{}?v={}",
            url.to_str().expect("css file path not utf safe"),
            hash(&pth))
});

