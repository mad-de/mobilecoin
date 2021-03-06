// Copyright (c) 2018-2020 MobileCoin Inc.

// Mobilenode-side support for sgx_panic crate impl
//
// If the enclave panics, it will pass us a message, which we write to stderr
// The enclave is expected to call rsgx_abort itself after this

use std::str;

#[no_mangle]
pub extern "C" fn report_panic_message(msg: *const u8, msg_len: usize) {
    let panic_msg_bytes = unsafe { std::slice::from_raw_parts(msg, msg_len) };

    match str::from_utf8(panic_msg_bytes) {
        Ok(v) => eprintln!("Enclave panic:\n{}\n", v),
        Err(e) => eprintln!(
            "Enclave panic message contained invalid utf8:\n{}\n{:?}",
            e, panic_msg_bytes
        ),
    }
}
