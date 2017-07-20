use std::env::{args,var};
use std::io::prelude::*;
use std::fs::File;

mod vm;
mod parser;
mod utils;

const VERSION: &str = "0.1.0";
fn main() {
    let arg = parse_args(args().collect());
    //println!("{:?}",arg.type_class1);
    match arg.type_class1 {
        ArgumentType::PrintHelp => {
            let binary_path: Vec<String> = args().collect();
            let binary_path: &str = binary_path[0].as_str();
            println!(
                "MonkeyVM (Version {})
a tool for running coding's monkey-lang code
Usage:
{} help to print help
{} version to print version
<export PARGS=args>&&{} run [file]  to run a program
get source code on coding.net",
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
            let pargs : Vec<i32> = parsepargs();
            vm::execute_program(program_text.as_str(), pargs);
        }
    }
}
fn parsepargs() -> Vec<i32> {
    match var("PARGS") {
        Ok(a) => {
            let mut pargs = Vec::<i32>::new();
            let pargs_txt = a.trim().split(',');
            for i in pargs_txt {
                let tmp :i32 = i.parse::<i32>().unwrap();
                pargs.push(tmp);
            }
            pargs
        },
        Err(e) => Vec::<i32>::new(),
    }
}
/*
fn parsepargs() -> Vec<i32> {
    let mut ret = Vec::<i32>::new();
    let mut l = 0;
    let i : &str;
    for i in args().collect() {
        if l > 3 {
            let tmp : i32 = i.parse() ;
            ret.push(tmp);
        }
        l+=1;
    }
    ret
}
*/
//useless tests,but ... >_>
#[cfg(test)]
mod tests {
    use main;
    use parse_args;
    use ArgumentType;
    #[test]
    #[should_panic]
    fn test_argparser_panic_arglen() {
        let test_argument = "help".to_string();
        parse_args(vec![test_argument]);
    }
    #[test]
    fn test_argparser_parses_help() {
        match parse_args(vec!["mvm".to_string(), "help".to_string()]).type_class1 {
            ArgumentType::PrintHelp => {}
            ArgumentType::PrintVersion => panic!("argparser mistake help as version"),
            ArgumentType::ExecuteProgram => panic!("argparser mistake help as run"),
        }
    }
    #[test]
    fn test_argparser_help_on_inputwrong() {
        match parse_args(vec!["mvm".to_string(), "foo".to_string()]).type_class1 {
            ArgumentType::PrintHelp => {}
            _ => panic!("argparser doesn't print help when input wrong"),
        }
    }
    #[test]
    #[should_panic]
    fn test_argparser_parses_run() {
        parse_args(vec!["mvm".to_string(), "run".to_string()]);
    }
}
fn parse_args(args: Vec<String>) -> Argument {
    if args.len() >= 2 {
    } else {
        panic!(
            "Error: wrong of number argument({} given,1+ expected),use '{} help'to print help",
            args.len(),
            args[0]
        );
    }
    match args[1].as_str() {
        "help" => Argument::new(ArgumentType::PrintHelp),
        "version" => Argument::new(ArgumentType::PrintVersion),
        "run" => {
            if args.len() < 3 {
                panic!("Error: please give a file to run");
            }
            if let Ok(f) = File::open(&args[2]) {
                let mut ret = Argument::new(ArgumentType::ExecuteProgram);
                ret.put_file(f);
                ret
            } else {
                println!("Error: can't open file");
                unreachable!();
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
    fn get_file(&self) -> &File {
        if let Some(ref f) = self.file {
            f
        } else {
            panic!("Failed to unpack Option<File>");
        }
    }
}

#[derive(Debug)]
enum ArgumentType {
    PrintVersion,
    PrintHelp,
    ExecuteProgram,
}
