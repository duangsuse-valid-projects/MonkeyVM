//module for printing error
use utils::memory::CellType;

pub fn check_idpointer_validate(pointer: CellType) -> Presult {
    if pointer > 1024 || pointer < 0 {
        /*panic!(
            "invalid indirect pointer: {} doesn't point to an valid memory location",
            pointer
        );*/
        Presult::Err
    } else {
        Presult::Ok
    }
}

#[derive(Debug)]
pub enum Presult {
    Ok,
    Err,
}
impl Presult {
    pub fn unwrap(self) {
        match self {
            Presult::Ok => {}
            Presult::Err => panic!("attemp to unwrap on an Err value"),
        }
    }
}
