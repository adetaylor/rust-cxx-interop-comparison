use autocxx::include_cxx;

include_cxx!(Header("demo.h"), Allow("Address"), Allow("parse_address"),);

#[cxx::bridge]
mod ffi2 {
    extern "Rust" {
        fn validate_house_number(number: u64);
    }
}

fn validate_house_number(number: u64) {
    if number < 10000 {
        panic!("Unrealistically small California house number.");
    }
}

fn main() {
    let input_address = &std::env::args().collect::<Vec<String>>()[1];
    let parsed = ffi::cxxbridge::parse_address(input_address);
    println!("Street is: {}. Number is {}.", parsed.as_ref().unwrap().get_street(), parsed.as_ref().unwrap().get_house_number());
}
