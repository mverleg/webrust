use ::std::fs::File;
use ::std::io::BufReader;
use ::std::io::BufWriter;
use ::std::path::Path;
use ::std::path::PathBuf;
use ::std::sync::Arc;
use ::std::sync::LazyLock;
use ::std::sync::Mutex;

use ::serde::Deserialize;
use ::serde::Serialize;

pub static CONF: LazyLock<ConfContainer> = LazyLock::new(ConfContainer::empty);

#[derive(Debug)]
pub struct ConfContainer {
    conf: Mutex<Option<(PathBuf, Arc<Conf>)>>,
}

impl ConfContainer {
    pub fn empty() -> Self {
        ConfContainer { conf: Mutex::new(None) }
    }

    pub fn get(&self, pth: &Path) -> Arc<Conf> {
        let mut conf_ref = self.conf.lock().unwrap();
        match &mut *conf_ref {
            Some((conf_pth, conf)) => {
                assert_eq!(pth, conf_pth);
                conf.clone()
            },
            None => {
                let new_conf = Arc::new(Conf::load_from_disk(pth).unwrap_or_default());
                *conf_ref = Some((pth.to_owned(), new_conf.clone()));
                new_conf
            }
        }
    }

    pub fn set(&self, pth: &Path, new_conf: Conf) {
        let new_conf = Arc::new(new_conf);
        let new_state = Some((pth.to_owned(), new_conf.clone()));
        let mut conf_ref = self.conf.lock().unwrap();
        if let Some((conf_pth, _)) = &*conf_ref {
            assert_eq!(pth, conf_pth);
        }
        Conf::save_to_disk(pth, &*new_conf);
        *conf_ref = new_state
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
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