use crate::hex;
use crate::memory::Memory;
use crate::register::RegisterFile;
use crate::rv32i_decode;
use std::io::Write;
use std::io::stdout;
use std::io::sink;

const INSTRUCTION_WIDTH: i32 = 35;

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
        
        let _ = ostream.write_all("YEAHHHH!!!\n".as_bytes()); //Placeholder test
        let _ = ostream.flush();
    }

    fn exec_illegal_insn(& mut self, insn:u32, pos: &mut dyn Write){
        if self.show_instructions{
            let _ = pos.write_all(rv32i_decode::render_illegal_insn(insn).as_bytes());
            let _ = pos.flush();
        }

        self.halt = true;
        self.halt_reason = "Illegal instruction".to_string();
    }

    fn exec_add(& mut self, insn:u32, pos: &mut dyn Write){
        let rd: u32 = rv32i_decode::get_rd(insn);
        let rs1: u32 = rv32i_decode::get_rs1(insn);
        let rs2: u32 = rv32i_decode::get_rs2(insn);

        let val: i32 = self.regs.get(rs1) as i32 + self.regs.get(rs2) as i32;

        //render stuff here
        if self.show_instructions{
            let mut s: String = rv32i_decode::render_rtype(insn, "add");
            s = format!("{:<width$}",s,width = INSTRUCTION_WIDTH as usize);
            s = format!("{}// {} = {} + {} = {}\n",s, 
            rv32i_decode::render_reg(rd),
            hex::to_hex0x32(self.regs.get(rs1) as u32),
            hex::to_hex0x32(self.regs.get(rs2) as u32),
            hex::to_hex0x32(val as u32));

            let _ = pos.write_all(s.as_bytes());
            let _ = pos.flush();
        }

        self.regs.set(rd,val);
        self.pc += 4;
    }

    fn exec_addi(& mut self, insn:u32, pos: &mut dyn Write){
        let rd: u32 = rv32i_decode::get_rd(insn);
        let rs1: u32 = rv32i_decode::get_rs1(insn);
        let imm_i: i32 = rv32i_decode::get_imm_i(insn);

        let val: i32 = self.regs.get(rs1) as i32 + imm_i as i32;

        //render stuff here
        if self.show_instructions{
            let mut s: String = rv32i_decode::render_itype_alu(insn, "addi", imm_i);
            s = format!("{:<width$}",s,width = INSTRUCTION_WIDTH as usize);
            s = format!("{}// {} = {} + {} = {}\n",s, 
            rv32i_decode::render_reg(rd),
            hex::to_hex0x32(self.regs.get(rs1) as u32),
            hex::to_hex0x32(imm_i as u32),
            hex::to_hex0x32(val as u32));

            let _ = pos.write_all(s.as_bytes());
            let _ = pos.flush();
        }

        self.regs.set(rd,val);
        self.pc += 4;
    }

    fn exec_and(& mut self, insn:u32, pos: &mut dyn Write){
        let rd: u32 = rv32i_decode::get_rd(insn);
        let rs1: u32 = rv32i_decode::get_rs1(insn);
        let rs2: u32 = rv32i_decode::get_rs2(insn);

        let val: i32 = self.regs.get(rs1) & self.regs.get(rs2);

        //put render stuff here
        //TODO: look into switching render work off if pos is sink

        self.regs.set(rd,val);
        self.pc += 4;

    }

    fn exec_andi(& mut self, insn:u32, pos: &mut dyn Write){
        let rd: u32 = rv32i_decode::get_rd(insn);
        let rs1: u32 = rv32i_decode::get_rs1(insn);
        let imm_i: i32 = rv32i_decode::get_imm_i(insn);

        let val: i32 = self.regs.get(rs1) & imm_i;

        //put render stuff here
        //TODO: look into switching render work off if pos is sink

        self.regs.set(rd,val);
        self.pc += 4;

    }

    fn exec_auipc(& mut self, insn:u32, pos: &mut dyn Write){
        let rd: u32 = rv32i_decode::get_rd(insn);
        let imm_u: i32 = rv32i_decode::get_imm_u(insn);

        let val: i32 = self.pc as i32 + imm_u;

        //render stuff

        self.regs.set(rd, val);
        self.pc+=4;

    }

// ----------------------------------------------------------------
// B type
    fn exec_btype(& mut self, insn:u32, pos: &mut dyn Write){
        let funt3: u32 =  rv32i_decode::get_funct3(insn);
        let rs1: u32 =  rv32i_decode::get_rs1(insn);
        let rs2: u32 =  rv32i_decode::get_rs2(insn);
        let imm_b: i32 =  rv32i_decode::get_imm_b(insn);

        let mut render_str:String = String::new();
        let mut mnemonic:String = String::new();




    }

// ----------------------------------------------------------------
// C type

}