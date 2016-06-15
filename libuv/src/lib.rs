extern crate libuv_sys;
extern crate libc;

use std::ffi::CStr;

pub fn version_hex() -> u32 {
    unsafe { libuv_sys::uv_version() as u32 }
}

pub fn version_string() -> &'static str {
    unsafe { CStr::from_ptr(libuv_sys::uv_version_string()).to_str().unwrap() }
}

pub fn strerror(err: u32) -> &'static str {
    unsafe { CStr::from_ptr(libuv_sys::uv_strerror(err as libc::c_int)).to_str().unwrap() }
}

pub fn err_name(err: u32) -> &'static str {
    unsafe { CStr::from_ptr(libuv_sys::uv_err_name(err as libc::c_int)).to_str().unwrap() }
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum RunMode {
    Default,
    Once,
    NoWait,
}

impl std::convert::From<libuv_sys::uv_run_mode> for RunMode {
    fn from(v: libuv_sys::uv_run_mode) -> RunMode {
        match v {
            libuv_sys::UV_RUN_DEFAULT => RunMode::Default,
            libuv_sys::UV_RUN_NOWAIT => RunMode::NoWait,
            libuv_sys::UV_RUN_ONCE => RunMode::Once,
        }
    }
}

impl std::convert::Into<libuv_sys::uv_run_mode> for RunMode {
    fn into(self) -> libuv_sys::uv_run_mode {
        match self {
            RunMode::Default => libuv_sys::UV_RUN_DEFAULT,
            RunMode::NoWait => libuv_sys::UV_RUN_NOWAIT,
            RunMode::Once => libuv_sys::UV_RUN_ONCE,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_version_hex() {
        assert_eq!(version_hex(), 0x10901);
    }

    #[test]
    fn test_version_string() {
        assert_eq!(version_string(), "1.9.1");
    }
}
