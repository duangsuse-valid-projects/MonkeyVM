//module for printing error
use utils::memory::CellType;

pub fn check_idpointer_validate(pointer: CellType) -> Presult {
    if pointer > 1023 || pointer < 0 {
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
