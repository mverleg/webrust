use ::std::fs::File;
use ::std::io::BufReader;
use ::std::io::BufWriter;
use ::std::path::Path;
use ::std::path::PathBuf;
use ::std::sync::Arc;
use ::std::sync::LazyLock;

use ::serde::Deserialize;
use ::serde::Serialize;
use ::tokio::sync::RwLock;

static CONF: LazyLock<ConfContainer> = LazyLock::new(|| ConfContainer { conf: RwLock::new(None) });

#[derive(Debug)]
pub struct ConfContainer {
    conf: RwLock<Option<(PathBuf, Arc<Conf>)>>,
}

impl ConfContainer {
    fn get(&mut self, pth: &Path) -> Arc<Conf> {
        let mut conf_ref = self.conf.get_mut();
        match conf_ref {
            Some((_, conf)) => conf.clone(),
            None => {
                let new_conf = Arc::new(Conf::load_from_disk(pth).unwrap_or_default());
                *conf_ref = Some((pth.to_owned(), new_conf.clone()));
                new_conf
            }
        }
    }

    fn set(&mut self, pth: &Path) -> Conf {
        todo!()
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Conf {
    name: String,
    score: u8,
}

impl Conf {
    fn load_from_disk(pth: &Path) -> Option<Conf> {
        let reader = BufReader::new(File::options().write(false).open(pth).ok()?);
        serde_json::from_reader(reader).ok()
    }

    fn save_to_disk(pth: &Path, conf: &Conf) {
        let writer = BufWriter::new(File::options().write(true).create(true).open(pth).unwrap());
        serde_json::to_writer_pretty(writer, conf).unwrap()
    }
}