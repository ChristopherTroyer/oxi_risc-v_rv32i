// RISC-V Simulator Rust implementation

use std::process::exit;
use clap::Parser;
use std::path::PathBuf;

use crate::register::RegisterFile;
use crate::memory::Memory;

mod hex;
mod register;
mod memory;
mod rv32i_decode;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli{
    #[arg(short = 'd',long, help="Show memory disassembly before running simulation.")]
    disassembly: bool,
    #[arg(short,long,help="Print instructions during execution.")]
    instructions: bool,
    #[arg(short = 'r',long,help="Show dump of hart before each instruction is simulated.")]
    dump_hart: bool,
    #[arg(short = 'z',long,help="Show dump of hart status and memory after simulation is haulted.")]
    dump_hart_after: bool,
    #[arg(short = 'm',long,default_value_t = 0x100, help="Specify memory size.")] //default 0x100
    memory_limit: u32,
    #[arg(short = 'l', long, default_value_t = 0, help="Maximum limit of instructions to execute.")]
    exec_limit: u64,
    //#[arg(value_name = "FILE", help="Input file to load into memory.",required=true)]
    //input_file: PathBuf,
}

/**
 *  loops through memory and decodes each 32-bit instruction
 *  mem current system memory
 */
fn disassemble(mem:&Memory) -> String{
    let mut result = String::new();
    for i in (0..mem.get_size()).step_by(4){
        result.push_str(&format!("{:0>8}: ", hex::to_hex32(i)));
        result.push_str(&format!("{:0>8}", hex::to_hex32(mem.get32(i))));
        //render instruction via decode function
        result.push_str(&format!("  {}\n",rv32i_decode::decode(i,mem.get32(i))));
    }
    result
}

fn main(){
    let args = Cli::parse();
    //let rf = RegisterFile::new();
    let mut mem = Memory::new(args.memory_limit);
    //let filename = args.input_file;
    //println!("Loading file: {:?}", filename);


    mem.load_file("target/debug/input/a4/badhex.bin".to_string());

    println!("{}", disassemble(&mem));
    println!("{}", mem.dump());

    exit(0);
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_hex(){
        assert_eq!(hex::to_hex8(255), "ff");
        assert_eq!(hex::to_hex32(4294967295), "ffffffff");
        assert_eq!(hex::to_hex0x12(4095), "0xfff");
        assert_eq!(hex::to_hex0x20(0xfffff), "0xfffff");
        assert_eq!(hex::to_hex0x32(4294967295), "0xffffffff");
    }

    #[test]
    fn test_registers(){
        let mut RF = RegisterFile::new();
        RF.set(1, 1234);
        assert_eq!(RF.get(1), 1234);
        RF.set(0, 5678);
        assert_eq!(RF.get(0), 0);
        RF.reset();
        assert_eq!(RF.get(1), 0xf0f0f0f0);
    }

    #[test]
    fn test_memory(){
        let mut mem = Memory::new(64);
        mem.set8(0, 0x12);
        mem.set8(1, 0x34);
        mem.set8(2, 0x56);
        mem.set8(3, 0x78);
        assert_eq!(mem.get8(0), 0x12);
        assert_eq!(mem.get16(0), 0x3412);
        assert_eq!(mem.get32(0), 0x78563412);
    }

    #[test]
    fn test_decode_instructions(){
        assert_eq!(rv32i_decode::decode(0, 0x00000013), "addi    x0,x0,0");
        assert_eq!(rv32i_decode::decode(0, 0x00500113), "addi    x2,x0,5");
        assert_eq!(rv32i_decode::decode(0, 0x00008067), "jalr    x0,0(x1)");
        assert_eq!(rv32i_decode::decode(0, 0x00000073), "ecall");
    }

    #[test]
    fn test_decode_illegal(){
        assert_eq!(rv32i_decode::decode(0, 0x00000000), "ERROR: UNIMPLEMENTED INSTRUCTION");
        assert_eq!(rv32i_decode::decode(0, 0xffffffff), "ERROR: UNIMPLEMENTED INSTRUCTION");
    }


    #[test]
    fn test_decode_allinsn(){
        let mut mem = Memory::new(0xc0);
        mem.load_file("target/debug/input/a4/allinsns.bin".to_string());

        let test_output = format!("{}{}", disassemble(&mem), mem.dump());
        let expected_output = include_str!("../target/debug/input/a4/allinsns-mc0.out");
        let lExpected = expected_output.lines();
        let lTest = test_output.lines();

        for (line_e, line_t) in lExpected.zip(lTest){
            assert_eq!(line_e, line_t);
        }
    }

    #[test]
    fn test_decode_allnsns4(){
        let mut mem = Memory::new(0xc0);
        mem.load_file("target/debug/input/a4/allinsns4.bin".to_string());

        let test_output = format!("{}{}", disassemble(&mem), mem.dump());
        let expected_output = include_str!("../target/debug/input/a4/allinsns4-mc0.out");
        let lExpected = expected_output.lines();
        let lTest = test_output.lines();

        for (line_e, line_t) in lExpected.zip(lTest){
            assert_eq!(line_e, line_t);
        }
    }
}