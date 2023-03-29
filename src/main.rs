use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let prefix = &args[2];
    let files = fs::read_dir(path).unwrap();

    for file in files {
        let file_name = file.unwrap().file_name();
        let new_file_name = format!("{}{}", prefix, file_name.to_str().unwrap());
        fs::rename(format!("{}{}", path, file_name.to_str().unwrap()), format!("{}{}", path, new_file_name)).unwrap();
    }
}
