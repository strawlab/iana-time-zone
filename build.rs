fn main() {
    #[cfg(target_os = "haiku")]
    let _: () = {
        cxx_build::bridge("src/tz_haiku.rs")
            .file("src/impl_haiku.cc")
            .flag_if_supported("-std=c++11")
            .compile("tz_haiku");

        println!("cargo:return-if-changed=include/impl_haiku.h");
        println!("cargo:return-if-changed=src/impl_haiku.cc");
        println!("cargo:return-if-changed=src/tz_haiku.rs");
        println!("cargo:rustc-link-lib=be");
    };
}
