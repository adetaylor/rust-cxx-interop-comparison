fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/demo.cc")
        .flag_if_supported("-std=c++14")
        .compile("cxx-demo");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/demo.cc");
    println!("cargo:rerun-if-changed=include/demo.h");
}
