use crate::register;

/**
 * CPU register simulator
 */
pub struct RegisterFile{
    regs: Vec<i32>,
}

impl RegisterFile {
    pub fn new() -> RegisterFile {
        let mut regs = Vec::new();
        RegisterFile::init_regs();
        RegisterFile { regs }
    }

    fn init_regs(){
        regs.resize(32,0);
        reset();
    }

    pub fn reset(){
        Self::set(r,0,0);

        for i in 0..regs.len() as i32 {
            Self::set(r,i as u32,0xf0f0f0f0);
        }

    }

    pub fn set(reg:u32, val:i32){
        if reg == 0 {
             return;
        }
        regs[reg as usize] = val;

    }

    pub fn get(reg:u32) -> i32{
        regs[reg as usize]
    }

    pub fn dump(hdr:String){
        
    }
    
}
