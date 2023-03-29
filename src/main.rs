use std::env;
use std::fs;
use std::collections::HashMap;

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

    if path == "." {
        println!("The -path parameter is required.");
        return;
    }

    if show_help {
        println!("Usage: ./path/to/program [-path <path>] [-prefix <prefix>] [-new <new_name>] [-help]");
        println!("-path: the path to the directory containing the files to be renamed (default: current directory)");
        println!("-prefix: the prefix to be added to the beginning of each file name (default: none)");
        println!("-new: the new name to be used for all files (default: none)");
        println!("-help: show this help message");
        return;
    }
    // FIXME 第一次排序后，修改了第最后一个文件，再排序会删除前面的文件
    let mut file_map = HashMap::new();
    let files = fs::read_dir(path).unwrap();
    for file in files {
        let file_name = file.unwrap().file_name();
        let metadata = fs::metadata(format!("{}/{}", path, file_name.to_str().unwrap())).unwrap();
        let modified_time = metadata.modified().unwrap();
        file_map.insert(file_name.to_str().unwrap().to_string(), modified_time);
    }
    let mut sorted_files: Vec<_> = file_map.iter().collect();
    sorted_files.sort_by(|a, b| b.1.cmp(a.1));
    let mut count = 1;
    for (file_name, _) in sorted_files {
        let new_file_name = if use_new_name {
            format!("{}{}.{}", new_name, count, file_name.split('.').last().unwrap())
        } else {
            format!("{}{}", prefix, file_name)
        };
        fs::rename(format!("{}/{}", path, file_name), format!("{}/{}", path, new_file_name)).unwrap();
        count += 1;
    }
}
