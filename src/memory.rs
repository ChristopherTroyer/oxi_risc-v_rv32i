/**
 * Simulation of memory with a capacity of a requested amount of bytes
 */
use crate::hex;

struct Memory{
    mem: Vec<u8>,
}

impl Memory {
    pub fn new(mut s:u32) -> Memory {
        let mut mem = Vec::new();
        s = (s+15)&0xfffffff0;
        mem.resize(s.try_into().unwrap(), 0xa5);
        let mut memory = Memory { mem };
        memory
    }
}

/* //class memory : public hex
    //public:
        /**
         * @param s Amount of memory to create and populate in bytes 
         * @note s will be rounded up to nearest multiple of 16
         */
        //memory(uint32_t s);
        
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
         * @brief Returns amount of memory allotted as a 32-bit int
         */
        //uint32_t get_size() const;

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
         * @brief Prints entire contents of memory as both hex and ASCII
         * 
         */
        //void dump() const;

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