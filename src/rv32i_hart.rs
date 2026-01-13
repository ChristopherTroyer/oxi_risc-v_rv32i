use crate::hex;
use crate::memory::Memory;
use crate::register::RegisterFile;
use crate::rv32i_decode;
use std::io::Write;
use std::io::stdout;
use std::io::sink;

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
            show_instructions: false, //default false
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
                self.exec(insn, Some(&mut stdout()));
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

    fn exec(&self, insn:u32, writer: Option<&mut dyn Write>){

        let opcode: u32 = rv32i_decode::get_opcode(insn);
        let funct3: u32 = rv32i_decode::get_funct3(insn);
        let funct7: u32 = rv32i_decode::get_funct7(insn);


        //sink any output if ostream not provided
        let mut ostream: &mut dyn Write = &mut sink();
        match writer{
            Some(w) => {
                ostream = w;
            },
            None => {
            }
        }
        
        let _ = ostream.write_all("YEAHHHH!!!".as_bytes()); //Placeholder test
        let _ = ostream.flush();
    }

    fn exec_illegal_insn(& mut self, insn:u32, pos: &mut dyn Write){
        let _ = pos.write_all(rv32i_decode::render_illegal_insn(insn).as_bytes());
        let _ = pos.flush();
        self.halt = true;
        self.halt_reason = "Illegal instruction".to_string();
    }

    fn exec_add(& mut self, insn:u32, pos: &mut dyn Write){
        let rd: u32 = rv32i_decode::get_rd(insn);
        let rs1: u32 = rv32i_decode::get_rs1(insn);
        let rs2: u32 = rv32i_decode::get_rs2(insn);

        let val: i32 = self.regs.get(rs1) as i32 + self.regs.get(rs2) as i32;

        //render stuff here

        self.regs.set(rd,val);
        self.pc += 4;
    }
}