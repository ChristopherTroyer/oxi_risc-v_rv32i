use std::process::exit;
use clap::Parser;


#[derive(Parser)]
struct Cli{
    pattern: String,
    path: std::path::PathBuf,
}

fn usage(){
    println!("Usage: rv32i [-dirz] [-m hex-mem-size] [-l execution-limit] infile");
    println!("    -d Show memory disassembly before running simulation.");
    println!("    -i Print instructions during execution.");
    println!("    -r show dump of hart before each instruction is simulated.");
    println!("    -z show dump of hart status and memory after simulation is haulted.");
    println!("    -m specify memory size (default = 0x100)");
    println!("    -l maximum limit of instructions to execute. (default = 0 (no limit))");
    exit(1)
}

//fn disassemble(){
//    for 
//}


fn main(){
    let args = Cli::parse();
    //dbg!(&args);
    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
    //if args.len() == 1{
    //    usage();
    //}

    //TODO: remove mut after getting these values from arguments, default to these if none
    let _memory_limit: u32 = 0x100;
    let _exec_limit: u64 = 0;



    exit(0);
}
