extern crate std_unicode;
extern crate test;
use self::test::Bencher;
use std::fmt::{Display, Formatter, Result};
use std::time::SystemTime;
use self::std_unicode::char::from_u32;
use parser;
use parser::MonkeyAST;
use utils::memory::{CellType, Hmem};
use utils::res::{HCommands, HDataTypes};

pub fn execute_program(program: &str, arg: Vec<CellType>) {
    println!("Executing program... args: {:?}", arg);
    let time_start = SystemTime::now();
    let prog_ast = parser::parse_program(program);
    //TODO :-( I don't know,either!
    let result = do_emulate(prog_ast, arg);
    let time_end = SystemTime::now();
    let time_duration = time_end.duration_since(time_start).unwrap();
    println!(
        "Program finished in {} secs. ({:?})",
        time_duration.as_secs(),
        time_duration
    );
    println!("{}", result);
}

#[cfg(test)]
mod tests_proc {
    #[test]
    fn inprete_well() {
        let test_hprog = MonkeyAST::new();
    }
}

//Clone trait??
fn borrowing_workaround(dat: &HDataTypes) -> HDataTypes {
    match dat {
        &HDataTypes::IndirectPointer(p) => HDataTypes::IndirectPointer(p),
        &HDataTypes::NumLiteral(n) => HDataTypes::NumLiteral(n),
        &HDataTypes::Pointer(p) => HDataTypes::Pointer(p),
        &HDataTypes::Nil => HDataTypes::Nil,
    }
}

//emulate hlang program with memory
fn do_emulate(hast: MonkeyAST, arg: Vec<CellType>) -> PResult {
    let mut ln = 0usize; //line executing
    let mut presult = PResult::new();
    let mut x: Option<CellType> = Some(0);
    let mut mem: Hmem = Hmem::new();
    let mut input = InputManager::new(arg);
    let mut steps = 0u32;
    loop {
        let command_current = &hast.CMD[ln];
        let data_current = borrowing_workaround(&hast.DAT[ln]);
        match command_current {
            &HCommands::ADD => {
                match data_current {
                    HDataTypes::Nil => x = Some(x.unwrap()+1),
                    HDataTypes::IndirectPointer(v) => {
                        let val = &mem.get_cell_indirect(v)+1;
                        &mem.put_cell_indirect(v,val);
                    },
                    HDataTypes::NumLiteral(v) => {

                    },
                    HDataTypes::Pointer(v) => {

                    }
                    _ => {},
                }
            }
            &HCommands::AO => {
                presult.add_char_from_ascii(x.unwrap());
            }
            &HCommands::I => {
                x = input.feed();
            }
            &HCommands::JMP => {
                ln = hast.Tags.locate(data_current.get_value(&mem)).unwrap() as usize;
            }
            &HCommands::O => {
                presult.add_num(x.unwrap());
            }
            &HCommands::QNJ => {
                if x.unwrap() < 0 {
                    ln = hast.Tags.locate(data_current.get_value(&mem)).unwrap() as usize;
                }
            }
            &HCommands::QNU => {
                if x == None {
                    ln = hast.Tags.locate(data_current.get_value(&mem)).unwrap() as usize;
                }
            }
            &HCommands::QPJ => {
                if x.unwrap() > 0 {
                    ln = hast.Tags.locate(data_current.get_value(&mem)).unwrap() as usize;
                }
            }
            &HCommands::QZJ => {
                if x.unwrap() == 0 {
                    ln = hast.Tags.locate(data_current.get_value(&mem)).unwrap() as usize;
                }
            }
            &HCommands::RAD => {
                x = Some(data_current.get_value(&mem) + 1);
            }
            &HCommands::RED => x = Some(data_current.get_value(&mem)),
            &HCommands::RSB => x = Some(data_current.get_value(&mem) - 1),
            &HCommands::SUB => x = Some(x.unwrap() - data_current.get_value(&mem)),
            &HCommands::WRT => {
                let ptr = data_current.get_value(&mem) as usize;
                mem.put_cell(ptr, x.unwrap());
            }
            _ => panic!("unsupported command:{:?}", command_current),
        }
        ln += 1;
        steps+=1;
        if ln == hast.CMD.len() {
            break;
        }
    }
    presult.put_step(steps);
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
    pub fn add_char_from_ascii(&mut self, output: CellType) {
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
        assert_eq!(
            format!("{}", pres),
            "numeric output: []
ascii output:['a']
steps:2"
        );
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

#[cfg(test)]
mod tests_tgmgr {
    use vm::{TagManager, Tag, Bencher};
    #[test]
    fn it_works() {
        let mut tm = TagManager::new();
        tm.add_tag(Tag::new(0, 2));
        assert_eq!(tm.locate(0), Some(2));
    }
    #[test]
    #[should_panic]
    fn none_on_not_exist() {
        let tm = TagManager::new();
        tm.locate(3).unwrap();
    }
    #[bench]
    fn tag_locate_speed(b: &mut Bencher) {
        b.iter(move || tm_bench())
    }
    fn tm_bench() {
        let mut tm = TagManager::new();
        tm.add_tag(Tag::new(4, 384));
        tm.add_tag(Tag::new(7, 384));
        tm.add_tag(Tag::new(3428, 384));
        tm.add_tag(Tag::new(354, 384));
        tm.add_tag(Tag::new(324, 384));
        tm.add_tag(Tag::new(65, 384));
        tm.add_tag(Tag::new(422, 384));
        tm.add_tag(Tag::new(234, 384));
        tm.add_tag(Tag::new(24, 384));
        tm.add_tag(Tag::new(9, 384));
        tm.locate(4);
        tm.locate(7);
        tm.locate(3428);
        tm.locate(354);
        tm.locate(324);
        tm.locate(65);
        tm.locate(422);
        tm.locate(234);
        tm.locate(24);
        tm.locate(9);
    }
}

#[derive(Debug)]
pub struct Tag {
    id: i32,
    lo: u32,
}
impl Tag {
    pub fn new(id: i32, lo: u32) -> Tag {
        Tag { id: id, lo: lo }
    }
    pub fn get_id(&self) -> i32 {
        self.id
    }
    pub fn get_lo(&self) -> u32 {
        self.lo
    }
}
#[derive(Debug)]
pub struct TagManager {
    tags: Vec<Tag>,
}
impl TagManager {
    pub fn new() -> TagManager {
        TagManager { tags: Vec::<Tag>::new() }
    }
    pub fn locate(&self, id: i32) -> Option<u32> {
        let mut n: usize = 0;
        loop {
            if self.tags[n].get_id() == id {
                break;
            }
            n += 1;
        }
        Some(self.tags[n].get_lo())
    }
    pub fn add_tag(&mut self, tag: Tag) {
        self.tags.push(tag);
    }
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
