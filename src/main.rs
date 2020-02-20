use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use docopt::Docopt;
use serde_derive::Deserialize;
use std::fs;
use std::path::Path;

const VERSION: &'static str = "0.0.1";

const USAGE: &'static str = "
fq -- yasashii file uploader client.

Usage:
  fq (a | u | add | upload) [--config=<cfg>] [-n | --nocopy] <file>...
  fq (-h | --help)
  fq --version

Options:
  -h --help       Show this screen.
  --version       Show version.
  -nc --nocopy    Do not copy to clipboard after uploading.
  --config=<cfg>  Config path [default: config.toml].
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_file: Vec<String>,
    cmd_a: bool,
    cmd_add: bool,
    cmd_u: bool,
    cmd_upload: bool,
    flag_config: String,
    flag_n: bool,
    flag_nocopy: bool,
    flag_version: bool,
}

// TODO: mask password?
#[derive(Debug, Deserialize)]
struct Auth {
    endpoint: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    service: Auth,
}

fn main() -> Result<(), std::io::Error> {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    // TODO: glob, multiple files
    let config_text = fs::read_to_string(&args.flag_config)
        .expect(&format!("failed to load config {:?}", &args.flag_config));
    let config: Config = toml::from_str(&config_text).expect("failed to read config");

    if args.cmd_a || args.cmd_add || args.cmd_u || args.cmd_upload {
        upload(&args, &config);
    } else if args.flag_version {
        version();
    }

    Ok(())
}

fn version() {
    println!("{}", VERSION);
}

fn upload(args: &Args, config: &Config) {
    let client = reqwest::Client::new();
    let mut urls: Vec<String> = Vec::new();

    for ref pathstr in &args.arg_file {
        let path = Path::new(&pathstr);
        if !path.is_file() {
            panic!("{:?} is not a file", path);
        }

        // TODO: Check for filesize limit
        // TODO: Handle directory upload (zip?)
        let form = reqwest::multipart::Form::new()
            .text("password", config.service.password.clone())
            .file("file", &path)
            .expect("could not create form");

        let res = client
            .post(&config.service.endpoint)
            .multipart(form)
            .send()
            .expect("failed to send");

        if res.status() == 200 {
            urls.push(res.url().to_string());
        } else {
            panic!("failed to send: {:?}", res);
        }
    }

    println!("{}", urls.join("\r\n"));

    if !args.flag_nocopy && !args.flag_n {
        let _ = ClipboardProvider::new()
            .and_then(|mut c: ClipboardContext| c.set_contents(urls.join(" ")));
    }
}
