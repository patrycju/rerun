use notify::RecursiveMode;
use notify_debouncer_full::{new_debouncer, notify::*};
use std::env;
use std::process::Command;
use std::{path::Path, time::Duration};

fn run_command(command: &str) {
    let exit_status = Command::new("sh")
        .arg("-c")
        .arg(command)
        .spawn()
        .unwrap()
        .wait()
        .expect("failed to execute process");
    println!("({}) ===========================================================",
             exit_status.code().expect("unable to decode exit code")
    )
}

fn watch<P: AsRef<Path>>(path: P, command: &str) {
    let (tx, rx) = std::sync::mpsc::channel();
    let mut debouncer = new_debouncer(Duration::from_secs(1), None, tx).unwrap();

    debouncer
        .watcher()
        .watch(path.as_ref(), RecursiveMode::Recursive)
        .unwrap();

    for result in rx {
        match result {
            Ok(_events) => run_command(command),
            Err(_error) => eprintln!("watch error"),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: rerun <command>");
        std::process::exit(1);
    }
    let command = args[1..].join(" ");

    println!("Rerunning {}", command);
    run_command(&command);
    watch(".", &command)
}
