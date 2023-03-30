use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut path = String::new();
    let mut prefix = "";
    let mut postfix = "";
    let mut replace = "";
    let mut replace_to = "";
    let mut new_name = "";
    let mut use_new_name = false;
    let mut show_help = false;
    let mut need_replace = false;

    let mut options_exist = HashSet::new();

    for i in 1..args.len() {
        if args[i] == "--path" {
            check_option_exist(&mut options_exist, &args[i]);
            path = args[i + 1].clone();
            if !path.ends_with("/") {
                path.push_str("/");
            }
        } else if args[i] == "--prefix" {
            check_option_exist(&mut options_exist, &args[i]);
            prefix = &args[i + 1];
        } else if args[i] == "--postfix" {
            check_option_exist(&mut options_exist, &args[i]);
            postfix = &args[i + 1];
        } else if args[i] == "--replace" {
            check_option_exist(&mut options_exist, &args[i]);
            replace = &args[i + 1];
            replace_to = &args[i + 2];
            need_replace = true;
        } else if args[i] == "--new" {
            check_option_exist(&mut options_exist, &args[i]);
            new_name = &args[i + 1];
            use_new_name = true;
        } else if args[i] == "--help" {
            show_help = true;
        }
    }

    if path == "" && !show_help {
        println!("The --path parameter is required.");
        return;
    }

    if show_help {
        println!("Usage: batch_rename [OPTIONS]\n");
        println!("Options:");
        println!(
            "  --path <path>         The path to the directory containing the files to rename."
        );
        println!("  --prefix <prefix>     The prefix to add to the file names.");
        println!("  --postfix <postfix>   The postfix to add to the file names.");
        println!(
            "  --replace <old> <new> Replace the old string in the file names with the new string."
        );
        println!("  --new <name>          Use a new name for the files, with a number appended to each file name.");
        println!("  --help                Print this help message.");
        return;
    }

    let files = fs::read_dir(&path).unwrap();
    let mut count = 1;
    for file in files {
        let file_name = file.unwrap().file_name().into_string().unwrap();
        let new_file_name = if use_new_name {
            format!(
                "{}{}.{}",
                new_name,
                count,
                file_name.split('.').last().unwrap()
            )
        } else if need_replace {
            let file_name = file_name.replace(replace, replace_to);
            let x: Vec<&str> = file_name.split(".").collect();
            format!("{}{}{}.{}", prefix, x[0], postfix, x[1])
        } else {
            let x: Vec<&str> = file_name.split(".").collect();
            format!("{}{}{}.{}", prefix, x[0], postfix, x[1])
        };
        fs::rename(
            format!("{}/{}", path, file_name),
            format!("{}/{}", path, new_file_name),
        )
        .unwrap();
        count += 1;
    }
}

fn check_option_exist<'a>(options_exist: &mut HashSet<&'a String>, option: &'a String) {
    if !options_exist.insert(option) {
        panic!(
            "Option {} has been set more than ones, please check the options",
            option
        );
    }
}
