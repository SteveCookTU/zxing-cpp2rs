fn main() -> miette::Result<()> {
    let mut config = cmake::Config::new("zxing-cpp");

    config.define("BUILD_EXAMPLES", "OFF");

    #[cfg(target_os = "windows")]
    config.define("CMAKE_MSVC_RUNTIME_LIBRARY", "MultiThreadedDLL");

    let dst = config.build();

    println!("cargo:rustc-link-search=native={}\\lib", dst.display());
    println!("cargo:rustc-link-lib=static=ZXing");

    let path = std::path::PathBuf::from("zxing-cpp/core/src");
    let mut b = autocxx_build::Builder::new("src/lib.rs", &[&path])
        .extra_clang_args(&["-std=c++17", "-Wc++17-extensions"])
        .build()?;

    #[cfg(target_os = "windows")]
    b.flag_if_supported("-std:c++17");

    #[cfg(not(target_os = "windows"))]
    b.flag_if_supported("-std=c++17");

    b.flag_if_supported("-Wc++17-extensions")
        .compile("zxing-cpp2rs");

    println!("cargo:rerun-if-changed=src/lib.rs");

    Ok(())
}
