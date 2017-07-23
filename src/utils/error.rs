//module for printing error ~~名不副实~~
use utils::memory::CellType;

pub fn check_idpointer_validate(pointer: CellType) -> Presult {
    if pointer > 1023 || pointer < 0 {
        Presult::Err
    } else {
        Presult::Ok
    }
}

pub fn check_pointer_validate(pointer: usize) -> Presult {
    if pointer > 1023 {
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
