/**
 * CPU register simulator
 */
pub struct RegisterFile{
    regs: Vec<u32>,
}

impl RegisterFile {
    pub fn new() -> Self {
        let mut regs = Vec::new();
        regs.resize(32,0);
        //reset();

        RegisterFile { regs }
    }

    fn init_regs(&mut self){
        self.regs.resize(32, 0);
        self.reset();
    }

    pub fn reset(&mut self){
        self.set(0,0);

        for i in 0..self.regs.len() as i32 {
            self.set(i as u32,0xf0f0f0f0);
        }
    }

    pub fn set(&mut self, reg:u32, val:u32){
        if reg == 0 {
             return;
        }
        self.regs[reg as usize] = val;
    }

    pub fn get(&self, reg:u32) -> u32{
        self.regs[reg as usize]
    }

    pub fn dump(&self, hdr:String){
        println!("your header here: {}", hdr); //placeholder
    }
    
}