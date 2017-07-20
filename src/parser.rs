use utils::res::{HDataTypes, HCommands};
use vm::TagManager;

pub fn parse_program(program: &str) -> MonkeyAST {
    let mut ret = MonkeyAST::new();
    ret.CMD.push(HCommands::ADD);
    ret.CMD.push(HCommands::O);
    ret.DAT.push(HDataTypes::NumLiteral(2));
    ret.DAT.push(HDataTypes::Nil);
    ret
}
//use array instead of vector for benchmark(higer performace).
#[derive(Debug)]
pub struct MonkeyAST {
    pub CMD: Vec<HCommands>,
    pub DAT: Vec<HDataTypes>,
    pub Tags: TagManager,
}
impl MonkeyAST {
    fn new() -> MonkeyAST {
        MonkeyAST {
            CMD: Vec::<HCommands>::new(),
            DAT: Vec::<HDataTypes>::new(),
            Tags: TagManager::new(),
        }
    }
}
