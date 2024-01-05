fn main() -> miette::Result<()> {
    let dst = cmake::Config::new("zxing-cpp")
        .define("BUILD_EXAMPLES", "OFF")
        .build();

    println!("cargo:rustc-link-search=native={}\\lib", dst.display());
    println!("cargo:rustc-link-lib=static=ZXing");

    let path = std::path::PathBuf::from("zxing-cpp/core/src");
    let mut b = autocxx_build::Builder::new("src/lib.rs", &[&path])
        .extra_clang_args(&["-std=c++17", "-Wc++17-extensions"])
        .build()?;
    b.flag_if_supported("-std:c++17")
        .flag_if_supported("-Wc++17-extensions")
        .compile("zxing-cpp2rs");

    println!("cargo:rerun-if-changed=src/lib.rs");

    Ok(())
}
