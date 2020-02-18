#[derive(serde::Deserialize)]
struct Config {
    bin: String,
    args: Option<Vec<String>>,
    rust_args_as_one: Option<bool>,
    rust_args: Option<bool>,
    debug: Option<bool>,
    env: Option<std::collections::HashMap<String, String>>,
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut rust_args = std::env::args().collect::<Vec<_>>();
    let selfbin = rust_args.remove(0);
    let winapi_args = {
        let mut args = winapi_args();
        args.remove(0);
        args
    };

    let config: Config = {
        let config_path = format!("{}.conf", selfbin);
        let config_contents = std::fs::read_to_string(config_path)?;
        toml::from_str(config_contents.as_ref())?
    };

    let config_args = config.args.map_or(Vec::<String>::new(), |it| it);
    let config_args = config_args.iter().map(|it| it.as_str());

    let cli_args = match config.rust_args
        .map_or(false, |it| it) {
            true => match config.rust_args_as_one.map_or(false, |it| it) {
                false => Box::new(rust_args.into_iter()) as Box<dyn Iterator<Item = String>>,
                true => Box::new(std::iter::once(rust_args.join(" "))) as Box<dyn Iterator<Item = String>>,
            },
            false => Box::new(winapi_args.into_iter()),
        }
        .collect::<Vec<_>>();
    let cli_args = cli_args.iter().map(|it| it.as_str());

    let combined_args = itertools::chain(config_args, cli_args).collect::<Vec<_>>();
    if config.debug.map_or(false, |it| it) {
        eprintln!("Args of the child process are {:?}", combined_args);
    }

    let mut command = std::process::Command::new(config.bin);
    config.env.map_or(std::collections::HashMap::new(), |it| it).iter().for_each(|(k, v)| {
        command.env(k, v);
    });

    let mut child = command
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

fn winapi_args() -> Vec<String> {
    use std::os::windows::ffi::OsStringExt;

    const QUOTE: u16 = '"' as u16;
    const SPACE: u16 = ' ' as u16;

    let lp_cmd_line = unsafe { winapi::um::processenv::GetCommandLineW() as *const u16 };

    let mut ret_val = Vec::<String>::new();
    /*if lp_cmd_line.is_null() || *lp_cmd_line == 0 {
        ret_val.push(exe_name());
        return ret_val;
    }*/
    let mut cmd_line = unsafe {
        let mut end = 0;
        while *lp_cmd_line.offset(end) != 0 {
            end += 1;
        }
        std::slice::from_raw_parts(lp_cmd_line, end as usize)
    };
    cmd_line = match cmd_line[0] {
        QUOTE => {
            let args = {
                let mut cut = cmd_line[1..].splitn(2, |&c| c == QUOTE);
                if let Some(exe) = cut.next() {
                    ret_val.push(std::ffi::OsString::from_wide(exe).into_string().unwrap());
                }
                cut.next()
            };
            if let Some(args) = args {
                args
            } else {
                return ret_val;
            }
        }
        0..=SPACE => {
            ret_val.push(String::new());
            &cmd_line[1..]
        }
        _ => {
            let args = {
                let mut cut = cmd_line.splitn(2, |&c| c > 0 && c <= SPACE);
                if let Some(exe) = cut.next() {
                    ret_val.push(std::ffi::OsString::from_wide(exe).into_string().unwrap());
                }
                cut.next()
            };
            if let Some(args) = args {
                args
            } else {
                return ret_val;
            }
        }
    };

    ret_val.push(std::ffi::OsString::from_wide(cmd_line).into_string().unwrap());
    ret_val
}
