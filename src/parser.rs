use utils::res::{HDataTypes, HCommands};
use vm::TagManager;

pub fn parse_program(program: &str) -> MonkeyAST {
    MonkeyAST::new()
}
//use array instead of vector for benchmark(higer performace).
#[derive(Debug)]
pub struct MonkeyAST {
    CMD: Vec<HCommands>,
    DAT: Vec<HDataTypes>,
    Tags: TagManager,
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
