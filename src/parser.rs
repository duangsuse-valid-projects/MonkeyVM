extern crate test;

use utils::res::{HDataTypes, HCommands};
use vm::{TagManager, Tag};
use std::time::SystemTime;
use std::process::exit;

#[cfg(test)]
mod parser_tests {
    use parser::parse_program;
    use utils::res::{HCommands, HDataTypes};
    use parser::test::Bencher;
    #[test]
    fn it_works() {
        let program = "
        :point_right: 2
        :see_no_evil::point_right:3:point_right:
        :see_no_evil:3
        :hankey:
        ";
        let r = parse_program(program, false, false);
        match r.CMD[0] {
            HCommands::SUB => {}
            _ => panic!("parser err, expected SUB but {} given", r.CMD[0].to_str()),
        }
        match r.CMD[1] {
            HCommands::SUB => {}
            _ => panic!("parser err, expected SUB but {} given", r.CMD[1].to_str()),
        }
        match r.CMD[2] {
            HCommands::O => {}
            _ => panic!("parser err, expected O but {} given", r.CMD[2].to_str()),
        }
        match r.DAT[0] {
            HDataTypes::IndirectPointer(3) => {}
            _ => panic!("parser err, expected IPtr(3) but {:?} given", r.DAT[0]),
        }
        match r.DAT[1] { 
            HDataTypes::NumLiteral(3) => {}
            _ => panic!("parser err, expected 3 but {:?} given", r.DAT[1]),
        }
        assert_eq!(r.Tags.locate(2), Some(0));
    }
    #[bench]
    fn parser_speed(b: &mut Bencher) {
        b.iter(|| parse_bench())
    }
    fn parse_bench() {
        let program = "
//[prime_factorizer]
:point_right:1 0
:poultry_leg:
:memo::point_right: 1
:question::mailbox_with_no_mail::monkey: 4
:memo::point_right: 3
:eyes: 2
:memo::point_right: 2
:eyes: 0
:memo::point_right: 4
:point_right:2
:thumbsup::point_right: 4
:eyes::point_right: 2
:memo::point_right: 1023
:eyes::point_right: 1
:see_no_evil::point_right: 1023
:memo::point_right: 1
:question::banana::monkey: 2
:question::ghost::monkey: 3
:eyes: 0
:memo::point_right: 4
:eyes::point_right: 3
:memo::point_right: 1
:thumbsup::point_right: 2
:monkey: 2
:point_right:3
:eyes::point_right: 2
:hankey:
:thumbsdown::point_right: 4
:question::ghost::monkey: 1
:monkey_face: 1
:memo::point_right: 1
:memo::point_right: 3
";
        parse_program(program, false, false);
    }
}

pub fn parse_program(prog: &str, verbose: bool, debug: bool) -> MonkeyAST {
    let time_start = SystemTime::now();
    let mut ret = MonkeyAST::new();
    for (n, l) in prog.lines().enumerate() {
        let line_trim = l.split("//").next().unwrap_or(l);
        if line_trim.trim() != "" {
            parse_line(&n, l.replace(" ", "").as_str(), &mut ret, debug);
        }
    }
    let time_duration = SystemTime::now().duration_since(time_start);
    if debug {
        println!("parse finished in {:?}", time_duration);
    }
    if verbose {
        println!(
            "parse finished in {} secs. result: {:?}",
            time_duration.unwrap_or_default().as_secs(),
            ret
        );
    }
    ret
}
fn parse_line(ln: &usize, line: &str, target: &mut MonkeyAST, debug: bool) {
    if debug {
        println!("parser: parsing line {}:{}", ln, line);
    }
    if line.starts_with(":monkey_") {
        target.CMD.push(HCommands::ADD);
        target.DAT.push(datparse(HCommands::ADD, line, ln));
    } else if line.starts_with(":l") {
        target.CMD.push(HCommands::AO);
        target.DAT.push(datparse(HCommands::AO, line, ln));
    } else if line.starts_with(":pou") {
        target.CMD.push(HCommands::I);
        target.DAT.push(datparse(HCommands::I, line, ln));
    } else if line.starts_with(":poi") {
        let mut trimed = line.replace(":point_right:", "");
        if debug {
            println!("parser[parsetag]: trimed line:{}", trimed);
        }
        trimed = trimed.split("//").next().unwrap().to_string();
        if debug {
            println!("parser[parsetag]: splited line:{}", trimed);
        }
        target.Tags.add_tag(Tag::new(
            trimed.parse::<i32>().unwrap(),
            target.CMD.len() as u32,
        ));
    } else if line.starts_with(":monkey:") {
        target.CMD.push(HCommands::JMP);
        target.DAT.push(datparse(HCommands::JMP, line, ln));
    } else if line.starts_with(":h") {
        target.CMD.push(HCommands::O);
        target.DAT.push(datparse(HCommands::O, line, ln));
    } else if line.starts_with(":question::s") {
        target.CMD.push(HCommands::QNJ);
        target.DAT.push(datparse(HCommands::QNJ, line, ln));
    } else if line.starts_with(":question::m") {
        target.CMD.push(HCommands::QNU);
        target.DAT.push(datparse(HCommands::QNU, line, ln));
    } else if line.starts_with(":question::b") {
        target.CMD.push(HCommands::QPJ);
        target.DAT.push(datparse(HCommands::QPJ, line, ln));
    } else if line.starts_with(":question::g") {
        target.CMD.push(HCommands::QZJ);
        target.DAT.push(datparse(HCommands::QZJ, line, ln));
    } else if line.starts_with(":thumbsu") {
        target.CMD.push(HCommands::RAD);
        target.DAT.push(datparse(HCommands::RAD, line, ln));
    } else if line.starts_with(":e") {
        target.CMD.push(HCommands::RED);
        target.DAT.push(datparse(HCommands::RED, line, ln));
    } else if line.starts_with(":thumbsd") {
        target.CMD.push(HCommands::RSB);
        target.DAT.push(datparse(HCommands::RSB, line, ln));
    } else if line.starts_with(":s") {
        target.CMD.push(HCommands::SUB);
        target.DAT.push(datparse(HCommands::SUB, line, ln));
    } else if line.starts_with(":m") {
        target.CMD.push(HCommands::WRT);
        target.DAT.push(datparse(HCommands::WRT, line, ln));
    } else {
        println!("fatal: cannot parse command at line {}", ln + 1);
        exit(2);
    }
}
fn datparse(cmdtpe: HCommands, line: &str, ln: &usize) -> HDataTypes {
    let mut tmp: String = line.replace(cmdtpe.to_str(), "");
    tmp = tmp.split("//").next().unwrap().to_string();
    if let Ok(i) = tmp.parse::<i32>() {
        HDataTypes::NumLiteral(i)
    } else {
        if tmp.starts_with(":point_right:") && tmp.ends_with(":point_right:") {
            let replaced = tmp.replace(":point_right:", "");
            HDataTypes::IndirectPointer(replaced.parse::<usize>().unwrap())
        } else {
            if tmp == "" {
                HDataTypes::Nil
            } else {
                let replaced = tmp.replace(":point_right:", "");
                if let Ok(i) = replaced.parse::<usize>() {
                    HDataTypes::Pointer(i)
                } else {
                    println!("fatal: cannot parse data at line {}", ln + 1);
                    exit(2);
                }
            }
        }
    }
}

#[cfg(test)]
mod bin_util_works {
    #[test]
    fn numeric_zipper() {
        use parser::{zip_numeric, unzip_numeric};
        let mut tmp = zip_numeric(12123);
        assert_eq!(unzip_numeric(tmp), Some(12123));
        tmp = zip_numeric(12123);
        assert_eq!(unzip_numeric(tmp), Some(12123));
        tmp = zip_numeric(1323423);
        assert_eq!(unzip_numeric(tmp), Some(1323423));
        tmp = zip_numeric(1);
        assert_eq!(unzip_numeric(tmp), Some(1));
    }
}

pub fn zip_numeric(numeric: u32) -> [u8; 32] {
    let mut ret = [0u8; 32];
    let tmp: Vec<u8>;
    unsafe {
        let mut atemp = format!("{}", numeric);
        let temp = atemp.as_mut_vec();
        tmp = temp.to_vec();
    }
    let mut ptr = 0usize;
    for c in tmp {
        ret[ptr] = (c as char) as u8;
        ptr += 1;
    }
    ret
}
pub fn unzip_numeric(from: [u8; 32]) -> Option<u32> {
    let mut tmp = String::new();
    for b in from.iter() {
        let cur = *b as char;
        match cur {
            '1' => tmp.push('1'),
            '2' => tmp.push('2'),
            '3' => tmp.push('3'),
            '4' => tmp.push('4'),
            '5' => tmp.push('5'),
            '6' => tmp.push('6'),
            '7' => tmp.push('7'),
            '8' => tmp.push('8'),
            '9' => tmp.push('9'),
            '0' => tmp.push('0'),
            _ => {}
        }
    }
    match tmp.parse::<u32>() {
        Ok(t) => Some(t),
        Err(_) => None,
    }
}

//use array instead of vector for benchmark(higer performace).
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MonkeyAST {
    pub CMD: Vec<HCommands>,
    pub DAT: Vec<HDataTypes>,
    pub Tags: TagManager,
}
impl MonkeyAST {
    pub fn new() -> MonkeyAST {
        MonkeyAST {
            CMD: Vec::<HCommands>::new(),
            DAT: Vec::<HDataTypes>::new(),
            Tags: TagManager::new(),
        }
    }
}
