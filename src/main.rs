use std::process::exit;
use clap::Parser;
use std::path::PathBuf;

mod hex;

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
    //input_file: PathBuf,
}

fn main(){
    let args = Cli::parse();
    dbg!(&args);

    dbg!(&args.memory_limit);
    dbg!(&args.exec_limit);

    dbg!(hex::to_hex32(args.memory_limit));

    exit(0);
}
