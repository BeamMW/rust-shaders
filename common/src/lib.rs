#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}

include!("bvm_bindings.rs");
include!("contract_sid.rs");

use crate::root::*;

#[repr(C, packed(1))]
pub struct CtorParams {}

#[repr(C, packed(1))]
pub struct DtorParams {}

#[repr(C, packed(1))]
pub struct HelloWorld {}

impl CtorParams {
    pub const METHOD: u32 = 0;
}

impl DtorParams {
    pub const METHOD: u32 = 1;
}

impl HelloWorld {
    pub const METHOD: u32 = 2;
}

// Vault-specific types
#[repr(C, packed(1))]
pub struct Key {
    pub account: PubKey,
    pub aid: AssetID,
}

#[repr(C, packed(1))]
pub struct Request {
    pub key: Key,
    pub amount: Amount,
}

#[repr(C, packed(1))]
pub struct Deposit {
    pub request: Request,
}

#[repr(C, packed(1))]
pub struct Withdraw {
    pub request: Request,
}

impl Deposit {
    pub const METHOD: u32 = 2;
}

impl Withdraw {
    pub const METHOD: u32 = 3;
}