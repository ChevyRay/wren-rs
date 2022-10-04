use std::ffi::{c_char, c_int, CStr, CString};
use std::mem::MaybeUninit;
use wren_sys::*;

pub struct Machine {
    vm: *mut WrenVM,
}

impl Machine {
    pub fn new() -> Self {
        let mut config = unsafe {
            let mut config: MaybeUninit<WrenConfiguration> = MaybeUninit::uninit();
            wrenInitConfiguration(config.as_mut_ptr());
            config.assume_init()
        };
        config.write_fn = write_fn;
        config.error_fn = error_fn;
        config.load_module_fn = load_module_fn;

        todo!()
    }
}

extern "C" fn load_module_fn(_vm: *mut WrenVM, name: *const c_char) -> WrenLoadModuleResult {
    let name = unsafe { CStr::from_ptr(name) };
    let path = format!("{}.wren", name.to_str().unwrap());
    let str = std::fs::read_to_string(&path).unwrap();
    let str = CString::new(str).unwrap();
    WrenLoadModuleResult {
        source: str.into_raw(),
        on_complete: load_module_complete_fn,
        user_data: std::ptr::null_mut(),
    }
}

extern "C" fn load_module_complete_fn(
    _vm: *mut WrenVM,
    _name: *const c_char,
    result: WrenLoadModuleResult,
) {
    drop(unsafe { CString::from_raw(result.source) });
}

extern "C" fn write_fn(_vm: *mut WrenVM, text: *const c_char) {
    let str = unsafe { CStr::from_ptr(text) };
    println!("{}", str.to_str().unwrap());
}

extern "C" fn error_fn(
    _vm: *mut WrenVM,
    ty: WrenErrorType,
    module: *const c_char,
    line: c_int,
    message: *const c_char,
) {
    let module = unsafe { CStr::from_ptr(module) };
    let message = unsafe { CStr::from_ptr(message) };
    // TODO: handle errors properly (via wren user data)
    panic!(
        "{} error in module `{}` line {}: {}",
        match ty {
            WrenErrorType::Compile => "Compile",
            WrenErrorType::Runtime => "Runtime",
            WrenErrorType::StackTrace => "Stack trace",
        },
        module.to_str().unwrap(),
        line,
        message.to_str().unwrap(),
    );
}
