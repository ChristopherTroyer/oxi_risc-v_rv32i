/**
 * Simulation of memory with a capacity of a requested amount of bytes
 */
use crate::hex;
use std::fs;
use std::env;
use std::path;

pub struct Memory{
    mem: Vec<u8>,
}

impl Memory {
    /**
    * @param s Amount of memory to create and populate in bytes 
    * @note s will be rounded up to nearest multiple of 16
    */
    pub fn new(mut s:u32) -> Memory {
        let mut mem = Vec::new();
        s = (s+15)&0xfffffff0;
        mem.resize(s.try_into().unwrap(), 0xa5);
        let mut memory = Memory { mem };
        memory
    }

    pub fn check_illegal(&self, addr:u32) -> bool{
        if addr >= self.get_size(){
            println!("WARNING: Address out of range: {}", hex::to_hex0x32(addr));
        }
        addr >= self.get_size()
    }

    /**
      @brief Returns amount of memory allotted as a 32-bit int
    */
    pub fn get_size(&self) -> u32{
        self.mem.len() as u32
    }

    /**
    * @defgroup getX Get little-endian
    * Read and return value from memory 
    *
    * @param addr Lowest address from which to return a value from.
    * @return returns an appropriately sized int containing the value in addr.
    * @{
    */
    pub fn get8(&self, addr:u32) -> u8{ //Get an 8-bit value from memory.
        if !self.check_illegal(addr) {
            self.mem[addr as usize]
        }
        else{
            0
        }
    }
    pub fn get16(&self, addr:u32) -> u16{ //Get an 16-bit value from memory.
        let x = self.get8(addr) as u16;
        let y = self.get8(addr+1) as u16;

        x|y<<8
    }
    pub fn get32(&self, addr:u32) -> u32{ //Get an 16-bit value from memory.
        let x = self.get16(addr) as u32;
        let y = self.get16(addr+2) as u32;

        x|y<<16
    }
    /**@}*/

    /**
     * getX Get little-endian, sign-extended
     * Read and return value from memory 
     *
     * addr Lowest address from which to return a value from.
     * returns an appropriately sized int containing the value in addr.
     * @{
     */
    pub fn get8_sx(&self, addr:u32) -> u32{
        let x: u32 = self.get8(addr) as u32;
        x | if x & 0x00000080 != 0 { 0xffffff00 } else { 0 }
    }
    pub fn get16_sx(&self, addr:u32) -> u32{
        let x: u32 = self.get16(addr) as u32;
        x | if x & 0x00008000 != 0 { 0xffff0000 } else { 0 }
    }
    pub fn get32_sx(&self, addr:u32) -> u32{
        self.get32(addr)
    }
    /**@}*/

    /**
    * getX Set little-endian value
    * Write a specified value into specified location in memory.
    *
    * addr Lowest address from which to write a value into.
    * val  Value to be written into addr
    * @{
    */
    pub fn set8(& mut self, addr:u32, val:u8){
        if !self.check_illegal(addr) {
            let v = val.to_le_bytes();
            self.mem[addr as usize] = v[0];
        }
    }
    pub fn set16(& mut self, addr:u32, val:u16){
        if !self.check_illegal(addr) {
            let v = val.to_le_bytes();
            println!("{:?}", v);
            self.mem[addr as usize] = v[0];
            self.mem[(addr+1) as usize] = v[1];
        }
        //self.set8(addr, val);
        //self.set8(addr+2, val as u8 >> 8);
        //self.set8(addr, val as u8);
    }
    pub fn set32(& mut self, addr:u32, val:u32){
        if !self.check_illegal(addr) {
            let v = val.to_le_bytes();
            println!("{:?}", v);
            self.mem[addr as usize] = v[0];
            self.mem[(addr+1) as usize] = v[1];
            self.mem[(addr+2) as usize] = v[2];
            self.mem[(addr+3) as usize] = v[3];
        }
    }
    /**@}*/

    /**
    * Prints entire contents of memory as both hex and ASCII
    * 
    */
    pub fn dump(&self){
        for index in (0..self.get_size()).step_by(16) {
            print!("{:>8}: ",hex::to_hex32(index));

            for x in index..index+16{
                print!("{}", hex::to_hex8(self.get8(x as u32)));

                if (x-index)==7{ //space at mid point
                    print!("  ");
                }
                if (x-index)!=7{
                    print!(" ");
                }
            }

            print!("*");

            for x in index..index+16{ //ascii printable HEX 21 to 7E
                let num = self.get8(x) as char;
                if num.is_ascii_graphic()  {
                    print!("{}", num as char);
                }
                else{
                    print!(".");
                }
            }

            println!("*");  
        }
    }

    pub fn load_file(& mut self, fname: String) -> bool{
        let mut load_state = true; //Return value, only changes upon failure

        let file = fs::read(&fname);
        match file{
            Ok(content) => {
                //let real_file = file.unwrap();
                //println!("begin load"); //TODO
                if self.check_illegal(content.len() as u32){
                    println!("Program too big.");
                    load_state = false;
                }
                else {
                    for byte in 0..content.len(){
                        //println!("{}", byte);
                        self.mem[byte] = content[byte];
                }
                }
            },
            Err(error) => {
                load_state = false;
                println!("Can't open file {} for reading, error: {}", &fname, error)
            },
        }

        load_state
    }
}
