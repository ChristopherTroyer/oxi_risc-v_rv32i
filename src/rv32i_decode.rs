use std::string;

use crate::hex::{self, to_hex0x20};

const MNEMONIC_WIDTH:  u32 = 8;

const OPCODE_LUI:      u32 = 0b0110111;
const OPCODE_AUIPC:    u32 = 0b0010111;
const OPCODE_JAL:      u32 = 0b1101111;
const OPCODE_JALR:     u32 = 0b1100111;
const OPCODE_BTYPE:    u32 = 0b1100011;
const OPCODE_LOAD_IMM: u32 = 0b0000011;
const OPCODE_STYPE:    u32 = 0b0100011;
const OPCODE_ALU_IMM:  u32 = 0b0010011;
const OPCODE_RTYPE:    u32 = 0b0110011;
const OPCODE_SYSTEM:   u32 = 0b1110011;

const FUNCT3_BEQ:   u32 = 0b000;
const FUNCT3_BNE:   u32 = 0b001;
const FUNCT3_BLT:   u32 = 0b100;
const FUNCT3_BGE:   u32 = 0b101;
const FUNCT3_BLTU:  u32 = 0b110;
const FUNCT3_BGEU:  u32 = 0b111;
const FUNCT3_LB:    u32 = 0b000;
const FUNCT3_LH:    u32 = 0b001;
const FUNCT3_LW:    u32 = 0b010;
const FUNCT3_LBU:   u32 = 0b100;
const FUNCT3_LHU:   u32 = 0b101;
const FUNCT3_SB:    u32 = 0b000;
const FUNCT3_SH:    u32 = 0b001;
const FUNCT3_SW:    u32 = 0b010;
const FUNCT3_ADD:   u32 = 0b000;
const FUNCT3_SLL:   u32 = 0b001;
const FUNCT3_SLT:   u32 = 0b010;
const FUNCT3_SLTU:  u32 = 0b011;
const FUNCT3_XOR:   u32 = 0b100;
const FUNCT3_SRX:   u32 = 0b101;
const FUNCT3_OR:    u32 = 0b110;
const FUNCT3_AND:   u32 = 0b111;

const FUNCT7_SRL:    u32 = 0b0000000;
const FUNCT7_SRA:    u32 = 0b0100000;
const FUNCT7_ADD:    u32 = 0b0000000;
const FUNCT7_SUB:    u32 = 0b0100000;
const INSN_ECALL:    u32 = 0x00000073;
const INSN_EBREAK:   u32 = 0x00100073;

const FUNCT3_CSRRW:  u32 = 0b001;
const FUNCT3_CSRRS:  u32 = 0b010;
const FUNCT3_CSRRC:  u32 = 0b011;
const FUNCT3_CSRRWI: u32 = 0b101;
const FUNCT3_CSRRSI: u32 = 0b110;
const FUNCT3_CSRRCI: u32 = 0b111;

fn get_opcode(insn:u32) -> u32{
    insn & 0x0000007f
}

fn get_rd(insn:u32) -> u32{
    (insn & 0x00000f80)>>7
}

fn get_func3(insn:u32) -> u32{
    (insn & 0x00007000)>>12
}

fn get_rs1(insn:u32) -> u32{
    (insn & 0x000f8000)>>15
}

fn get_rs2(insn:u32) -> u32{
    (insn & 0x01f00000)>>20
}

fn get_func7(insn:u32) -> u32{
    (insn & 0xfe000000)>>25
}

//immediate values

fn get_imm_i(insn:u32) -> i32{
    let mut imm_i:u32 = (insn & 0xfff00000) >> 20;
    if (imm_i & 0x800) != 0 {
        imm_i |= 0xfffff000;
    }
    imm_i
}

fn get_imm_u(insn:u32) -> i32{
    //Zero Extended on the right
    let imm_u: i32 = insn as i32 & 0xfffff000;
    imm_u
}

fn get_imm_b(insn:u32) -> i32{
    let mut imm_b: i32 = (insn as i32 & 0x80000000) >> (31-12);
    imm_b |= (insn as i32  & 0x7e000000) >> (25-5);
    imm_b |= (insn as i32  & 0x00000f00) >> (8-1);
    imm_b |= (insn as i32  & 0x00000080) << (11-7);

    //sign exted the left
    if (insn as i32 & 0x80000000) != 0{
        imm_b |= 0xffffe000;
    }

    imm_b
}

fn get_imm_s(insn:u32) -> i32{
    let mut imm_s: i32 = (insn as i32  & 0xfe000000) >> (25-5);
    imm_s |= (insn as i32  & 0x00000f80) >> (7-0);
    
    if (insn as i32  & 0x80000000) != 0{
        imm_s |= 0xfffff000; // sign-extend the left
    }

    imm_s
}

fn get_imm_j(insn:u32) -> i32{
    let mut imm_j: i32 = (insn as i32  & 0x80000000) >> (31-20);
    imm_j |= (insn as i32  & 0x7fe00000) >> (21-1);
    imm_j |= (insn as i32  & 0x00100000) >> (20-11);
    imm_j |= (insn as i32  & 0x000ff000);

    if (insn as i32 & 0x80000000) != 0{
        imm_j |= 0xffe00000;
    }

    imm_j
}

//render instructions

fn render_mnemonic(m:String) -> String{
    format!("{:<8}",m)
}

fn render_reg(r:u32) -> String{
    format!("x{}",r)
}

fn render_illegal_insn(insn:u32) -> String{
    "ERROR: UNIMPLEMENTED INSTRUCTION".to_string()
}

fn render_lui(insn:u32) -> String{
    let rd = get_rd(insn);
    let imm_u = get_imm_u(insn);
    format!("{}{},{}",render_mnemonic("lui".to_string()),render_reg(rd),to_hex0x20((imm_u as u32>>12)&0x0fffff))
}

fn render_auipc(insn:u32) -> String{
    let rd = get_rd(insn);
    let imm_u = get_imm_u(insn);
    format!("{}{},{}",render_mnemonic("auipc".to_string()),render_reg(rd),to_hex0x20((imm_u as u32>>12)&0x0fffff))
}

fn render_jal(addr:u32, insn:u32) -> String{
    let rd = get_rd(insn);
    let imm_j = get_imm_j(insn);
    format!("{}{},{}",render_mnemonic("jal".to_string()),render_reg(rd),hex::to_hex0x32(imm_j as u32 + addr))
}

fn render_jalr(insn:u32) -> String{
    let rd = get_rd(insn);
    let rs1 = get_rs1(insn);
    let imm_i = get_imm_i(insn);
    format!("{}{},{}",render_mnemonic("jal".to_string()),render_reg(rd),render_base_disp(imm_i, rs1))
}

//types

fn render_btype(addr:u32, insn:u32, mnemonic:String) -> String{
    
}

fn render_itype_load(insn:u32, mnemonic:String) -> String{
    
}

fn render_stype(insn:u32, mnemonic:String) -> String{
    
}

fn render_itype_alu(insn:u32, mnemonic:String, imm_i:i32) -> String{
    
}

fn render_rtype(insn:u32, mnemonic:String) -> String{
    
}

//render controls

fn render_ecall(insn:u32, mnemonic:String) -> String{
    "ecall".to_string()
}

fn render_ebreak(insn:u32, mnemonic:String) -> String{
    "ebreak".to_string()
}

//helpers

fn render_csrrx(insn:u32, mnemonic:String) -> String{
    
}

fn render_csrrxi(insn:u32, mnemonic:String) -> String{
    
}

fn render_base_disp(base:i32, disp:u32) -> String{

}

//Big decode monster
//Look into improving dispatch

fn decode(addr:u32, insn:u32) -> String{

}