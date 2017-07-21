use utils::res::{HDataTypes, HCommands};
use vm::{TagManager, Tag};

//TODO impl. parser
pub fn parse_program(program: &str) -> MonkeyAST {
    let mut ret = MonkeyAST::new();
    ret.Tags.add_tag(Tag::new(1, 0));
    ret.Tags.add_tag(Tag::new(2333, 5));
    ret.CMD.push(HCommands::I); //ln0 input
    ret.DAT.push(HDataTypes::Nil);
    ret.CMD.push(HCommands::QNU); //ln1 if x=Nil jump to ln5,end program
    ret.DAT.push(HDataTypes::NumLiteral(2333));
    ret.CMD.push(HCommands::ADD); //ln2 add one
    ret.DAT.push(HDataTypes::Nil);
    ret.CMD.push(HCommands::O); //ln3 output
    ret.DAT.push(HDataTypes::Nil);
    ret.CMD.push(HCommands::JMP); //ln4 jump back
    ret.DAT.push(HDataTypes::NumLiteral(1));
    ret.CMD.push(HCommands::ADD);
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
    pub fn new() -> MonkeyAST {
        MonkeyAST {
            CMD: Vec::<HCommands>::new(),
            DAT: Vec::<HDataTypes>::new(),
            Tags: TagManager::new(),
        }
    }
}
