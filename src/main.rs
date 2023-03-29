use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut path = ".";
    let mut prefix = "";
    let mut new_name = "";
    let mut use_new_name = false;
    let mut show_help = false;

    for i in 1..args.len() {
        if args[i] == "-path" {
            path = &args[i+1];
        } else if args[i] == "-prefix" {
            prefix = &args[i+1];
        } else if args[i] == "-new" {
            new_name = &args[i+1];
            use_new_name = true;
        } else if args[i] == "-help" {
            show_help = true;
        }
    }

    if show_help {
        println!("Usage: ./path/to/program [-path <path>] [-prefix <prefix>] [-new <new_name>] [-help]");
        println!("-path: the path to the directory containing the files to be renamed (default: current directory)");
        println!("-prefix: the prefix to be added to the beginning of each file name (default: none)");
        println!("-new: the new name to be used for all files (default: none)");
        println!("-help: show this help message");
        return;
    }

    let files = fs::read_dir(path).unwrap();
    let mut count = 1;

    for file in files {
        let file_name = file.unwrap().file_name();
        let new_file_name = if use_new_name {
            format!("{}{}.{}", new_name, count, file_name.to_str().unwrap().split('.').last().unwrap())
        } else {
            format!("{}{}", prefix, file_name.to_str().unwrap())
        };
        fs::rename(format!("{}/{}", path, file_name.to_str().unwrap()), format!("{}/{}", path, new_file_name)).unwrap();
        count += 1;
    }
}
