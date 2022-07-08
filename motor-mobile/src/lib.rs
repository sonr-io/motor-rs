// use std::os::raw::{c_uint};
use motor_core::create_account as _create_account;
use prs::account::CreateAccountRequest;
use protobuf::Message;

#[no_mangle]
pub extern "C" fn create_account(req: *mut u8, len: usize) -> *mut u8 {
    let r = unsafe { Vec::from_raw_parts(req, len, len) };
    let request: CreateAccountRequest = match Message::parse_from_bytes(r.as_slice()) {
        Err(_) => CreateAccountRequest::default(),
        Ok(val) => val,
    };

    let result = _create_account(request);
    match result.write_to_bytes() {
        Ok(mut v) => v.as_mut_ptr(),
        Err(_) => 0 as *mut u8,
    }
}
