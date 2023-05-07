use ::std::path::PathBuf;

use ::clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "rustweb", about = "Experimental Rust server")]
pub struct Args {
    #[arg(default_value = "127.0.0.1:8080")]
    pub host: String,
    #[arg(default_value = "/tmp/webrust.conf.json")]
    pub conf_state_path: PathBuf,
}

#[test]
fn test_cli_args() {
    Args::try_parse_from(&["cmd"]).unwrap();
    Args::try_parse_from(&["cmd", "test.domain.tld:80"]).unwrap();
}
