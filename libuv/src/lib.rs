extern crate libuv_sys;
extern crate libc;

use std::ffi::CStr;

pub fn version_hex() -> u32 {
    unsafe { libuv_sys::uv_version() as u32 }
}

pub fn version_string() -> &'static str {
    unsafe { CStr::from_ptr(libuv_sys::uv_version_string()).to_str().unwrap() }
}

pub fn strerror(err: i32) -> &'static str {
    unsafe { CStr::from_ptr(libuv_sys::uv_strerror(err as libc::c_int)).to_str().unwrap() }
}

pub fn err_name(err: i32) -> &'static str {
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

pub type Result<T> = std::result::Result<T, i32>;

// loop wrapper
// TODO : add support for data...
pub struct Loop {
    uv_loop: *mut libuv_sys::uv_loop_t,
}

impl Loop {
    pub fn new() -> Result<Loop> {
        unsafe {
            let q = libc::malloc(libuv_sys::uv_loop_size()) as *mut libuv_sys::uv_loop_t;
            if q == std::ptr::null_mut::<libuv_sys::uv_loop_t>() {
                panic!("Failed to allocate memory");
            }

            let u = libuv_sys::uv_loop_init(q) as i32;

            if u != 0 {
                libc::free(q as *mut libc::c_void);
                return Err(u);
            }

            let mut l = Loop { uv_loop: q };
            (*q).data = (&mut l as *mut Loop) as *mut libc::c_void;
            Ok(l)
        }
    }

    pub unsafe fn to_uv(&self) -> *mut libuv_sys::uv_loop_t {
        self.uv_loop
    }

    pub fn run(&self, mode: RunMode) -> Result<()> {
        unsafe {
            let u = libuv_sys::uv_run(self.uv_loop, mode.into());

            if u != 0 { Err(u) } else { Ok(()) }
        }
    }

    pub fn stop(&self) {
        unsafe {
            libuv_sys::uv_stop(self.uv_loop);
        }
    }

    pub fn is_alive(&self) -> bool {
        unsafe {
            0 != libuv_sys::uv_loop_alive(self.uv_loop)
        }
    }
}

impl Drop for Loop {
    fn drop(&mut self) {
        unsafe {
            (*self.uv_loop).data = std::ptr::null_mut();
            libuv_sys::uv_loop_close(self.uv_loop);
            libc::free(self.uv_loop as *mut libc::c_void);
            self.uv_loop = std::ptr::null_mut();
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
