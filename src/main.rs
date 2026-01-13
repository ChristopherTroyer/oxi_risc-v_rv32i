// RISC-V Simulator Rust implementation

use std::process::exit;
use clap::Parser;
//use std::path::PathBuf;

use crate::register::RegisterFile;
use crate::memory::Memory;
use crate::rv32i_hart::Rv32iHart;

mod hex;
mod register;
mod memory;
mod rv32i_decode;
mod rv32i_hart;

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
    let mut mem = Memory::new(0x40);
    //let filename = args.input_file;
    //println!("Loading file: {:?}", filename);

    mem.load_file("target/debug/input/a4/tinyprog.bin".to_string());

    println!("{}", disassemble(&mem));
    println!("{}", mem.dump());

    //test hart
    let mut hart = Rv32iHart::new(mem, RegisterFile::new());
    &hart.reset();
    let id = hart.mhartid;
    &hart.tick(format!("hart {}:", id).to_string());


    exit(0);
}

#[cfg(test)]
mod tests{
    use super::*;
    use std::result;

    fn string_lines_equal(s1: &str, s2: &str, tested_file: &str){
        let lines1 = s1.lines();
        let lines2 = s2.lines();

        for (line1, line2) in lines1.zip(lines2) {
            assert!(line1 == line2, "Lines in '{}' do not match:\n'{}'\n'{} <--- Expected'", tested_file, line1, line2);
        }
    }

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
        assert_eq!(RF.get(1), 0xf0f0f0f0u32 as i32);
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

        string_lines_equal(&test_output, &expected_output, "allinsns.bin");
    }

    #[test]
    fn test_decode_allnsns4(){
        let mut mem = Memory::new(0xc0);
        mem.load_file("target/debug/input/a4/allinsns4.bin".to_string());

        let test_output = format!("{}{}", disassemble(&mem), mem.dump());
        let expected_output = include_str!("../target/debug/input/a4/allinsns4-mc0.out");

        string_lines_equal(&test_output, &expected_output, "allinsns4.bin");
    }

    #[test]
    fn test_decode_badhex(){
        let mut mem = Memory::new(0x100);
        mem.load_file("target/debug/input/a4/badhex.bin".to_string());

        let test_output = format!("{}{}", disassemble(&mem), mem.dump());
        let expected_output = include_str!("../target/debug/input/a4/badhex-m100.out");

        string_lines_equal(&test_output, &expected_output, "badhex.bin");
    }

    #[test]
    fn test_decode_li(){
        let mut mem = Memory::new(0x1);
        mem.load_file("target/debug/input/a4/li.bin".to_string());

        let test_output = format!("{}{}", disassemble(&mem), mem.dump());
        let expected_output = include_str!("../target/debug/input/a4/li-m1.out");

        string_lines_equal(&test_output, &expected_output, "li.bin");
    }

    #[test]
    fn test_decode_master(){
        let mut mem = Memory::new(0x100);
        mem.load_file("target/debug/input/a4/master.bin".to_string());

        let test_output = format!("{}{}", disassemble(&mem), mem.dump());
        let expected_output = include_str!("../target/debug/input/a4/master.out");

        string_lines_equal(&test_output, &expected_output, "master.bin");
    }

    #[test]
    fn test_decode_pcrel(){
        let mut mem = Memory::new(0x1000);
        mem.load_file("target/debug/input/a4/pcrel.bin".to_string());

        let test_output = format!("{}{}", disassemble(&mem), mem.dump());
        let expected_output = include_str!("../target/debug/input/a4/pcrel-m1000.out");

        string_lines_equal(&test_output, &expected_output, "pcrel.bin");
    }

    #[test]
    fn test_decode_reladdr(){
        let mut mem = Memory::new(0x20);
        mem.load_file("target/debug/input/a4/reladdr.bin".to_string());

        let test_output = format!("{}{}", disassemble(&mem), mem.dump());
        let expected_output = include_str!("../target/debug/input/a4/reladdr-m20.out");

        string_lines_equal(&test_output, &expected_output, "reladdr.bin");
    }

    #[test]
    fn test_decode_sieve(){
        let mut mem = Memory::new(0x33e68);
        mem.load_file("target/debug/input/a4/sieve.bin".to_string());

        let test_output = format!("{}{}", disassemble(&mem), mem.dump());
        let expected_output = include_str!("../target/debug/input/a4/sieve-m33e68.out");

        string_lines_equal(&test_output, &expected_output, "sieve.bin");
    }

    #[test]
    fn test_decode_small(){
        let mut mem = Memory::new(0xa0);
        mem.load_file("target/debug/input/a4/small.bin".to_string());

        let test_output = format!("{}{}", disassemble(&mem), mem.dump());
        let expected_output = include_str!("../target/debug/input/a4/small-ma0.out");

        string_lines_equal(&test_output, &expected_output, "small.bin");
    }

    #[test]
    fn test_decode_tinyprog(){
        let mut mem = Memory::new(0x40);
        mem.load_file("target/debug/input/a4/tinyprog.bin".to_string());

        let test_output = format!("{}{}", disassemble(&mem), mem.dump());
        let expected_output = include_str!("../target/debug/input/a4/tinyprog-m40.out");

        string_lines_equal(&test_output, &expected_output, "tinyprog.bin");
    }

    #[test]
    fn test_decode_torture(){
        let mut mem = Memory::new(0x410);
        mem.load_file("target/debug/input/a4/torture.bin".to_string());

        let test_output = format!("{}{}", disassemble(&mem), mem.dump());
        let expected_output = include_str!("../target/debug/input/a4/torture-m410.out");

        string_lines_equal(&test_output, &expected_output, "torture.bin");
    }

    #[test]
    fn test_decode_torture5(){
        let mut mem = Memory::new(0x500);
        mem.load_file("target/debug/input/a4/torture5.bin".to_string());

        let test_output = format!("{}{}", disassemble(&mem), mem.dump());
        let expected_output = include_str!("../target/debug/input/a4/torture5-m500.out");

        string_lines_equal(&test_output, &expected_output, "torture5.bin");
    }
}