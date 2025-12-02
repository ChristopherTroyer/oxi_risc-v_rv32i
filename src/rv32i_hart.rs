use crate::hex;
use crate::memory::Memory;
use crate::register::RegisterFile;


//this struct needs a reference to memory and register file
pub struct rv32i_hart{
    mem: Memory,
    regs: RegisterFile,

    pc: u32,
    insn_counter: u64,
    mhartid: u32,

    halt: bool,
    halt_reason: String,
    show_instructions: bool,
    show_registers: bool,
}

impl rv32i_hart {
    pub fn new(&self, m:&Memory)
    {

    }
    
    fn dump(&self, hdr:String) -> String{
        let mut result = String::new();
        result.push_str(&self.regs.dump(hdr.to_string()));
        result.push_str(&format!("{} pc {}", hdr.to_string(), hex::to_hex32(self.pc)));
        result
    }
}
