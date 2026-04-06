use toml;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
enum StreamConfig {
    #[serde(
        alias = "sub",
        alias = "subStream",
        alias = "substream",
        alias = "SubStream"
    )]
    Sub,
}

#[derive(Debug, Deserialize)]
struct Config {
    stream: StreamConfig,
}

fn main() {
    let toml_str = r#"stream = "sub""#;
    let config: Result<Config, _> = toml::from_str(toml_str);
    println!("{:?}", config);
    let toml_str2 = r#"stream = "Sub""#;
    let config2: Result<Config, _> = toml::from_str(toml_str2);
    println!("{:?}", config2);
}
