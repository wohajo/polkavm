#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;
use simplealloc::SimpleAlloc;

#[global_allocator]
static ALLOCATOR: SimpleAlloc<4096> = SimpleAlloc::new();

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe {
        core::arch::asm!("unimp", options(noreturn));
    }
}

#[polkavm_derive::polkavm_import]
extern "C" {
    #[polkavm_import(index = 0)]
    pub fn gas() -> i64;
    #[polkavm_import(index = 1)]
    pub fn lookup(service: u32, hash_ptr: *const u8, out: *mut u8, out_len: u32) -> u32;
    #[polkavm_import(index = 2)]
    pub fn read(service: u32, key_ptr: *const u8, key_len: u32, out: *mut u8, out_len: u32) -> u32;
    #[polkavm_import(index = 3)]
    pub fn write(key_ptr: *const u8, key_len: u32, value: *const u8, value_len: u32) -> u32;
    #[polkavm_import(index = 4)]
    pub fn info(service: u32, out: *mut u8) -> u32;
    #[polkavm_import(index = 5)]
    pub fn empower(m: u32, a: u32, v: u32, o: u32, n: u32) -> u32;
    #[polkavm_import(index = 6)]
    pub fn assign(c: u32, out: *mut u8) -> u32;
    #[polkavm_import(index = 7)]
    pub fn designate(out: *mut u8) -> u32;
    #[polkavm_import(index = 8)]
    pub fn checkpoint() -> u64;
    #[polkavm_import(index = 9)]
    pub fn new(service: u32, hash_ptr: *const u8, out: *mut u8, out_len: u32) -> u32;
    #[polkavm_import(index = 10)]
    pub fn upgrade(out: *const u8, g: u64, m: u64) -> u32;
    #[polkavm_import(index = 11)]
    pub fn transfer(d: u32, a: u64, g: u64, out: *mut u8) -> u32;
    #[polkavm_import(index = 12)]
    pub fn quit(d: u32, a: u64, g: u64, out: *mut u8) -> u32;
    #[polkavm_import(index = 13)]
    pub fn solicit(hash_ptr: *const u8, z: u32) -> u32;
    #[polkavm_import(index = 14)]
    pub fn forget(hash_ptr: *const u8, z: u32) -> u32;
    #[polkavm_import(index = 15)]
    pub fn historical_lookup(service: u32, hash_ptr: *const u8, out: *mut u8, out_len: u32) -> u32;
    #[polkavm_import(index = 16)]
    pub fn import(import_index: u32, out: *mut u8, out_len: u32) -> u32;
    #[polkavm_import(index = 17)]
    pub fn export(out: *const u8, out_len: u32) -> u32;
    #[polkavm_import(index = 18)]
    pub fn machine(out: *const u8, out_len: u32) -> u32;
    #[polkavm_import(index = 19)]
    pub fn peek(out: *const u8, out_len: u32, i: u32) -> u32;
    #[polkavm_import(index = 20)]
    pub fn poke(n: u32, a: u32, b: u32, l: u32) -> u32;
    #[polkavm_import(index = 21)]
    pub fn invoke(n: u32, out: *mut u8) -> u32;
    #[polkavm_import(index = 22)]
    pub fn expunge(n: u32) -> u32;
    #[polkavm_import(index = 99)]
    pub fn blake2b(data: *const u8, data_len: u32, hash_ptr: *mut u8) -> u32;
    #[polkavm_import(index = 100)]
    pub fn blake2s(data: *const u8, data_len: u32, hash_ptr: *mut u8) -> u32;
    #[polkavm_import(index = 101)]
    pub fn ecrecover(h: *const u8, v: *const u8, r: *const u8, s: *const u8, out: *mut u8) -> u32;
    #[polkavm_import(index = 102)]
    pub fn sha2_256(data: *const u8, data_len: u32, hash_ptr: *mut u8) -> u32;
}

#[polkavm_derive::polkavm_export]
extern "C" fn is_authorized() -> u32 {
    0
}

#[polkavm_derive::polkavm_export]
extern "C" fn refine() -> u32 {
    // let address_a = [0u8; 32];
    // let address_b = [0u8; 32];
    // let amount = 0_u32;

    let mut buffer = [0u8; 68];
    let result = unsafe { import(0, buffer.as_mut_ptr(), buffer.len() as u32) };
    
    // Assuming the buffer contains two u8 arrays of length 32 and one u32 value
    let address_a: [u8; 32] = buffer[0..32].try_into().expect("slice with incorrect length");
    let address_b: [u8; 32] = buffer[32..64].try_into().expect("slice with incorrect length");
    let amount = u32::from_le_bytes([buffer[64], buffer[65], buffer[66], buffer[67]]);
    
    // println!("Array 1: {:?}", array1);
    // println!("Array 2: {:?}", array2);
    // println!("Value 3: {}", value3);   

    // read current balance a
    let mut balance_a = [0u8; 4];
    unsafe { read(0, address_a.as_ptr(), address_a.len() as u32, balance_a.as_mut_ptr(), balance_a.len() as u32); }
    let mut balance_a = u32::from_le_bytes(balance_a);

    // read current balance b
    let mut balance_b = [0u8; 4];
    unsafe { read(0, address_b.as_ptr(), address_b.len() as u32, balance_b.as_mut_ptr(), balance_b.len() as u32); }
    let mut balance_b = u32::from_le_bytes(balance_b);


    // transfer
    // TODO [ToDr] check underflow
    balance_a = balance_a - amount;
    // TODO [ToDr] check overflow
    balance_b = balance_b + amount;

    // write new balance_a
    let balance_a = balance_a.to_le_bytes();
    unsafe { write(address_a.as_ptr(), address_a.len() as u32, balance_a.as_ptr(), balance_a.len() as u32); }
    // write new balance_b
    let balance_b = balance_b.to_le_bytes();
    unsafe { write(address_b.as_ptr(), address_b.len() as u32, balance_b.as_ptr(), balance_b.len() as u32); }

    0
}

#[polkavm_derive::polkavm_export]
extern "C" fn accumulate() -> u32 {
    let buffer = [0u8; 12];
    let key = [0u8; 1];

    unsafe {
        write(key.as_ptr(), 1, buffer.as_ptr(), buffer.len() as u32);
    }

    0
}

#[polkavm_derive::polkavm_export]
extern "C" fn on_transfer() -> u32 {
    0
}