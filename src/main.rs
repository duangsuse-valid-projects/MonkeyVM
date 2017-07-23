#![feature(unicode)]
#![feature(test)]

use std::env::{args, var};
use std::io::prelude::*;
use std::fs::File;
use std::process::exit;

mod vm;
mod parser;
mod utils;

const VERSION: &str = "0.2.0";

fn main() {
    let commandline = args().collect();
    let debug = var("PDBG").unwrap_or_default() == "1";
    let verbose = var("PVBS").unwrap_or_default() == "1";
    if debug {
        println!(
            "Monkey VM v{} started in debug mode with commandline argument {:?}",
            VERSION,
            commandline
        );
    }
    let arg = parse_args(commandline);
    if debug {
        println!("parsed argument: {:?}", arg);
    }
    match arg.type_class1 {
        ArgumentType::PrintHelp => {
            let binary_path: String = args().next().unwrap_or(String::from("mvm"));
            println!(
                "MonkeyVM v{}
a tool for running coding's monkey-lang code
Usage:
{} help|h to print help
{} version|v to print version
{} run|r [file]  to execute a program

Environment variables:
PARGS -> ',' splited hprog arguments
PDBG=1 -> Debug mode (higest verbose level)
PVBS=1 -> Verbose (output when memory change,etc.)
get source code at https://coding.net",
                VERSION,
                binary_path,
                binary_path,
                binary_path
            );
        }
        ArgumentType::PrintVersion => println!("{}", VERSION),
        ArgumentType::ExecuteProgram => {
            let mut program_text: String = String::new();
            arg.get_file().read_to_string(&mut program_text).unwrap();
            let pargs: Vec<i32> = parsepargs();
            vm::execute_program(program_text.as_str(), pargs, verbose, debug);
        }
    }
}
fn parsepargs() -> Vec<i32> {
    match var("PARGS") {
        Ok(a) => {
            let arg_text_trimed = a.trim();
            if arg_text_trimed == "" {
                println!("please unset blank varaible PARGS first.");
                exit(1);
            }
            let mut pargs = Vec::<i32>::new();
            let pargs_txt = arg_text_trimed.split(",");
            for i in pargs_txt {
                let tmp: i32 = i.parse::<i32>().unwrap();
                pargs.push(tmp);
            }
            pargs
        }
        Err(_) => Vec::<i32>::new(),
    }
}

#[cfg(test)]
mod tests {
    use parse_args;
    use ArgumentType;
    #[test]
    fn command_parser_parses_help() {
        match parse_args(vec![String::from("mvm"), String::from("help")]).type_class1 {
            ArgumentType::PrintHelp => {}
            _ => panic!(),
        }
    }
    #[test]
    fn command_parser_parses_version() {
        match parse_args(vec![String::from("mvm"), String::from("version")]).type_class1 {
            ArgumentType::PrintVersion => {}
            _ => panic!(),
        }
    }
}
fn parse_args(args: Vec<String>) -> Argument {
    if args.len() >= 2 {
    } else {
        println!(
            "WARN: wrong of number argument({} given,1+ expected),use '{} help'to print help",
            args.len(),
            args[0]
        );
        exit(1);

    }
    match args[1].as_str() {
        "help" | "h" => Argument::new(ArgumentType::PrintHelp),
        "version" | "v" => Argument::new(ArgumentType::PrintVersion),
        "run" | "r" => {
            if args.len() < 3 {
                println!("Error: please give a file to run");
                exit(1);
            }
            if let Ok(f) = File::open(&args[2]) {
                let mut ret = Argument::new(ArgumentType::ExecuteProgram);
                ret.put_file(f);
                ret
            } else {
                println!("Error: can't open file '{}'", args[2]);
                exit(1);
            }
        }
        _ => Argument::new(ArgumentType::PrintHelp),
    }
}

#[derive(Debug)]
struct Argument {
    type_class1: ArgumentType,
    file: Option<File>,
}
impl Argument {
    fn new(argtype: ArgumentType) -> Argument {
        Argument {
            type_class1: argtype,
            file: None,
        }
    }
    fn put_file(&mut self, file: File) {
        self.file = Some(file);
    }
    fn get_file(self) -> File {
        self.file.unwrap()
    }
}

#[derive(Debug)]
enum ArgumentType {
    PrintVersion,
    PrintHelp,
    ExecuteProgram,
}
