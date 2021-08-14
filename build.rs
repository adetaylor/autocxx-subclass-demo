fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/test.cc")
        .flag_if_supported("-std=c++14")
        .compile("autocxx-subclass-demo");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/test.cc");
    println!("cargo:rerun-if-changed=include/test.h");
}
