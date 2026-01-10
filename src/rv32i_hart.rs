use crate::hex;
use crate::memory::Memory;
use crate::register::RegisterFile;
use std::io::Write;
use std::io;
use std::ptr;


//this is a single hardware thread (hart)
pub struct Rv32iHart{
    mem: Memory,
    regs: RegisterFile,

    pc: u32,
    insn_counter: u64,
    pub mhartid: u32,

    halt: bool,
    halt_reason: String,
    pub show_instructions: bool,
    show_registers: bool,
}

impl Rv32iHart {
    pub fn new(m:Memory, r:RegisterFile) -> Rv32iHart
    {
        Rv32iHart {
            mem: m,
            regs: r,
            pc: 0,
            insn_counter: 0,
            mhartid: 0,
            halt: false,
            halt_reason: "none".to_string(),
            show_instructions: true, //default false
            show_registers: false,
        }
    }
    

    pub fn tick(mut self, hdr:String){
        if self.show_registers{
            println!("{}",self.dump(hdr.to_string()));
        }
        
        if (self.pc % 4 != 0){
            self.halt = true;
            self.halt_reason = "PC alignment error".to_string();
        }
        else {
            self.insn_counter+=1;
            let insn:u32 = self.mem.get32(self.pc);

            if self.show_instructions{
                print!("{}",&format!("{} {}: {} ", hdr.to_string(), hex::to_hex32(self.pc), hex::to_hex32(insn)));
                self.exec(insn, Some(&mut io::stdout()));
                println!();
            }
            else {
                self.exec(insn, None);
            }
        }
    }
    
    pub fn dump(&self, hdr:String) -> String{
        let mut result = String::new();
        result.push_str(&self.regs.dump(hdr.to_string()));
        result.push_str(&format!("{} pc {}", hdr.to_string(), hex::to_hex32(self.pc)));
        result
    }

    pub fn reset(&mut self){
        self.pc = 0;
        self.regs.reset();
        self.insn_counter = 0;
        self.halt = false;
        self.halt_reason = "none".to_string();
    }

    fn exec(&self, insn:u32, writer: Option<&mut dyn io::Write>){
        match writer{
            Some(w) => {
                let _ = w.write_all("YEAHHHH!!!".as_bytes()); //Placeholder
                let _ = w.flush();
            },
            None => {
                //do nothing
            }
        }
    }
}