extern crate std_unicode;
use std::fmt::{Display, Formatter, Result};
use std::time::SystemTime;
use self::std_unicode::char::from_u32;
use parser;
use parser::MonkeyAST;
use utils::memory::CellType;

pub fn execute_program(program: &str, arg: Vec<CellType>) {
    println!("Executing program... args: {:?}", arg);
    let time_start = SystemTime::now();
    let prog_ast = parser::parse_program(program);
    //TODO :-( I don't know,either!
    let result = do_emulate(prog_ast);
    let time_end = SystemTime::now();
    let time_duration = time_end.duration_since(time_start).unwrap();
    println!(
        "Program finished in {} secs. ({:?})",
        time_duration.as_secs(),
        time_duration
    );
    println!("{}", result);
}

fn do_emulate(hast: MonkeyAST) -> PResult {
    let ln = 1u32; //line executing
    let presult = PResult::new();
    presult
}

#[derive(Debug)]
pub struct PResult {
    out_num: Vec<CellType>,
    out_ascii: Vec<char>,
    step: u32,
}
impl PResult {
    pub fn new() -> PResult {
        PResult {
            out_num: Vec::<CellType>::new(),
            out_ascii: Vec::<char>::new(),
            step: 0u32,
        }
    }
    pub fn add_num(&mut self, number: CellType) {
        self.out_num.push(number);
    }
    pub fn add_char(&mut self, output: char) {
        self.out_ascii.push(output);
    }
    pub fn add_char_from_ascii(&mut self,output: CellType) {
        self.add_char(from_u32(output as u32).unwrap());
    }
    pub fn put_step(&mut self, step: u32) {
        self.step = step;
    }
}
#[cfg(test)]
mod tests {
    use vm::PResult;
    #[test]
    fn ascii_convert_works() {
        let mut pres = PResult::new();
        pres.add_char_from_ascii(0x0061);
        pres.put_step(2);
        assert_eq!(format!("{}",pres),"numeric output: []
ascii output:['a']
steps:2");
    }
    #[test]
    fn test_presult_beautifier() {
        let mut pres = PResult::new();
        pres.add_char('f');
        pres.add_char('o');
        pres.add_char('o');
        pres.add_num(0);
        pres.put_step(8);
        println!("{}", pres);
        assert!(format!("{}", pres).contains(
            "numeric output: [0]
ascii output:['f', 'o', 'o']
steps:8",
        ));
    }
}
impl Display for PResult {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "numeric output: {:?}\nascii output:{:?}\nsteps:{}",
            self.out_num,
            self.out_ascii,
            self.step
        )
    }
}
#[derive(Debug)]
struct Tag {
    id: i32,
    lo: u32,
}
impl Tag {

}
#[derive(Debug)]
struct TagManager {
    tags: Vec<Tag>,
}
impl TagManager {

}

#[cfg(test)]
mod tests_im {
    #[test]
    fn it_works() {
        use vm::InputManager;
        let test_vec = vec![0, 2, 3, 42, 1];
        let mut im = InputManager::new(test_vec);
        assert_eq!(im.feed().unwrap(), 0);
        assert_eq!(im.feed().unwrap(), 2);
        assert_eq!(im.feed().unwrap(), 3);
        assert_eq!(im.feed().unwrap(), 42);
        assert_eq!(im.feed().unwrap(), 1);
        assert_eq!(im.feed(), None);
    }
}

#[derive(Debug)]
struct InputManager {
    Input: Vec<CellType>,
    last: usize,
}
impl InputManager {
    fn new(inp: Vec<CellType>) -> InputManager {
        InputManager {
            Input: inp,
            last: 0,
        }
    }
    fn feed(&mut self) -> Option<CellType> {
        self.last += 1;
        if self.last > self.Input.len() {
            None
        } else {
            Some(self.Input[self.last - 1])
        }
    }
}
