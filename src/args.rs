use ::clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "rustweb", about = "Experimental Rust server")]
pub struct Args {
    #[arg(default_value = "127.0.0.1:8080")]
    pub host: String,
}

#[test]
fn test_cli_args() {
    AddArgs::try_parse_from(&["cmd"]).unwrap();
    AddArgs::try_parse_from(&["cmd", "test.domain.tld:80"]).unwrap();
}
