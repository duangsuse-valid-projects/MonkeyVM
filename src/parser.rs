use utils::res::{HDataTypes, HCommands};
use vm::{TagManager, Tag};
use std::str::SplitWhitespace;

#[cfg(test)]
mod parser_tests {
    use parser::parse_program;
    #[test]
    fn it_works() {
        let program = "
        :point_right: 2
        ";
        let r = parse_program(program);
    }
}

//TODO ~~impl. parser~~
//TODO write test for parser
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
    println!("pasing {}",data);
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
