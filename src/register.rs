/**
 * CPU register simulator
 */
use crate::hex;
pub struct RegisterFile{
    regs: Vec<u32>,
}

impl RegisterFile {
    pub fn new() -> RegisterFile {
        let mut regs = Vec::new();
        let mut rf = RegisterFile { regs };
        rf.init_regs();
        rf
    }
    /**
     * Resizes the reg vector and resets it.
     */
    fn init_regs(&mut self){
        self.regs.resize(32, 0);
        self.reset();

        //DEBUG V
        //self.dump("DUMP".to_string());
    }

    /**
     * Ensures register is set to 0 and sets the rest of the registers to 0xf0f0f0f0.
     */
    pub fn reset(&mut self){
        self.set(0,0);

        for i in 0..self.regs.len() as i32 {
            self.set(i as u32,0xf0f0f0f0);
        }
    }
    /**
     * Sets the value of a register.
     * reg:u32 register to set.
     * val:u32 value to set register to.
     */
    pub fn set(&mut self, reg:u32, val:u32){
        if reg == 0 {
             return;
        }
        self.regs[reg as usize] = val;
    }
    /**
     * Gets a value from a register..
     * r:u32 register to get value from.
     * Returns value:u32 in passed register.
     */
    pub fn get(&self, reg:u32) -> u32{
        self.regs[reg as usize]
    }
    /**
     * Prints contents of registers with optional header.
     * hdr:String string to append to lines of dump.
     */
    pub fn dump(&self, hdr:String){
        for index in (0..self.regs.len()).step_by(8) {
            let reg = "x".to_owned()+&index.to_string();
            print!("{:>3} ",reg);

            for x in index..index+8{
                print!("{}", hex::to_hex32(self.get(x as u32)));

                if ((x-index)==3){
                    print!(" ");
                }
                if ((x-index)!=7){
                    print!(" ");
                }
            }
            if hdr.len() > 0 {
                println!(" {}",hdr);
            }
            else{
                println!();
            }
        }
    }
    
}