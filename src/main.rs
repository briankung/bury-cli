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
            create_dir(args.last().unwrap());
            execute_cp(cp_args);
        }
    };
}

fn create_dir(path: &OsString) {
    let mut path = PathBuf::from(path);
    if path.extension().is_some() {
        path = path.parent().unwrap().into();
    }

    let dir = std::fs::create_dir_all(path.clone());
    if dir.is_err() {
        eprint!("Directory `{:?}` could not be created", path);
    }
}

fn execute_cp(cp_args: Vec<&OsString>) {
    let mut cmd = Command::new("cp");
    cmd.args(cp_args);
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
