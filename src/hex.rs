///Formats and returns a 8-bit hex value.
pub fn to_hex8(i:u8) -> String{
    let h_str: String = format!("{:02X}",i);
    h_str
}

///Formats and returns a 32-bit hex value.
pub fn to_hex32(i:u32) -> String{
    let h_str: String = format!("{:08X}",i);
    h_str
}

///Formats and returns a 32-bit hex value starting with 0x.
pub fn to_hex0x32(i:u32) -> String{
    let h_str: String = format!("{:#08X}",i);
    h_str
}

///Formats and returns a 20-bit hex value starting with 0x.
pub fn to_hex0x20(i:u32) -> String{ //v this probably formats incorrectly right now, fix later
    let h_str: String = format!("{:#05X}",i);
    h_str
}

///Formats and returns a 12-bit hex value starting with 0x.
pub fn to_hex0x12(i:u32) -> String{ //v this probably formats incorrectly right now, fix later
    let h_str: String = format!("{:#03X}",i);
    h_str
}