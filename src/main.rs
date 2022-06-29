use std::ffi::OsString;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let args: Vec<OsString> = std::env::args_os().collect();

    match args.as_slice() {
        &[] => {
            print!("This should be unreachable")
        }
        [script_name] => {
            if arg_is_path_to_bin(script_name) {
                println!(r"TODO: help message");
            } else {
                eprint!("Please supply at least two arguments");
                std::process::exit(1);
            }
        }
        _ => {
            let cp_args = get_cp_args(&args);
            create_dir(&args);
            execute_cp(cp_args);
        }
    };
}

fn create_dir(args: &[OsString]) {
    let last = args.last().unwrap();
    let path = PathBuf::from(last);
    let parent = path.parent();
    if let Some(parent_path) = parent {
        let dir = std::fs::create_dir_all(parent_path);
        if dir.is_err() {
            eprint!("Directory `{:?}` could not be created", parent_path);
        }
    }
}

fn execute_cp(cp_args: Vec<&OsString>) {
    let mut cmd = Command::new("cp");
    for arg in cp_args.iter() {
        cmd.arg(arg);
    }
    cmd.spawn().expect("cp command failed");
}

fn get_cp_args(args: &[OsString]) -> Vec<&OsString> {
    let mut cp_args = Vec::new();
    let mut args = args.iter();

    let first = args.next().unwrap();

    if !arg_is_path_to_bin(first) {
        cp_args.push(first);
    };

    for arg in args {
        cp_args.push(arg);
    }

    cp_args
}

fn arg_is_path_to_bin(arg: &OsString) -> bool {
    arg.to_string_lossy().ends_with(env!("CARGO_PKG_NAME"))
}
