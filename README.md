# What it does

Runs an arbitrary child process and waits for its termination, while delegating the standard input, output and error to the child.

# How to build / install

1. Install Rust (rust-lang.org)
2. Clone this repo
3. Run `cargo install --path . --force`
   Or, just to build run `cargo build --release` and look in `target/release/`.

# How to use

The wrapper will look for a config file at self path plus `.conf` at the end. For example, the wrapper is at `C:\Users\ivan\bin\ashcmd.exe`.  The config file will be looked for at `C:\Users\ivan\bin\ashcmd.exe.conf`.

Full config contents with examples:
```toml
bin = 'C:\Users\ivan\bin\busybox.exe'
args = ["ash", "-c"]
cli_args_as_one = true
debug = false
```

Everything but `bin` is optional. `cli_args_as_one = true` will concatenate all `exewrapper`'s arguments with a space and provide it as a single argument to the child process. For example, with the config above, running `ashcmd.exe echo 'hello world'` will run

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

Now, if you're using FAR Manager, you could

1. Type `far:config` and ENTER
2. Set `System.Executor.Comspec` to `C:\Users\ivan\bin\ashcmd.exe`
3. Set `System.Executor.ComspecArguments` to `{0}`

Or do something else with it. I don't know.
