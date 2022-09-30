fn main() {
    cxx_build::bridge("src/lib.rs")
        .file("src/implementation.cc")
        .flag_if_supported("-std=c++11")
        .compile("tz_haiku");

    println!("cargo:return-if-changed=include/interface.h");
    println!("cargo:return-if-changed=src/implementation.cc");
    println!("cargo:return-if-changed=src/lib.rs");
    println!("cargo:rustc-link-lib=be");
}
