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

static CONF: LazyLock<Container<Conf>> = LazyLock::new(|| Container::<Conf> { conf: RwLock::new(None) });

#[derive(Debug)]
pub struct Container<T> {
    conf: RwLock<Option<(PathBuf, Arc<T>)>>,
}

impl <T: Default> Container<T> {
    fn get(&mut self, pth: &Path) -> Arc<T> {
        let mut val_ref = self.conf.get_mut();
        match val_ref {
            Some((old_pth, val)) => {
                assert_eq!(pth, old_pth);
                val.clone()
            }
            None => {
                let new_val = Arc::new(Conf::load_from_disk(pth).unwrap_or_default());
                *val_ref = Some((pth.to_owned(), new_val.clone()));
                new_val
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
    //TODO @mark: make these not public
    fn load_from_disk(pth: &Path) -> Option<Conf> {
        let reader = BufReader::new(File::options().write(false).open(pth).ok()?);
        serde_json::from_reader(reader).ok()
    }

    //TODO @mark: make these not public
    fn save_to_disk(pth: &Path, conf: &Conf) {
        let writer = BufWriter::new(File::options().write(true).create(true).open(pth).unwrap());
        serde_json::to_writer_pretty(writer, conf).unwrap()
    }
}
