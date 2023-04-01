use std::env;
use std::fs;

use crate::param_parser::ParamParser;

mod param_parser;

fn main() {
    let br_args = ParamParser::parse_args(env::args().collect());

    if br_args.show_help {
        println!("Usage: br [OPTIONS]\n");
        println!("Options:");
        println!(
            "  --path <path>         The path to the directory containing the files to rename."
        );
        println!("  --prefix <prefix>     The prefix to add to the file names.");
        println!("  --suffix <postfix>    The suffix to add to the file names.");
        println!(
            "  --replace <old> <new> Replace the old string in the file names with the new string."
        );
        println!("  --new <name>          Use a new name for the files, with a number appended to each file name.");
        println!("  --help                Print this help message.");
        println!("  --version             Print the version of the program.");
        return;
    }

    if br_args.show_version {
        println!("version: {}", br_args.version);
        return;
    }

    if br_args.path == None {
        println!("The --path parameter is required.");
        return;
    }

    let files = fs::read_dir(&br_args.path.as_ref().unwrap()).unwrap();
    let mut count = 1;
    for file in files {
        let file_name = file.unwrap().file_name().into_string().unwrap();
        let new_file_name = if br_args.use_new_name {
            format!(
                "{}{}.{}",
                br_args.new_name.as_ref().unwrap(),
                count,
                file_name.split('.').last().unwrap()
            )
        } else if br_args.need_replace {
            let file_name = file_name.replace(
                br_args.old.as_ref().unwrap().as_str(),
                br_args.new.as_ref().unwrap().as_str(),
            );
            let x: Vec<&str> = file_name.split(".").collect();
            format!(
                "{}{}{}.{}",
                br_args.prefix.as_ref().unwrap_or(&String::from("")),
                x[0],
                br_args.suffix.as_ref().unwrap_or(&String::from("")),
                x[1]
            )
        } else {
            let x: Vec<&str> = file_name.split(".").collect();
            format!(
                "{}{}{}.{}",
                br_args.prefix.as_ref().unwrap_or(&String::from("")),
                x[0],
                br_args.suffix.as_ref().unwrap_or(&String::from("")),
                x[1]
            )
        };
        fs::rename(
            format!("{}/{}", br_args.path.as_ref().unwrap(), file_name),
            format!("{}/{}", br_args.path.as_ref().unwrap(), new_file_name),
        )
        .unwrap();
        count += 1;
    }
}
