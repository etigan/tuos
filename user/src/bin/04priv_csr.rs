#![no_std]
#![no_main]

use user_lib::println;

use riscv::register::sstatus::{self, SPP};

#[no_mangle]
fn main() -> i32 {
    println!("Try to access privileged CSR in U Mode");
    println!("Kernel should kill this application!");
    unsafe {
        sstatus::set_spp(SPP::User);
    }
    0
}
