#[derive(serde_derive::Deserialize)]
struct Config {
    bin: String,
    args: Option<Vec<String>>,
    cli_args_as_one: Option<bool>,
    debug: Option<bool>,
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut cli_args = std::env::args().collect::<Vec<_>>();

    let selfbin = cli_args.remove(0);

    let config: Config = {
        let config_path = format!("{}.conf", selfbin);
        let config_contents = std::fs::read_to_string(config_path)?;
        toml::from_str(config_contents.as_ref())?
    };

    let config_args = config.args.map_or(Vec::<String>::new(), |it| it);
    let config_args = config_args.iter().map(|it| it.as_str());

    let cli_args: Box<dyn Iterator<Item = String>> = match config.cli_args_as_one.map_or(false, |it| it) {
        false => Box::new(cli_args.into_iter()),
        true => Box::new(std::iter::once(cli_args.join(" "))),
    };
    let cli_args = cli_args.collect::<Vec<_>>();
    let cli_args = cli_args.iter().map(|it| it.as_str());

    let combined_args = itertools::chain(config_args, cli_args).collect::<Vec<_>>();
    if config.debug.map_or(false, |it| it) {
        eprintln!("Args of the child process are {:?}", combined_args);
    }

    let mut child = std::process::Command::new(config.bin)
        .args(&combined_args)
        .spawn()?;

    loop {
        match child.try_wait()? {
            Some(status) => std::process::exit(match status.code() {
                Some(code) => code,
                None => {
                    eprintln!("The child process is finished, but there's no exit code -- probably got killed");
                    255
                },
            }),
            None => std::thread::sleep(std::time::Duration::from_millis(50)),
        }
    }
}
