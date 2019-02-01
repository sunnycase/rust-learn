#![no_std]
#![no_main]

extern crate uefi;
extern crate uefi_services;

use uefi::prelude::*;

#[no_mangle]
pub extern "win64" fn uefi_start(image: uefi::Handle, st: &'static SystemTable<Boot>) -> ! {
    uefi_services::init(&st).expect_success("Failed to initialize utilities");

    st.stdout()
        .reset(false)
        .expect_success("Failed to reset stdout");
    loop {}
}
