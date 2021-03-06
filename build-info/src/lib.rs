// Copyright (c) 2018-2020 MobileCoin Inc.

#![no_std]

include!(concat!(env!("OUT_DIR"), "/build_info_generated.rs"));

// Write a report as a json blob containing all the info
// For example pass a string to this, or any other implementor of core::fmt::Write
use core::fmt::{Result, Write};
pub fn write_report(output: &mut dyn Write) -> Result {
    write!(
        output,
        r##"{{ "GIT_COMMIT": "{}", "PROFILE": "{}", "DEBUG": "{}", "OPT_LEVEL": "{}", "DEBUG_ASSERTIONS": "{}", "TARGET_ARCH": "{}", "TARGET_OS": "{}", "TARGET_FEATURE": "{}", "RUSTFLAGS": "{}", "SGX_MODE": "{}", "IAS_MODE": "{}" }}"##,
        GIT_COMMIT,
        PROFILE,
        DEBUG,
        OPT_LEVEL,
        DEBUG_ASSERTIONS,
        TARGET_ARCH,
        TARGET_OS,
        TARGET_FEATURE,
        RUSTFLAGS,
        SGX_MODE,
        IAS_MODE,
    )
}
