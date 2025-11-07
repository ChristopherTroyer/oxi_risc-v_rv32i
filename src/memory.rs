/**
 * Simulation of memory with a capacity of a requested amount of bytes
 */
use crate::hex;

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
            let reg = "x".to_owned()+&index.to_string();
            print!("{:>3} ",reg);

            for x in index..index+8{
                print!("{}", hex::to_hex8(self.get8(x as u32)));

                if (x-index)==3{ //space at mid point
                    print!(" ");
                }
                if (x-index)!=7{
                    print!(" ");
                }
            }
            println!();  
        }
    }
}

/* //class memory : public hex
        /**
         * @brief Destroy the memory object
         */
        //~memory();

        /**
         * @brief Checks if passed address is legal
         * 
         * @param addr Address passed
         * @return true if passed address is a valid location in memory
         */
        //bool check_illegal(uint32_t addr) const;


        /**
         * @defgroup getX Get little-endian
         * Read and return value from memory 
         *
         * @param addr Lowest address from which to return a value from.
         * @return returns an appropriately sized int containing the value in addr.
         * @{
         */
        //uint8_t get8(uint32_t addr) const;    ///Get an 8-bit value from memory.
        //uint16_t get16(uint32_t addr) const;  ///Get a 16-bit value from memory.
        //uint32_t get32(uint32_t addr) const;  ///Get a 32-bit value from memory.
        /**@}*/

        /**
         * @defgroup getX Get little-endian, sign-extended
         * Read and return value from memory 
         *
         * @param addr Lowest address from which to return a value from.
         * @return returns an appropriately sized int containing the value in addr.
         * @{
         */
        //int32_t get8_sx(uint32_t addr) const; ///Get an 8-bit value from memory and  sign-extend it.
        //int32_t get16_sx(uint32_t addr) const; ///Get a 16-bit value from memory and  sign-extend it.
        //int32_t get32_sx(uint32_t addr) const; ///Get a 32-bit value from memory and  sign-extend it.
        /**@}*/

        /**
         * @defgroup getX Set little-endian value
         * Write a specified value into specified location in memory.
         *
         * @param addr Lowest address from which to write a value into.
         * @param val  Value to be written into addr
         * @{
         */
        //void set8(uint32_t addr, uint8_t val); //Set an 8-bit value in memory.
        //void set16(uint32_t addr, uint16_t val); //Set a 16-bit value in memory.
        //void set32(uint32_t addr, uint32_t val); //Set a 32-bit value in memory.
        /**@}*/

        /**
         * @brief Opens a file in binary mode and attempts to load it into memory.
         * 
         * @param fname File to be loaded into memory.
         * @return true if file was opened and entirely loaded without issue.
         * @return false if file failed to open or was too big for current amount of memory.
         */
        //bool load_file(const std::string &fname);

    //private:
        //std::vector<uint8_t> mem;   ///Memory buffer */