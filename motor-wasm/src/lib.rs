pub use wasm_bindgen_rayon::init_thread_pool;

use motor_core::create_account as _create_account;
use prs::account::CreateAccountRequest;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn create_account() {
    println!("Hello world");
    match _create_account(CreateAccountRequest::default()) {
        Ok(res) => log(&format!("res: {}", res)),
        Err(e) => log(&format!("err: {}", e)),
    };
}
