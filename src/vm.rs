extern crate std_unicode;
extern crate test;
use std::fmt::{Display, Formatter, Result};
use std::time::SystemTime;
use self::std_unicode::char::from_u32;
#[allow(unused_imports)]
use vm::test::Bencher;
use parser;
use parser::MonkeyAST;
use utils::memory::{CellType, Hmem};
use utils::res::{HCommands, HDataTypes};

pub fn execute_program(program: &str, arg: Vec<CellType>, verbose: bool, debug: bool) {
    println!("Executing program with args: {:?} ...", arg);
    let time_start = SystemTime::now();
    let prog_ast = parser::parse_program(program, verbose, debug);
    let result = do_emulate(prog_ast, arg, verbose, debug);
    let time_end = SystemTime::now();
    let time_duration = time_end.duration_since(time_start).unwrap();
    println!(
        "Program finished in {} secs. {:?}",
        time_duration.as_secs(),
        time_duration
    );
    println!("{}", result);
}

#[cfg(test)]
mod tests_proc {
    use utils::res::{HCommands, HDataTypes};
    use parser::MonkeyAST;
    use vm::do_emulate;
    use vm::test::Bencher;
    use vm::Tag;
    #[bench]
    fn interpret_speed(b: &mut Bencher) {
        b.iter(|| interprete_well())
    }
    #[test]
    fn interprete_well() {
        let mut test_hprog = MonkeyAST::new();
        //x=Some(0)
        test_hprog.CMD.push(HCommands::ADD);
        test_hprog.DAT.push(HDataTypes::NumLiteral(9)); //x=Some(9)
        test_hprog.CMD.push(HCommands::ADD);
        test_hprog.DAT.push(HDataTypes::Nil); //x=Some(10)
        test_hprog.CMD.push(HCommands::WRT);
        test_hprog.DAT.push(HDataTypes::IndirectPointer(2)); //Ptr#0=10
        test_hprog.CMD.push(HCommands::ADD);
        test_hprog.DAT.push(HDataTypes::Nil); //x=Some(11)
        test_hprog.CMD.push(HCommands::RSB);
        test_hprog.DAT.push(HDataTypes::Pointer(0)); //x=Some(9)
        test_hprog.CMD.push(HCommands::O);
        test_hprog.DAT.push(HDataTypes::Nil);
        let r = do_emulate(test_hprog, vec![], false, false);
        assert_eq!(r.get_num()[0], 9);
    }

    #[test]
    fn ao_works() {
        let mut hprog = MonkeyAST::new();
        hprog.CMD.push(HCommands::RED);
        hprog.DAT.push(HDataTypes::NumLiteral(0x0061)); //Unicode/ascii char a 0x0061
        hprog.CMD.push(HCommands::AO);
        hprog.DAT.push(HDataTypes::Nil);
        let r = do_emulate(hprog, vec![], false, false);
        assert_eq!(r.get_ascii()[0], 'a');
    }
    #[test]
    fn addsub_works() {
        let mut hprog = MonkeyAST::new();
        hprog.CMD.push(HCommands::ADD);
        hprog.DAT.push(HDataTypes::Nil);
        hprog.CMD.push(HCommands::WRT);
        hprog.DAT.push(HDataTypes::Pointer(0)); //memory #0=1
        hprog.CMD.push(HCommands::ADD);
        hprog.DAT.push(HDataTypes::IndirectPointer(9)); //memory #0=2
        hprog.CMD.push(HCommands::RED);
        hprog.DAT.push(HDataTypes::Pointer(0));
        hprog.CMD.push(HCommands::O);
        hprog.DAT.push(HDataTypes::Nil);
        let r = do_emulate(hprog, vec![], false, false);
        assert_eq!(r.get_num()[0], 1);
    }
    #[test]
    fn jump_works() {
        let mut hprog = MonkeyAST::new();
        hprog.Tags.add_tag(Tag::new(1, 0));
        hprog.Tags.add_tag(Tag::new(2333, 5));
        hprog.CMD.push(HCommands::I); //ln0 input
        hprog.DAT.push(HDataTypes::Nil);
        hprog.CMD.push(HCommands::QNU); //ln1 if x=Nil jump to ln5,end program
        hprog.DAT.push(HDataTypes::NumLiteral(2333));
        hprog.CMD.push(HCommands::ADD); //ln2 add one
        hprog.DAT.push(HDataTypes::Nil);
        hprog.CMD.push(HCommands::O); //ln3 output
        hprog.DAT.push(HDataTypes::Nil);
        hprog.CMD.push(HCommands::JMP); //ln4 jump back
        hprog.DAT.push(HDataTypes::NumLiteral(1));
        hprog.CMD.push(HCommands::ADD);
        hprog.DAT.push(HDataTypes::Nil);
        let r = do_emulate(hprog, vec![-1, 3, 5], false, false);
        assert_eq!(r.get_num(), [0, 4, 6]);
    }
    #[test]
    fn read_iptr_works() {
        use parser::parse_program;
        use vm::do_emulate;
        let program_ast = parse_program(
            "
:eyes:2233 //read 2233 to x
:memo::point_right:1 // write to #1
:eyes:1023 //read 1023 to x
:memo::point_right:0 //write 1023 to 0
:eyes:69 //read 69 to x
:memo::point_right:0:point_right: //write 69 to #1023
:thumbsup::point_right:1023 //#1023-> 70
:eyes::point_right:1023 //read #1023 to x
:hankey:
:eyes::point_right:1 //read 2233 to x
:hankey:
:eyes::point_right:0 //read 1023 to x
:hankey:
:see_no_evil::point_right:0:point_right:
:hankey:
        ",
            false,
            false,
        );
        let result = do_emulate(program_ast, vec![], false, false);
        assert_eq!(result.get_num(), [70, 2233, 1023, 953]);
    }
}

fn borrow_data(dat: &HDataTypes) -> HDataTypes {
    match dat {
        &HDataTypes::IndirectPointer(p) => HDataTypes::IndirectPointer(p),
        &HDataTypes::NumLiteral(n) => HDataTypes::NumLiteral(n),
        &HDataTypes::Pointer(p) => HDataTypes::Pointer(p),
        &HDataTypes::Nil => HDataTypes::Nil,
    }
}

//emulate hlang program with memory
fn do_emulate(hast: MonkeyAST, arg: Vec<CellType>, verbose: bool, debug: bool) -> PResult {
    let mut ln = 0usize;
    let mut presult = PResult::new();
    let mut x: Option<CellType> = None;
    let mut mem: Hmem = Hmem::new();
    let mut input = InputManager::new(arg);
    let mut steps = 0u32;
    let mut jumps = 0u32;
    let mut jumps_t = 0u32;
    let mut noplusline = false;
    loop {
        if ln >= hast.CMD.len() {
            break;
        }
        let command_current = &hast.CMD[ln];
        let data_current = borrow_data(&hast.DAT[ln]);
        if debug {
            println!(
                "vm: executing command:{:?} with data {:?}",
                command_current,
                data_current
            );
        }
        match command_current {
            &HCommands::ADD => {
                match data_current {
                    HDataTypes::Nil => x = Some(x.unwrap_or(0) + 1),
                    HDataTypes::IndirectPointer(v) => {
                        let val = x.unwrap_or(0) + &mem.get_cell_indirect(v);
                        x = Some(val);
                    }
                    HDataTypes::NumLiteral(v) => x = Some(x.unwrap_or(0) + v),
                    HDataTypes::Pointer(v) => {
                        let val = x.unwrap_or(0) + &mem.get_cell(v);
                        x = Some(val);
                    }
                }
            }
            &HCommands::AO => {
                if verbose {
                    println!("ascii outputting... x={:?}", x);
                }
                match data_current {
                    HDataTypes::Nil => {
                        if x == None {
                            println!("WARN: attempt to output when x is a None value near {}", ln);
                        } else {
                            presult.add_char_from_ascii(x.unwrap());
                        }
                    }
                    HDataTypes::IndirectPointer(_) => {
                        presult.add_char_from_ascii(data_current.get_value(&mem))
                    }
                    HDataTypes::Pointer(_) => {
                        presult.add_char_from_ascii(data_current.get_value(&mem))
                    }
                    HDataTypes::NumLiteral(n) => presult.add_char_from_ascii(n), 
                }
            }
            &HCommands::I => {
                let next_param = input.feed();
                if verbose {
                    println!("feeding {:?} to x-{:?}", next_param, x);
                }
                x = next_param;
            }
            &HCommands::JMP => {
                if verbose {
                    println!("process jump at cmd#{}", ln);
                }
                noplusline = true;
                jumps += 1;
                ln = hast.Tags.locate(data_current.get_value(&mem)).unwrap() as usize;
                if jumps > 10000 {
                    println!(
                        "WARN: monkey has jumped for over 10000 times (to:{}) ,reset timer.",
                        ln
                    );
                    jumps_t += jumps;
                    jumps = 0;
                }
            }
            &HCommands::O => {
                if verbose {
                    println!("outputting... x={:?}", x);
                }
                match data_current {
                    HDataTypes::Nil => {
                        if x == None {
                            println!("WARN: attempt to output when x is a None value near {}", ln);
                        } else {
                            presult.add_num(x.unwrap());
                        }
                    }
                    HDataTypes::IndirectPointer(_) => presult.add_num(data_current.get_value(&mem)),
                    HDataTypes::Pointer(_) => presult.add_num(data_current.get_value(&mem)),
                    HDataTypes::NumLiteral(n) => presult.add_num(n), 
                }
            }
            &HCommands::QNJ => {
                if verbose {
                    println!("negative-jumping from {}... x={:?}", ln, x);
                }
                if x == None {
                    println!("fatal: attempt to negative-jump on a None value,force stop.");
                    break;
                }
                if x.unwrap() < 0 {
                    noplusline = true;
                    jumps += 1;
                    ln = hast.Tags.locate(data_current.get_value(&mem)).unwrap() as usize;
                    if jumps > 10000 {
                        println!(
                            "WARN: monkey has jumped for over 10000 times (to:{}) ,reset timer.",
                            ln
                        );
                        jumps_t += jumps;
                        jumps = 0;
                    }
                }
            }
            &HCommands::QNU => {
                if verbose {
                    println!("none-jumping from {}... x={:?}", ln, x);
                }
                if x == None {
                    noplusline = true;
                    jumps += 1;
                    ln = hast.Tags.locate(data_current.get_value(&mem)).unwrap() as usize;
                    if jumps > 10000 {
                        println!(
                            "WARN: monkey has jumped for over 10000 times (to:{}) ,reset timer.",
                            ln
                        );
                        jumps_t += jumps;
                        jumps = 0;
                    }
                }
            }
            &HCommands::QPJ => {
                if verbose {
                    println!("positive-jumping from {}... x={:?}", ln, x);
                }
                if x == None {
                    println!("fatal: attempt to positive-jump on a None value,force stop.");
                    break;
                }
                if x.unwrap() > 0 {
                    noplusline = true;
                    jumps += 1;
                    ln = hast.Tags.locate(data_current.get_value(&mem)).unwrap() as usize;
                    if jumps > 10000 {
                        println!(
                            "WARN: monkey has jumped over for 10000 times (to:{}) ,reset timer.",
                            ln
                        );
                        jumps_t += jumps;
                        jumps = 0;
                    }
                }
            }
            &HCommands::QZJ => {
                if verbose {
                    println!("zero-jumping from {}... x={:?}", ln, x);
                }
                if x == None {
                    println!("fatal: attempt to zero-jump on a None value,force stop.");
                    break;
                }
                if x.unwrap() == 0 {
                    noplusline = true;
                    jumps += 1;
                    ln = hast.Tags.locate(data_current.get_value(&mem)).unwrap() as usize;
                    if jumps > 10000 {
                        println!(
                            "WARN: monkey has jumped over for 10000 times (to:{}) ,reset timer.",
                            ln
                        );
                        jumps_t += jumps;
                        jumps = 0;
                    }
                }
            }
            &HCommands::RAD => {
                match data_current {
                    HDataTypes::Pointer(p) => {
                        let val = &mem.get_cell(p) + 1;
                        &mem.put_cell(p, val);
                        x = Some(val);
                    }
                    HDataTypes::IndirectPointer(p) => {
                        let val = &mem.get_cell_indirect(p) + 1;
                        &mem.put_cell_indirect(p, val);
                        x = Some(val);
                    } 
                    HDataTypes::NumLiteral(_) => {
                        println!("WARN: trying to RAD without pointer {}", ln)
                    }
                    HDataTypes::Nil => {
                        println!("WARN: trying to RAD without any param near {}", ln)
                    }
                }
            }
            &HCommands::RED => x = Some(data_current.get_value(&mem)),
            &HCommands::RSB => {
                match data_current {
                    HDataTypes::Pointer(p) => {
                        let val = &mem.get_cell(p) - 1;
                        &mem.put_cell(p, val);
                        x = Some(val);
                    }
                    HDataTypes::IndirectPointer(p) => {
                        let val = &mem.get_cell_indirect(p) - 1;
                        &mem.put_cell_indirect(p, val);
                        x = Some(val);
                    } 
                    HDataTypes::NumLiteral(_) => {
                        println!("WARN: trying to RSB without pointer {}", ln)
                    }
                    HDataTypes::Nil => {
                        println!("WARN: trying to RSB without any param near {}", ln)
                    }
                }
            }
            &HCommands::SUB => {
                match data_current {
                    //>_<WTF
                    HDataTypes::Nil => x = Some(x.unwrap_or(0) - 1),
                    HDataTypes::IndirectPointer(v) => {
                        let val = x.unwrap_or(0) - &mem.get_cell_indirect(v);
                        x = Some(val);
                    }
                    HDataTypes::NumLiteral(v) => x = Some(x.unwrap_or(0) - v),
                    HDataTypes::Pointer(v) => {
                        let val = x.unwrap_or(0) - &mem.get_cell(v);
                        x = Some(val);
                    }
                }
            }
            &HCommands::WRT => {
                match data_current {
                    HDataTypes::Pointer(p) => {
                        let val = x.unwrap_or(0);
                        &mem.put_cell(p, val);
                    }
                    HDataTypes::IndirectPointer(p) => {
                        let val = x.unwrap_or(0);
                        &mem.put_cell_indirect(p, val);
                    } 
                    HDataTypes::NumLiteral(_) => {
                        println!("WARN: trying to Write without pointer {}", ln)
                    }
                    HDataTypes::Nil => {
                        println!("WARN: trying to Write without any param near {}", ln)
                    }
                }
            }
        }
        if noplusline {
            noplusline = false;
        } else {
            ln += 1;
        }
        if jumps_t > 900000 {
            use std::io::stdin;
            println!("monkey got *really* tired,contiue simulate? [y/n]");
            let mut user_input = String::new();
            stdin().read_line(&mut user_input).unwrap();
            if user_input.trim() == "y" {
                println!("contiue. resetting total jumps: {}", jumps_t + jumps);
                jumps = 0;
                jumps_t = 0;
            } else {
                break;
            }
        }
        steps += 1;
    }
    presult.put_step(steps);
    if verbose || debug {
        println!(
            "VM core: total jump count: {} ,end line: {} ,memory snap: {} ,x: {:?}",
            jumps_t + jumps,
            ln,
            mem.pretty(),
            x
        );
    }
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
    #[allow(unused)]
    pub fn get_step(&self) -> u32 {
        self.step
    }
    #[allow(unused)]
    pub fn get_num(self) -> Vec<CellType> {
        self.out_num
    }
    #[allow(unused)]
    pub fn get_ascii(self) -> Vec<char> {
        self.out_ascii
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
ascii output: ['a']
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
ascii output: ['f', 'o', 'o']
steps:8",
        ));
    }
}
impl Display for PResult {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "numeric output: {:?}\nascii output: {:?}\nsteps:{}",
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
            if n == self.tags.len() - 1 {
                break;
            }
            n += 1;
        }
        if n == self.tags.len() {
            None
        } else {
            Some(self.tags[n].get_lo())
        }
    }
    pub fn locate_print_reverse(&self, lo: usize) {
        for t in &self.tags {
            if t.get_lo() as usize == lo {
                println!(":point_right: {}", t.get_id());
            }
        }
    }
    pub fn locate_print_tail_tag(&self, len: usize) {
        for t in &self.tags {
            if t.get_lo() as usize > len {
                println!(":point_right: {}", t.get_id());
            }
        }
    }
    pub fn add_tag(&mut self, tag: Tag) {
        self.tags.push(tag);
    }
    pub fn print_fmt(self) {
        for t in self.tags {
            println!("--{}--{}--", t.get_id(), t.get_lo());
        }
    }
}

#[cfg(test)]
mod tests_im {
    use vm::InputManager;
    #[test]
    fn it_works() {
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
#[allow(non_snake_case)]
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
