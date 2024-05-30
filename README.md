# Rerun

Tool for running command on file change. Language agnostic.

## Build

To build binary run `cargo build --release` and then make binary available in any PATH `sudo cp target/release/rerun /usr/local/bin/`.

## Usage

Just run rerun with command you want to launch as parameter(s):

```shell
rerun cargo run
```

Command can also be in quotes:

```shell
rerun "go test -v ./..."
```