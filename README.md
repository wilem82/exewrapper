# What it does

Runs an arbitrary child process and waits for its termination, while delegating the standard input, output and error to the child.

# How to build / install

1. Install Rust (rust-lang.org)
2. Clone this repo
3. Run `cargo install --path . --force`
  - Or, just to build run `cargo build --release` and look in `target/release/`.
  - Or, build via `build.bat`, but you'll need to have `xargo` installed (see https://github.com/japaric/xargo). This variant provides the smallest binary size (277k as opposed to ~370k).

# How to use

The wrapper will look for a config file at self path plus `.conf` at the end. For example, the wrapper is at `C:\Users\ivan\bin\ashcmd.exe`.  The config file will be looked for at `C:\Users\ivan\bin\ashcmd.exe.conf`.

Full config contents with examples:
```toml
bin = 'C:\Users\ivan\bin\busybox.exe'
args = ["ash", "-c"]
rust_args = false
rust_args_as_one = false
debug = false

[env]
HOME = 'C:\Users\ivan'
```

Everything but `bin` is optional.

- `rust_args_as_one = true` will concatenate all `exewrapper`'s arguments with a space and provide it as a single argument to the child process. For example, with the config above, running `ashcmd.exe echo 'hello world'` will run

  - `C:\Users\ivan\bin\busybox.exe`
  - `ash`
  - `-c`
  - `echo 'hello world'`

  With `cli_args_as_one = false` it would've been

  - ...
  - `-c`
  - `echo`
  - `'hello`
  - `world'`

- `rust_args = false` disables Rust's stdlib handling of the command line on Windows and bypasses the command line as is, as a single argument. This leaves the command line intact, whereas the Rust's stdlib processing removes double quotes and does more stuff.

- `[env]` section allows setting / overriding environment variables for the child process.
