use crate::kernel::traits::vk_kernel::Symbol;

pub struct ConcreteSymbol;

impl Symbol for ConcreteSymbol {
    fn export(&self, sym_addr: u32, name: &str) {

    }

    fn unexport(&self, name: &str) {

    }
    
    fn search(&self, name: &str) {

    }
}
