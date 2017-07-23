//monkey-lang memory implemention
use utils::error::check_idpointer_validate;

pub type CellType = i32;
pub const MEM_SIZE: usize = 1024;

pub struct Hmem {
    memory: [CellType; MEM_SIZE],
}
#[cfg(test)]
mod tests {
    use utils::memory::Hmem;
    #[test]
    fn it_works() {
        let hmem = Hmem::new();
        assert_eq!(hmem.get_cell(0), 0, "assert initialized memory[0] equals 0");
        hmem.get_cell_indirect(1023);
    }
    #[test]
    fn index_well() {
        let mut hmem = Hmem::new();
        hmem.put_cell(3, 4); //cell 3 with value 4
        assert_eq!(hmem.get_cell(3), 4);
        hmem.put_cell(4, 6); //cell 4 with value 6
        assert_eq!(hmem.get_cell_indirect(3), 6);
    }
    #[test]
    #[should_panic]
    fn panic_out_of_range() {
        let mut hmem = Hmem::new();
        hmem.put_cell(0, 1029);
        hmem.get_cell_indirect(0);
    }
    #[test]
    fn puts_indirect_mem() {
        let mut hmem = Hmem::new();
        hmem.put_cell(3, 100);
        hmem.put_cell_indirect(3, 11);
        assert_eq!(hmem.get_cell(100), 11);
    }
}
impl Hmem {
    pub fn new() -> Hmem {
        Hmem { memory: [0; MEM_SIZE] }
    }
    #[allow(unused)]
    pub fn get_memory(&self) -> [i32; 1024] {
        self.memory
    }
    pub fn get_cell(&self, poniter: usize) -> CellType {
        self.memory[poniter]
    }
    pub fn put_cell(&mut self, pointer: usize, value: CellType) {
        self.memory[pointer] = value;
    }
    pub fn get_cell_indirect(&self, poniter: usize) -> CellType {
        //check_idpointer_validate(cell_pointer);
        let ptr = self.cell_points_to(poniter);
        //please give me a piece of advice >_>
        //let ptr_string = format!("{}", cell_pointer);
        //let ptr: usize = ptr_string.parse().unwrap();
        self.memory[ptr]
    }
    pub fn put_cell_indirect(&mut self, pointer: usize, value: CellType) {
        let ptr = self.cell_points_to(pointer);
        //println!("put {} to #{}",value,ptr);
        self.put_cell(ptr, value);
    }
    fn cell_points_to(&self, cell: usize) -> usize {
        use utils::error::Presult;
        let cell_contains = self.get_cell(cell);
        match check_idpointer_validate(cell_contains) {
            Presult::Err => {
                panic!(
                    "fatal: cell points_to failed: #{}:{} isn't a valid pointer",
                    cell,
                    cell_contains
                )
            }
            Presult::Ok => {}
        }
        cell_contains as usize
    }
    pub fn pretty(&self) -> String {
        let mut ret = String::new();
        let mut valcel_list = Vec::<usize>::new();
        for (n, c) in self.memory.iter().enumerate() {
            if c != &0 {
                valcel_list.push(n);
            }
        }
        for p in valcel_list {
            ret += format!("#{}:{}", p, self.memory[p]).as_str();
        }
        ret
    }
}
