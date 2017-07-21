extern crate test;

use utils::res::{HDataTypes, HCommands};
use vm::{TagManager, Tag};
use std::str::SplitWhitespace;

#[cfg(test)]
mod parser_tests {
    use parser::parse_program;
    use utils::res::{HCommands, HDataTypes};
    use parser::test::Bencher;
    #[test]
    fn it_works() {
        let program = "
        :point_right: 2
        :see_no_evil:
        :see_no_evil:3
        :hankey:
        ";
        let r = parse_program(program);
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
            HDataTypes::Nil => {}
            _ => panic!("parser err, expected Nil but {:?} given", r.DAT[0]),
        }
        match r.DAT[1] { 
            HDataTypes::NumLiteral(3) => {}
            _ => {
                panic!(
                    "parser err, expected NumLiteral(3) but {:?} given",
                    r.DAT[1]
                )
            }
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
        parse_program(program);
    }
}

//TODO ~~impl. parser~~
//TODO write test for parser
pub fn parse_program(prog: &str) -> MonkeyAST {
    let mut ret = MonkeyAST::new();
    let mut line_real = 0;
    for (n, l) in prog.lines().enumerate() {
        let line_trim = l.split("//").next().unwrap_or(l);
        if line_trim.trim() != "" {
            parse_line(&n, &mut line_real, l.replace(" ", "").as_str(), &mut ret);
            line_real += 1;
        }
    }
    println!("parse finish. result: {:?}", ret);
    ret
}
fn parse_line(ln: &usize, mut ln_real: &mut usize, line: &str, target: &mut MonkeyAST) {
    //println!("parsing {} ...", &line);
    if line.starts_with(":monkey_") {
        target.CMD.push(HCommands::ADD);
        target.DAT.push(
            datparse(HCommands::ADD, line, ln, &mut ln_real),
        );
    } else if line.starts_with(":l") {
        target.CMD.push(HCommands::AO);
        target.DAT.push(
            datparse(HCommands::AO, line, ln, &mut ln_real),
        );
    } else if line.starts_with(":pou") {
        target.CMD.push(HCommands::I);
        target.DAT.push(
            datparse(HCommands::I, line, ln, &mut ln_real),
        );
    } else if line.starts_with(":poi") {
        if line.contains("//") {
            if ln_real != &0 {
                *ln_real = *ln_real - 1;
            }
        }
        let mut trimd = line.replace(":point_right:", "");
        trimd = trimd.split("//").next().unwrap().to_string();
        //println!("tagr trimed line: {}", trimd);
        target.Tags.add_tag(Tag::new(
            trimd.parse::<i32>().unwrap(),
            *ln_real as u32,
        ));
    } else if line.starts_with(":monkey:") {
        target.CMD.push(HCommands::JMP);
        target.DAT.push(
            datparse(HCommands::JMP, line, ln, &mut ln_real),
        );
    } else if line.starts_with(":h") {
        target.CMD.push(HCommands::O);
        target.DAT.push(
            datparse(HCommands::O, line, ln, &mut ln_real),
        );
    } else if line.starts_with(":question::s") {
        target.CMD.push(HCommands::QNJ);
        target.DAT.push(
            datparse(HCommands::QNJ, line, ln, &mut ln_real),
        );
    } else if line.starts_with(":question::m") {
        target.CMD.push(HCommands::QNU);
        target.DAT.push(
            datparse(HCommands::QNU, line, ln, &mut ln_real),
        );
    } else if line.starts_with(":question::b") {
        target.CMD.push(HCommands::QPJ);
        target.DAT.push(
            datparse(HCommands::QPJ, line, ln, &mut ln_real),
        );
    } else if line.starts_with(":question::g") {
        target.CMD.push(HCommands::QZJ);
        target.DAT.push(
            datparse(HCommands::QZJ, line, ln, &mut ln_real),
        );
    } else if line.starts_with(":thumbsu") {
        target.CMD.push(HCommands::RAD);
        target.DAT.push(
            datparse(HCommands::RAD, line, ln, &mut ln_real),
        );
    } else if line.starts_with(":e") {
        target.CMD.push(HCommands::RED);
        target.DAT.push(
            datparse(HCommands::RED, line, ln, &mut ln_real),
        );
    } else if line.starts_with(":thumbsd") {
        target.CMD.push(HCommands::RSB);
        target.DAT.push(
            datparse(HCommands::RSB, line, ln, &mut ln_real),
        );
    } else if line.starts_with(":s") {
        target.CMD.push(HCommands::SUB);
        target.DAT.push(
            datparse(HCommands::SUB, line, ln, &mut ln_real),
        );
    } else if line.starts_with(":m") {
        target.CMD.push(HCommands::WRT);
        target.DAT.push(
            datparse(HCommands::WRT, line, ln, &mut ln_real),
        );
    } else {
        panic!("fatal: can not parse command at line {}", ln + 1);
    }
}
fn datparse(cmdtpe: HCommands, line: &str, ln: &usize, lnr: &mut usize) -> HDataTypes {
    if line.contains("//") {
        if lnr != &0 {
            *lnr = *lnr - 1;
        }
    }
    let mut tmp: String = line.replace(cmdtpe.to_str(), "");
    tmp = tmp.split("//").next().unwrap().to_string();
    //println!("strriped: {}", tmp);
    if let Ok(i) = tmp.parse::<i32>() {
        HDataTypes::NumLiteral(i)
    } else {
        if tmp.starts_with(":point_right::") {
            let replaced = tmp.replace(":point_right::point_right:", "");
            HDataTypes::IndirectPointer(replaced.parse::<usize>().unwrap())
        } else {
            if tmp == "" {
                HDataTypes::Nil
            } else {
                let replaced = tmp.replace(":point_right:", "");
                if let Ok(i) = replaced.parse::<usize>() {
                    HDataTypes::Pointer(i)
                } else {
                    panic!("fatal: cannot parse data at line {}", ln + 1);
                }
            }
        }
    }
}
//以下第一次写的parser无法贴合需求,已被注释掉,仅供参考~~其实是写了几十分钟心疼不想移除掉~~
/*
pub fn parse_program(program: &str) -> MonkeyAST {
    let mut ret = MonkeyAST::new();
    let mut line_strriped = 0usize;
    for (n, l) in program.lines().enumerate() {
        let lineq = l.split("//").next().unwrap_or(l);
        if lineq.trim() == "" {
            line_strriped += 1;
        } else {
            parse_cmdata(&line_strriped, &n, lineq, &mut ret);
        }
    }
    ret
}
fn parse_cmdata(l: &usize, n: &usize, line: &str, ast: &mut MonkeyAST) {
    let mut line_splited = line.split_whitespace();
    let (c, should_panic) = parse_cmd(n - l, line_splited.next().unwrap(), ast);
    if let Some(v) = c {
        &ast.CMD.push(v);
    } else {
        if should_panic {
            panic!("can not parse command at line {}", n);
        }
    }
    if let Some(d) = parse_data(line_splited.next().unwrap()) {
        &ast.DAT.push(d);
    }
}
fn parse_cmd(l: usize, cmd: &str, ast: &mut MonkeyAST) -> (Option<HCommands>, bool) {
    println!("execute {}:{}", l, cmd);
    if cmd.starts_with(":point_right:") {
        let id: i32 = cmd.replace(":point_right:", "").trim().parse().unwrap();
        ast.Tags.add_tag(Tag::new(id, l as u32));
        (None, false)
    } else {
        (HCommands::ADD.from_str(cmd), true)
    }
}
fn parse_data(data: &str) -> Option<HDataTypes> {
    println!("pasing {}", data);
    let lit: Result<i32, _> = data.parse();
    if let Ok(l) = lit {
        Some(HDataTypes::NumLiteral(l))
    } else {
        if data.starts_with(":point_right:") {
            let val: usize = data.replace(":point_right", "").parse().unwrap();
            if data.replace(" ", "") == ":point_right::point_right:" {
                Some(HDataTypes::IndirectPointer(val))
            } else {
                Some(HDataTypes::Pointer(val))
            }
        } else {
            if data.trim() == "" {
                Some(HDataTypes::Nil)
            } else {
                None
            }
        }
    }
}
*/
//use array instead of vector for benchmark(higer performace).
#[derive(Debug)]
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
