use libmotor_sys::*;

fn main() {
  let x = aes_encrypt_with_key(String::from(""), String::from(""));
  println!("{}", x);
}