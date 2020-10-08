#[cxx::bridge]
mod ffi {
    struct Address {
        house_number: u64,
        street: UniquePtr<CxxString>,
    }

    extern "C++" {
        include!("cxx-demo/include/demo.h");
        fn parse_address(input: &str) -> Address;
    }
}

fn main() {
    let input_address = &std::env::args().collect::<Vec<String>>()[1];
    let parsed = ffi::parse_address(input_address);
    println!("Street is: {}. Number is {}.", parsed.street, parsed.house_number);
}