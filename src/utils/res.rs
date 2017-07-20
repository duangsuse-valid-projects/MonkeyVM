use std::slice::Iter;
use utils::memory;
use utils::res::HDataTypes::*;

#[derive(Debug)]
pub enum HCommands {
    RED, //read memory to x , :eyes:
    RAD, //read memory,add one, write to x ::
    RSB, //read memory,sub one, write to x ::
    SUB, //sub x
    ADD, //plus one to x
    WRT, //write x to memory
    JMP, //jump to tag
    QNU, //if x is blank,jump to tag
    QPJ, //if x is positive,jump to tag
    QZJ, //if x is zero,jump to tag
    QNJ, //if x is negative,tump to tag
    O, //dump x to numeric output
    AO, //dump x to ascii output(as ascii char)
    I, //Input in to x
}
impl HCommands {
    pub fn to_str(&self) -> &str {
        match self {
            &HCommands::ADD => ":monkey_face:",
            &HCommands::AO => ":loudspeaker:",
            &HCommands::I => ":poultry_leg:",
            &HCommands::JMP => ":monkey:",
            &HCommands::O => ":hankey:",
            &HCommands::QNJ => ":question::scream::monkey:",
            &HCommands::QNU => ":question::mailbox_with_no_mail::monkey:",
            &HCommands::QPJ => ":question::banana::monkey:",
            &HCommands::QZJ => ":question::ghost::monkey:",
            &HCommands::RAD => ":thumbsup:",
            &HCommands::RED => ":eyes:",
            &HCommands::RSB => ":thumbsdown",
            &HCommands::SUB => ":see_no_evil:",
            &HCommands::WRT => ":memo:",
        }
    }
    /* Bad lifetime
    pub fn from_str<'a>(&'a self, cmd_str: &'a str) -> Option<&'a HCommands> {
        let tmp = Hcmd_as_array();
        let tmp_iter = tmp.iter();
        for i in tmp_iter {
            if i.to_str() == cmd_str {
                return Some(i);
            }
        }
        None
    }
    */

    //Lifetime bad guy.... go study lifetime...
    pub fn from_str(&self, cmd_str: &str) -> Option<HCommands> {
        if cmd_str == HCommands::SUB.to_str() {
            Some(HCommands::SUB)
        } else if cmd_str == HCommands::WRT.to_str() {
            Some(HCommands::WRT)
        } else if cmd_str == HCommands::RSB.to_str() {
            Some(HCommands::RSB)
        } else if cmd_str == HCommands::RED.to_str() {
            Some(HCommands::RED)
        } else if cmd_str == HCommands::RAD.to_str() {
            Some(HCommands::RAD)
        } else if cmd_str == HCommands::QZJ.to_str() {
            Some(HCommands::QZJ)
        } else if cmd_str == HCommands::QPJ.to_str() {
            Some(HCommands::QPJ)
        } else if cmd_str == HCommands::QNU.to_str() {
            Some(HCommands::QNU)
        } else if cmd_str == HCommands::QNJ.to_str() {
            Some(HCommands::QNJ)
        } else if cmd_str == HCommands::O.to_str() {
            Some(HCommands::O)
        } else if cmd_str == HCommands::JMP.to_str() {
            Some(HCommands::JMP)
        } else if cmd_str == HCommands::I.to_str() {
            Some(HCommands::I)
        } else if cmd_str == HCommands::AO.to_str() {
            Some(HCommands::AO)
        } else if cmd_str == HCommands::ADD.to_str() {
            Some(HCommands::ADD)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub enum HDataTypes {
    NumLiteral(i32),
    Pointer(usize),
    IndirectPointer(usize),
}

#[cfg(test)]
mod tests {
    use utils::res::{HDataTypes, HCommands};
    use utils::memory::Hmem;
    #[test]
    fn datatypes_gets_real_value() {
        let test_hmem = Hmem::new();
        let test_datatype = HDataTypes::NumLiteral(33);
        assert_eq!(test_datatype.get_value(test_hmem), 33);
        let mut test_hmem = Hmem::new();
        test_hmem.put_cell(2, 22);
        let test_datatype_ptr = HDataTypes::Pointer(2);
        assert_eq!(test_datatype_ptr.get_value(test_hmem), 22);
        let mut test_hmem = Hmem::new();
        test_hmem.put_cell_indirect(3, 33);
        let test_datatype_iptr = HDataTypes::IndirectPointer(3);
        assert_eq!(test_datatype_iptr.get_value(test_hmem), 33);
    }
    #[test]
    fn command_parser_works() {
        let command_str = ":see_no_evil:";
        let cmd = HCommands::ADD; //Bad Lifetime
        let cmd_o = cmd.from_str(command_str).unwrap();
        match cmd_o {
            HCommands::SUB => {}
            _ => panic!("bad command parser"),
        }
    }
    #[test]
    fn command_parser_None_on_input_worng() {
        let command_str = ":bad_command:";
        let cmd = HCommands::ADD; //Bad Lifetime
        let cmd_o = cmd.from_str(command_str);
        match cmd_o {
            None => {}
            _ => panic!("bad command parser"),
        }
    }
}

impl HDataTypes {
    pub fn get_value(self, hmem: memory::Hmem) -> memory::CellType {
        match self {
            NumLiteral(x) => x,
            Pointer(x) => hmem.get_cell(x),
            IndirectPointer(x) => hmem.get_cell_indirect(x),
        }
    }
}
/*
pub fn Hcmd_as_array() -> [HCommands; 14] {
    [
        HCommands::ADD,
        HCommands::AO,
        HCommands::I,
        HCommands::JMP,
        HCommands::O,
        HCommands::QNJ,
        HCommands::QNU,
        HCommands::QPJ,
        HCommands::QZJ,
        HCommands::RAD,
        HCommands::RED,
        HCommands::RSB,
        HCommands::SUB,
        HCommands::WRT,
    ]
}
*/
