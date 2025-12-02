use crate::hex;
use crate::memory::Memory;
use crate::register::RegisterFile;


//this is a single hardware thread (hart)
pub struct Rv32iHart{
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

impl Rv32iHart {
    pub fn new(mut self, m:Memory, r:RegisterFile)
    {
        self.mem = m;
        self.regs = r;

        self.pc = 0;
        self.insn_counter = 0;
        self.mhartid = 0;

        self.halt = false;
        self.halt_reason = "none".to_string();
    }
    
    fn dump(&self, hdr:String) -> String{
        let mut result = String::new();
        result.push_str(&self.regs.dump(hdr.to_string()));
        result.push_str(&format!("{} pc {}", hdr.to_string(), hex::to_hex32(self.pc)));
        result
    }
}
