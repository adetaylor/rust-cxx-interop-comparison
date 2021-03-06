#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[no_mangle]
pub extern "C" fn validate_house_number(number: u64) {
    if number < 10000 {
        panic!("Unrealistically small California house number.");
    }
}

fn main() {
    let input_address = &std::env::args().collect::<Vec<String>>()[1];
    let ministring_address = MiniString {
        len: input_address.len() as u64,
        data: input_address.as_bytes().as_ptr() as *const i8,
    };
    let mut parsed = unsafe { parse_address(&ministring_address) };
    let str_street: &[u8] = unsafe { std::slice::from_raw_parts(parsed.street.data as *const u8, parsed.street.len as usize) };
    let mut str_street_vec = Vec::new();
    str_street_vec.extend(str_street);
    let street = unsafe { String::from_utf8_unchecked(str_street_vec) };
    unsafe { free_mini_string(&mut parsed.street) };
    println!("Street is: {}. Number is {}.", street, parsed.house_number);
}
