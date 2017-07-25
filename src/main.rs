#![feature(unicode)]
#![feature(test)]

use std::env::{args, var};
use std::io::prelude::*;
use std::fs::File;
use std::process::exit;
use std::time::SystemTime;
use parser::MonkeyAST;
use parser::{zip_numeric, unzip_numeric};

mod vm;
mod parser;
mod utils;

const VERSION: &str = "0.2.5";

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
a tool for running coding monkey-lang code
Usage:
{} help|h print help
{} version|v print version
{} licence|l view licence
{} run|r [file] execute a program
{} beautify|b [file] beautify a program
{} parse|p [file] parse a program
{} compile|c [file] to compile a file

Note:
it's best to use Gzip to compress compile output 

Environment variables:
PARGS -> ',' splited hprog arguments
PDBG=1 -> Debug mode (higest verbose level)
PVBS=1 -> Verbose (output when memory change,etc.)
get source code on https://coding.net",
                VERSION,
                binary_path,
                binary_path,
                binary_path,
                binary_path,
                binary_path,
                binary_path,
                binary_path
            );
        }
        ArgumentType::PrintVersion => println!("{}", VERSION),
        ArgumentType::PrintLicence => {
            println!(
                "MonkeyVM 是自由软件；您可以在自由软件基金会发布的 GNU 通用公共许可证下重新发布或修改它；许可证应使用第三版本或您所选择的更新的版本。
发布 MonkeyVM 的目的是希望它能够在一定程度上帮到您。但我们并不为它提供任何形式的担保，也无法保证它可以在特定用途中得到您希望的结果。请参看 GNU GPL 许可中的更多细节。
您应该在收到 MonkeyVM 的同时收到了 GNU 通用公共许可证的副本；如果您没有收到，请查看 <http://www.gnu.org/licenses/>。"
            )
        }
        ArgumentType::ExecuteProgram => {
            let mut program_text: String = String::new();
            arg.get_file().read_to_string(&mut program_text).unwrap();
            if program_text.starts_with("H2C") {
                println!("Compiled chunk header found,unzipping AST...");
                let mut file: Vec<u8> = vec![];
                for byte in program_text.bytes() {
                    file.push(byte);
                }
                let time_start = SystemTime::now();
                let monkey_ast = unzip_ast(file);
                let time_cost = SystemTime::now().duration_since(time_start);
                println!("AST unzipped in {:?}", time_cost);
                let r = vm::do_emulate(monkey_ast, parsepargs(), verbose, debug);
                println!("{}", r);
            } else {
                let pargs: Vec<i32> = parsepargs();
                vm::execute_program(program_text.as_str(), pargs, verbose, debug);
            }
        }
        ArgumentType::ParseProgram => {
            use parser::parse_program;
            let mut program_text: String = String::new();
            if program_text.starts_with("H2C") {
                println!("Err: cannot parse compiled chunk directly now.(Sorry)");
                exit(233);
            }
            let mut file = arg.get_file();
            file.read_to_string(&mut program_text).unwrap();
            let parsed = parse_program(program_text.as_str(), verbose, debug);
            println!("ln--CMD--DAT--");
            let end = parsed.CMD.len();
            for i in 0..end {
                println!("{}--{:?}--{:?}--", i, parsed.CMD[i], parsed.DAT[i]);
            }
            println!(".............");
            println!("--TAG--LOC--");
            parsed.Tags.print_fmt();
        }
        ArgumentType::BeautifyProgram => {
            use parser::parse_program;
            let mut buf = Vec::<u8>::new();
            arg.get_file().read_to_end(&mut buf).unwrap();
            let parsed;
            if buf.len() < 3 {
                println!("fatal: invalid file size");
                exit(1);
            }
            if buf[0] == b"H"[0] && buf[1] == b"2"[0] && buf[2] == b"C"[0] {
                if debug {
                    println!("magic: {} {} {}", buf[0], buf[1], buf[2]);
                }
                println!("//Unzipping binary AST...");
                parsed = unzip_ast(buf);
            } else {
                let tmp = String::from_utf8(buf);
                if let Ok(s) = tmp {
                    parsed = parse_program(s.as_str(), verbose, debug);
                } else {
                    println!("fatal: cannot read file to string.please use utf-8 coding.");
                    exit(1);
                }
            }
            println!("//beautified with MonkeyVM v{}", VERSION);
            let end = parsed.CMD.len();
            for i in 0..end {
                parsed.Tags.locate_print_reverse(i);
                println!("{}{}", parsed.CMD[i].to_str(), parsed.DAT[i].to_str());
            }
            parsed.Tags.locate_print_tail_tag(parsed.CMD.len() - 1);
        }
        ArgumentType::CompileSource => {
            use parser::parse_program;
            use std::io::prelude::*;
            use utils::res::HDataTypes;
            let mut program_text: String = String::new();
            let mut cmdlen = 0usize;
            arg.get_file().read_to_string(&mut program_text).unwrap();
            if program_text.starts_with("H2C") {
                println!("fatal: cannot compile compiled source.");
                exit(1);
            }
            let parsed = parse_program(program_text.as_str(), verbose, debug);
            if verbose {
                println!("MonkeyVM v{} is in compile mode.", VERSION);
            }
            if let Ok(mut f) = File::create("a.mkc") {
                if debug {
                    println!("writing magic to file..");
                }
                f.write(b"H2C").unwrap(); //write binary magic
                if debug {
                    println!(
                        "writing length:{:?} to file...",
                        zip_numeric(parsed.CMD.len() as u32)
                    );
                }
                f.write(&zip_numeric(parsed.CMD.len() as u32)).unwrap(); //write section length
                for c in parsed.CMD {
                    if debug {
                        println!("writing {:?} to file...", c.to_u8());
                    }
                    f.write(&[c.to_u8()]).unwrap();
                    cmdlen += 1;
                }
                for d in parsed.DAT {
                    match d {
                        HDataTypes::NumLiteral(n) => {
                            if n >= 0 {
                                if debug {
                                    println!("writing positive numlt sigure:01 to file...");
                                }
                                f.write(&[01]).unwrap();
                            } else {
                                if debug {
                                    println!("writing positive numlt sigure:00 to file...");
                                }
                                f.write(&[00]).unwrap();
                            }
                            if debug {
                                println!("writing data:{:?} to file...", zip_numeric(n as u32));
                            }
                            f.write(&zip_numeric(n as u32)).unwrap();
                        }
                        HDataTypes::Pointer(n) => {
                            if debug {
                                println!("writing ptr sigure:10 to file...");
                            }
                            f.write(&[10]).unwrap();
                            if debug {
                                println!("writing ptr data:{:?} to file...", zip_numeric(n as u32));
                            }
                            f.write(&zip_numeric(n as u32)).unwrap();
                        }
                        HDataTypes::IndirectPointer(n) => {
                            if debug {
                                println!("writing iptr sigure:20 to file...");
                            }
                            f.write(&[20]).unwrap();
                            if debug {
                                println!("writing ptr data:{:?} to file...", zip_numeric(n as u32));
                            }
                            f.write(&zip_numeric(n as u32)).unwrap();
                        }
                        HDataTypes::Nil => {
                            if debug {
                                println!("writing Nil sigure:30 to file...");
                            }
                            f.write(&[30]).unwrap();
                            if debug {
                                println!("writing Nil data:00x32 to file...");
                            }
                            f.write(&[0; 32]).unwrap();
                        }
                    }
                }
                for t in &parsed.Tags.tags {
                    if t.get_id() >= 0 {
                        if debug {
                            println!("writing tag id positive sigure 1...");
                        }
                        f.write(&[1]).unwrap();
                    } else {
                        if debug {
                            println!("writing tag id negative sigure 0...");
                        }
                        f.write(&[0]).unwrap();
                    }
                    if debug {
                        println!("writing tag id data:{:?}", zip_numeric(t.get_id() as u32));
                    }
                    f.write(&zip_numeric(t.get_id() as u32)).unwrap();
                    if debug {
                        println!("writing tag lo data:{:?}", zip_numeric(t.get_lo() as u32));
                    }
                    f.write(&zip_numeric(t.get_lo())).unwrap();
                }
                for t in &parsed.Tags.locate_get_tail_tag(cmdlen) {
                    if t.get_id() >= 0 {
                        if debug {
                            println!("writing tag id positive sigure 1...");
                        }
                        f.write(&[1]).unwrap();
                    } else {
                        if debug {
                            println!("writing tag id negative sigure 0...");
                        }
                        f.write(&[0]).unwrap();
                    }
                    if debug {
                        println!("writing tag id data:{:?}", zip_numeric(t.get_id() as u32));
                    }
                    f.write(&zip_numeric(t.get_id() as u32)).unwrap();
                    if debug {
                        println!("writing tag lo data:{:?}", zip_numeric(t.get_lo() as u32));
                    }
                    f.write(&zip_numeric(t.get_lo())).unwrap();
                }
                println!("compile finished. use beautify tool to decompile.");
            } else {
                println!("ERR: failed to create a.mkc");
                exit(1);
            }
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

///#About MonkeyVM compiled chunk format
///`file structure`:
///Header: 48 32 43 (ASCII H2C)
///command-length: starts from byte #4,32-byte length
///command section: starts from byte #36,length=command-length section convert to u32
///data section: starts from the end of command section,length=command-length convert to u32 x33
///tag section: followed by data section,end= file end
///
///`data format`:
///Type ,32-byte data
///Type:
///01->numliteral
///00->numliteral,negative
///10->pointer
///20->indirectpointer
///30->nil
///`tag format`:
///Id Lo
///Id:33-byte value(i32)
///Lo:32-byte value(u32)
fn unzip_ast(file: Vec<u8>) -> MonkeyAST {
    use std::env::var;
    use utils::res::{HCommands, HDataTypes};
    use vm::Tag;
    let debug = {
        var("PDBG") == Ok(String::from("1"))
    };
    let mut ret = MonkeyAST::new(); //let's create an AST to return first.
    let mut length_tmp = 0;
    let mut numeric_parse_temp = [0u8; 32];
    for i in 0x3..0x23 {
        numeric_parse_temp[length_tmp] = file[i];
        length_tmp += 1;
    }
    if debug {
        println!("{:?}", numeric_parse_temp);
    }
    let command_length = unzip_numeric(numeric_parse_temp); //get length then
    let command_end = 34usize + command_length.unwrap() as usize;
    println!(
        "//Info: file length={} command length={} command ends at {} (expected)",
        file.len(),
        command_length.unwrap(),
        command_end
    );
    for i in 35..command_end + 1 {
        let command = match file[i] {
            1 => HCommands::ADD,
            2 => HCommands::AO,
            3 => HCommands::I,
            4 => HCommands::JMP,
            5 => HCommands::O,
            6 => HCommands::QNJ,
            7 => HCommands::QNU,
            8 => HCommands::QPJ,
            9 => HCommands::QZJ,
            10 => HCommands::RAD,
            11 => HCommands::RED,
            12 => HCommands::RSB,
            13 => HCommands::SUB,
            14 => HCommands::WRT,
            _ => {
                println!("fatal: invalid command id at {}:{}", i, file[i]);
                exit(1);
            }
        };
        if debug {
            println!(
                "binparser: file location {}:{} parsed command:{:?}",
                i,
                file[i],
                command
            );
        }
        ret.CMD.push(command);
    }
    let data_start = command_end + 1;
    let data_end = data_start + 33 * command_length.unwrap() as usize - 1;
    println!(
        "//calcuated data section start:{} end:{}",
        data_start,
        data_end
    );
    let mut current = data_start;
    let mut i = 0usize;
    //data section format: Type(1byte) data(32byte)
    loop {
        if i > 31 {
            i = 0;
        }
        if debug {
            println!("block {} with value {}", current, file[current]);
        }
        match file[current] {
            01 => {
                for m in file[current + 1..current + 33].iter() {
                    numeric_parse_temp[i] = *m;
                    i += 1;
                }
                ret.DAT.push(HDataTypes::NumLiteral(
                    unzip_numeric(numeric_parse_temp).unwrap() as i32,
                ));
                if debug {
                    println!("binparser: data type Numlt at {}", current);
                }
            }
            00 => {
                for m in file[current + 1..current + 33].iter() {
                    numeric_parse_temp[i] = *m;
                    i += 1;
                }
                ret.DAT.push(HDataTypes::NumLiteral(
                    -(unzip_numeric(numeric_parse_temp).unwrap() as i32),
                ));
                if debug {
                    println!("binparser: data type -Numlt at {}", current);
                }
            }
            10 => {
                for m in file[current + 1..current + 33].iter() {
                    numeric_parse_temp[i] = *m;
                    i += 1;
                }
                ret.DAT.push(HDataTypes::Pointer(
                    unzip_numeric(numeric_parse_temp).unwrap() as usize,
                ));
                if debug {
                    println!("binparser: data type ptr at {}", current);
                }
            }
            20 => {
                for m in file[current + 1..current + 33].iter() {
                    numeric_parse_temp[i] = *m;
                    i += 1;
                }
                ret.DAT.push(HDataTypes::IndirectPointer(
                    unzip_numeric(numeric_parse_temp).unwrap() as usize,
                ));
                if debug {
                    println!("binparser: data type Nil at {}", current);
                }
            }
            30 => {
                ret.DAT.push(HDataTypes::Nil);
                if debug {
                    println!("binparser: data type Nil at {}", current);
                }
            }
            _ => {
                println!("fatal: invalid data type at {}:{}", current, file[current]);
                exit(1);
            }
        }
        current += 33;
        if current > data_end {
            break;
        }
        if debug {
            println!(
                "data {:?} added {:?}",
                ret.DAT[ret.DAT.len() - 1],
                numeric_parse_temp
            );
        }
    }
    let tag_start = data_end + 1;
    let tag_end = file.len() - 1;
    if tag_start > tag_end {
        println!("//tag section not present,finishing parse");
    } else {
        let tags_num = (tag_end - tag_start) / 65;
        current = tag_start;
        println!(
            "//calcuated tag section start:{} end:{} tags: {}",
            tag_start,
            tag_end,
            tags_num
        );
        i = 0;
        let mut p_temp: bool;
        let mut id_temp: i32;
        for _ in 0..tags_num + 1 {
            match file[current] {
                1 => {
                    p_temp = true;
                    if debug {
                        println!("binparser: tag type + at {}", current);
                    }
                }
                0 => {
                    p_temp = false;
                    if debug {
                        println!("binparser: tag type - at {}", current);
                    }
                }
                _ => {
                    println!("fatal: invalid tag at {}:{}", current, file[current]);
                    exit(1);
                }
            }
            for m in file[current + 1..current + 31].iter() {
                if i == 31 {
                    i = 0;
                }
                numeric_parse_temp[i] = *m;
                i += 1;
            }
            id_temp = unzip_numeric(numeric_parse_temp).unwrap() as i32;
            if !p_temp {
                id_temp = -id_temp;
            }
            current += 33;
            for m in file[current..current + 30].iter() {
                if i == 31 {
                    i = 0;
                }
                numeric_parse_temp[i] = *m;
                i += 1;
            }
            ret.Tags.add_tag(Tag::new(
                id_temp,
                unzip_numeric(numeric_parse_temp).unwrap(),
            ));
            current += 32;
        }
    }
    ret
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
        "parse" | "p" => {
            if args.len() < 3 {
                println!("Error: please give a file to parse");
                exit(1);
            }
            if let Ok(f) = File::open(&args[2]) {
                let mut ret = Argument::new(ArgumentType::ParseProgram);
                ret.put_file(f);
                ret
            } else {
                println!("Error: can't open file '{}'", args[2]);
                exit(1);
            }
        }
        "beautify" | "b" => {
            if args.len() < 3 {
                println!("Error: please give a file to beautify");
                exit(1);
            }
            if let Ok(f) = File::open(&args[2]) {
                let mut ret = Argument::new(ArgumentType::BeautifyProgram);
                ret.put_file(f);
                ret
            } else {
                println!("Error: can't open file '{}'", args[2]);
                exit(1);
            }
        }
        "compile" | "c" => {
            if args.len() < 3 {
                println!("Error: please give a file to compile");
                exit(1);
            }
            if let Ok(f) = File::open(&args[2]) {
                let mut ret = Argument::new(ArgumentType::CompileSource);
                ret.put_file(f);
                ret
            } else {
                println!("Error: can't open file '{}'", args[2]);
                exit(1);
            }
        }
        "licence" | "l" => Argument::new(ArgumentType::PrintLicence),
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
    PrintLicence,
    ExecuteProgram,
    ParseProgram,
    BeautifyProgram,
    CompileSource,
}
