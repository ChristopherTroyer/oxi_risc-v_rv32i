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

const XLEN:i32 = 32;

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
    imm_i as i32
}

fn get_imm_u(insn:u32) -> i32{
    //Zero Extended on the right
    let imm_u: u32 = insn & 0xfffff000;
    imm_u as i32
}

fn get_imm_b(insn:u32) -> i32{
    let mut imm_b: u32 = (insn & 0x80000000) >> (31-12);
    imm_b |= (insn & 0x7e000000) >> (25-5);
    imm_b |= (insn & 0x00000f00) >> (8-1);
    imm_b |= (insn & 0x00000080) << (11-7);

    //sign exted the left
    if (insn & 0x80000000) != 0{
        imm_b |= 0xffffe000;
    }

    imm_b as i32
}

fn get_imm_s(insn:u32) -> i32{
    let mut imm_s: u32 = (insn & 0xfe000000) >> (25-5);
    imm_s |= (insn & 0x00000f80) >> (7-0);
    
    if (insn & 0x80000000) != 0{
        imm_s |= 0xfffff000; // sign-extend the left
    }

    imm_s as i32
}

fn get_imm_j(insn:u32) -> i32{
    let mut imm_j: u32 = (insn & 0x80000000) >> (31-20);
    imm_j |= (insn & 0x7fe00000) >> (21-1);
    imm_j |= (insn & 0x00100000) >> (20-11);
    imm_j |= (insn & 0x000ff000);

    if (insn & 0x80000000) != 0{
        imm_j |= 0xffe00000;
    }

    imm_j as i32
}

//render instructions

fn render_mnemonic(m:&str) -> String{
    format!("{:<8}",m)
}

fn render_reg(r:u32) -> String{
    format!("x{}",r)
}

fn render_illegal_insn(insn:u32) -> String{
    "ERROR: UNIMPLEMENTED INSTRUCTION".to_string()
}

fn render_lui(insn:u32) -> String{
    let rd: u32 = get_rd(insn);
    let imm_u: i32 = get_imm_u(insn);
    format!("{}{},{}",render_mnemonic("lui"),render_reg(rd),to_hex0x20((imm_u as u32>>12)&0x0fffff))
}

fn render_auipc(insn:u32) -> String{
    let rd: u32 = get_rd(insn);
    let imm_u: i32 = get_imm_u(insn);
    format!("{}{},{}",render_mnemonic("auipc"),render_reg(rd),to_hex0x20((imm_u as u32>>12)&0x0fffff))
}

fn render_jal(addr:u32, insn:u32) -> String{
    let rd: u32 = get_rd(insn);
    let imm_j: i32 = get_imm_j(insn);
    format!("{}{},{}",render_mnemonic("jal"),render_reg(rd),hex::to_hex0x32(imm_j.wrapping_add_unsigned(addr) as u32)) //POSSIBLY NOT CORRECT BUT WRAPPING TO AVOID OVERFLOW PANIC
}

fn render_jalr(insn:u32) -> String{
    let rd: u32 = get_rd(insn);
    let rs1: u32 = get_rs1(insn);
    let imm_i: i32 = get_imm_i(insn);
    format!("{}{},{}",render_mnemonic("jal"),render_reg(rd),render_base_disp(imm_i, rs1))
}

//types

fn render_btype(addr:u32, insn:u32, mnemonic:&str) -> String{
    let imm_b: i32 = get_imm_b(insn);
    let rs1: u32 = get_rs1(insn);
    let rs2: u32 = get_rs2(insn);

    format!("{}{},{},{}", render_mnemonic(mnemonic), render_reg(rs1),render_reg(rs2),hex::to_hex0x32(imm_b.wrapping_add_unsigned(addr) as u32)) //POSSIBLY NOT CORRECT BUT WRAPPING TO AVOID OVERFLOW PANIC
}

fn render_itype_load(insn:u32, mnemonic:&str) -> String{
    let rd: u32 = get_rd(insn);
    let rs1: u32 = get_rs1(insn);
    let imm_i: i32 = get_imm_i(insn);
    
    format!("{}{},{}", render_mnemonic(mnemonic),render_reg(rd),render_base_disp(imm_i, rs1))
}

fn render_stype(insn:u32, mnemonic:&str) -> String{
    let imm_s: i32 = get_imm_s(insn);
    let rs1: u32 = get_rs1(insn);
    let rs2: u32 = get_rs2(insn);

    format!("{}{},{}", render_mnemonic(mnemonic),render_reg(rs2),render_base_disp(imm_s, rs1))
}

fn render_itype_alu(insn:u32, mnemonic:&str, imm_i:i32) -> String{
    let rd: u32 = get_rd(insn);
    let rs1: u32 = get_rs1(insn);

    format!("{}{},{},{}",render_mnemonic(mnemonic),render_reg(rd),render_reg(rs1),imm_i)
}

fn render_rtype(insn:u32, mnemonic:&str) -> String{
    let rd: u32 = get_rd(insn);
    let rs1: u32 = get_rs1(insn);
    let rs2: u32 = get_rs2(insn);

    format!("{}{},{},{}", render_mnemonic(mnemonic),render_reg(rd),render_reg(rs1),render_reg(rs2))
}

//render controls

fn render_ecall(_insn:u32) -> String{
    "ecall".to_string()
}

fn render_ebreak(_insn:u32) -> String{
    "ebreak".to_string()
}

//helpers

fn render_csrrx(insn:u32, mnemonic:&str) -> String{
    let rd: u32 = get_rd(insn);
    let rs1: u32 = get_rs1(insn);
    let imm_u: u32 = get_imm_u(insn) as u32;

    format!("{}{},{},{}", render_mnemonic(mnemonic), render_reg(rd),hex::to_hex0x12((imm_u>>20)&0xfff), render_reg(rs1))
}

fn render_csrrxi(insn:u32, mnemonic:&str) -> String{
    let rd: u32 = get_rd(insn);
    let rs1: u32 = get_rs1(insn);
    let imm_u: u32 = get_imm_u(insn) as u32;

    format!("{}{},{},{}", render_mnemonic(mnemonic), render_reg(rd),hex::to_hex0x12((imm_u>>20)&0xfff), rs1)
}

fn render_base_disp(base:i32, disp:u32) -> String{
    format!("{}({})",base,render_reg(disp))
}

//Big decode monster
//Look into improving dispatch

pub fn decode(addr:u32, insn:u32) -> String{
    let opcode:u32 = get_opcode(insn);
    let funct3: u32 = get_func3(insn);
    let funct7:u32 = get_func7(insn);
    let imm_i: i32 = get_imm_i(insn);

    //println!("opcode={} funct3={} funct7={} imm_i={}", opcode,funct3,funct7,imm_i);
    //"DEBUG NOT IMPLEMENTED"

    match opcode{
        OPCODE_SYSTEM => match funct3{
            FUNCT3_CSRRW => return render_csrrx(insn, "csrrw"),
            FUNCT3_CSRRS => return render_csrrx(insn, "csrrs"),
            FUNCT3_CSRRC => return render_csrrx(insn, "csrrc"),
            FUNCT3_CSRRWI => return render_csrrx(insn, "csrrwi"),
            FUNCT3_CSRRSI => return render_csrrx(insn, "csrrsi"),
            FUNCT3_CSRRCI => return render_csrrx(insn, "csrrci"),
            FUNCT3_BEQ => match insn{
                INSN_EBREAK => render_ebreak(insn),
                INSN_ECALL => render_ecall(insn),
                _ => render_illegal_insn(insn)
            }
            _ => render_illegal_insn(insn)
            //replaced assert(0 && "unrecognized imm_i"); with panic but likely not needed
            //panic!("unrecognized immi")
        }

        OPCODE_LUI => return render_lui(insn),
        OPCODE_AUIPC => return render_auipc(insn),
        OPCODE_JAL => return render_jal(addr, insn),
        OPCODE_JALR => return render_jalr(insn),
        OPCODE_BTYPE => match funct3 {
            FUNCT3_BEQ => return render_btype(addr, insn, "beq"),
            FUNCT3_BNE => return render_btype(addr, insn, "bne"),
            FUNCT3_BLT => return render_btype(addr, insn, "blt"),
            FUNCT3_BGE => return render_btype(addr, insn, "bge"),
            FUNCT3_BLTU => return render_btype(addr, insn, "bltu"),
            FUNCT3_BGEU => return render_btype(addr, insn, "bgeu"),
            _ => return render_illegal_insn(insn),
        }

        OPCODE_LOAD_IMM => match funct3 {
            FUNCT3_LB => return render_itype_load(insn, "lb"),
            FUNCT3_LH => return render_itype_load(insn, "lh"),
            FUNCT3_LW => return render_itype_load(insn, "lw"),
            FUNCT3_LBU => return render_itype_load(insn, "lbu"),
            FUNCT3_LHU => return render_itype_load(insn, "lhu"),
            _ => return render_illegal_insn(insn),
        }

        OPCODE_STYPE => match funct3 {
            FUNCT3_SB => return render_stype(insn, "sb"),
            FUNCT3_SH => return render_stype(insn, "sh"),
            FUNCT3_SW => return render_stype(insn, "sw"),
            _ => return render_illegal_insn(insn),
        }

        OPCODE_ALU_IMM => match funct3 {
            FUNCT3_ADD => return render_itype_alu(insn, "addi", imm_i),
            FUNCT3_SLL => return render_itype_alu(insn, "slli", imm_i%XLEN), //unsure xlen
            FUNCT3_SLT => return render_itype_alu(insn, "slti", imm_i),
            FUNCT3_SLTU => return render_itype_alu(insn, "sltiu", imm_i),
            FUNCT3_XOR => return render_itype_alu(insn, "xori", imm_i),
            FUNCT3_AND => return render_itype_alu(insn, "andi", imm_i),
            FUNCT3_OR => return render_itype_alu(insn, "ori", imm_i),
            FUNCT3_SRX => match funct7 {
                FUNCT7_SRA => return render_itype_alu(insn, "srai", imm_i%XLEN),
                FUNCT7_SRL => return render_itype_alu(insn, "srli", imm_i%XLEN), 
                _ => return render_illegal_insn(insn),
            }
            _ => return render_illegal_insn(insn),
        }

        OPCODE_RTYPE => match funct3 {
            //TODO: FILL IN AND CONTINUE FROM LINE 323 in rv32i_decode.cpp
            FUNCT3_ADD => match(funct7){
                FUNCT7_ADD => return render_rtype(insn, "add"),
                FUNCT7_SUB => return render_rtype(insn, "sub"),
                _ => return render_illegal_insn(insn),
            }

            FUNCT3_SRX => match(funct7){
                FUNCT7_SRA => return render_rtype(insn, "sra"),
                FUNCT7_SRL => return render_rtype(insn, "srl"),
                _ => return render_illegal_insn(insn),
            }

            FUNCT3_OR => return render_rtype(insn, "or"),
            FUNCT3_XOR => return render_rtype(insn, "xor"),
            FUNCT3_AND => return render_rtype(insn, "and"),
            FUNCT3_SLT => return render_rtype(insn, "slt"),
            FUNCT3_SLL => return render_rtype(insn, "sll"),
            FUNCT3_SLTU => return render_rtype(insn, "sltu"),
            _ => return render_illegal_insn(insn),
        }
        //catch-all
        _ => return render_illegal_insn(insn),
    }
}