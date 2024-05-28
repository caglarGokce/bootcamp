use borsh::{BorshDeserialize, BorshSerialize};


#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct Toplama{
    pub sayi:u16,
    pub random_number:u64,//8 bytes
}


#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct InitPda{
    pub bump:u8,// bytes 0-255
    pub sayi2:u16, //2byte 255 * 255
    pub sayi3:u32, //4 byte 
    pub lamports:u64,
    pub no:u8,
}
