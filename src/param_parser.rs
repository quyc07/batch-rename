use std::collections::HashSet;
use std::env::Args;

pub struct ParamParser;

impl ParamParser {
    pub fn parse_args(args: Args) -> BrArgs {
        let args: Vec<String> = args.collect();
        let mut br_args = BrArgs {
            ..Default::default()
        };
        let mut options_exist = HashSet::new();
        for i in 1..args.len() {
            if args[i] == "--path" {
                check_option_exist(&mut options_exist, &args[i]);
                let mut path = args[i + 1].clone();
                if !path.ends_with("/") {
                    path.push_str("/");
                    br_args.path = Some(path);
                }
            } else if args[i] == "--prefix" {
                check_option_exist(&mut options_exist, &args[i]);
                br_args.prefix = Some(args[i + 1].clone());
            } else if args[i] == "--suffix" {
                check_option_exist(&mut options_exist, &args[i]);
                br_args.suffix = Some(args[i + 1].clone());
            } else if args[i] == "--replace" {
                check_option_exist(&mut options_exist, &args[i]);
                br_args.old = Some(args[i + 1].clone());
                br_args.new = Some(args[i + 2].clone());
                br_args.need_replace = true;
            } else if args[i] == "--new" {
                check_option_exist(&mut options_exist, &args[i]);
                br_args.new_name = Some(args[i + 1].clone());
                br_args.use_new_name = true;
            } else if args[i] == "--help" {
                br_args.show_help = true;
            } else if args[i] == "--version" {
                br_args.version = include_str!("version.txt").to_string();
                br_args.show_version = true;
            }
        }
        br_args
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

#[derive(Default, Debug)]
pub struct BrArgs {
    pub path: Option<String>,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
    pub old: Option<String>,
    pub new: Option<String>,
    pub new_name: Option<String>,
    pub use_new_name: bool,
    pub show_help: bool,
    pub show_version: bool,
    pub need_replace: bool,
    pub version: String,
}
