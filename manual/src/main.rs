pub type size_t = ::std::os::raw::c_ulong;

#[repr(C)]
pub struct MiniString {
    pub data: *const ::std::os::raw::c_char,
    pub len: size_t,
}

#[repr(C)]
pub struct Address {
    pub street: MiniString,
    pub house_number: u64,
}

extern "C" {
    pub fn free_mini_string(str: *mut MiniString);
}
extern "C" {
    pub fn parse_address(unparsed_address: *const MiniString) -> Address;
}       

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
