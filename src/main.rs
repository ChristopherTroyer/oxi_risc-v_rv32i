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
    #[arg(short = 'm',long,default_value_t = 0x100, help="Specify memory size.")]
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
fn disassemble(mem:Memory){
    for i in (0..mem.get_size()).step_by(4){
        print!("{:>8}: ", i);
        print!("{:>8}", mem.get32(i));
        //render instruction via decode function
        println!("");
    }
}

fn main(){
    let args = Cli::parse();
    //let rf = RegisterFile::new();
    let mut mem = Memory::new(args.memory_limit);
    //let filename = args.input_file;
    //println!("Loading file: {:?}", filename);

    //MEM.dump();
    mem.load_file("target/debug/input/align.bin".to_string());
    //mem.dump();
    disassemble(mem);

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
}