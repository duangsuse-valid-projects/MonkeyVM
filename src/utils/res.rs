use utils::memory;
use utils::res::HDataTypes::*;

#[derive(Debug)]
pub enum HCommands {

}
impl HCommands {
    pub fn to_str(&self) -> String {
        match self {
            _ => format!("{:?}", self),
        }
    }
}
pub enum HDataTypes {
    NumLiterial(i32),
    Pointer(usize),
    IndirectPointer(usize),
}

#[cfg(test)]
mod tests {
    use utils::res::HDataTypes;
    use utils::memory::Hmem;
    #[test]
    fn datatypes_gets_real_value() {
        let test_hmem = Hmem::new();
        let test_datatype = HDataTypes::NumLiterial(33);
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
}

impl HDataTypes {
    pub fn get_value(self, hmem: memory::Hmem) -> memory::CellType {
        match self {
            NumLiterial(x) => x,
            Pointer(x) => hmem.get_cell(x),
            IndirectPointer(x) => hmem.get_cell_indirect(x),
        }
    }
}
