use utils::res::{HDataTypes, HCommands};
use vm::{TagManager, Tag};

//TODO impl. parser
pub fn parse_program(program: &str) -> MonkeyAST {
    let mut ret = MonkeyAST::new();
    for l in program.lines() {
        
    }
    ret
}
fn parse_data(data: &str) -> HDataTypes {
    
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
