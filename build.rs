fn main() {
    let mut config = cmake::Config::new("mcl");
    config
        .define("MCL_STATIC_LIB", "ON")
        .define("MCL_STANDALONE", "ON");

    // On non-x86_64 targets mcl compiles LLVM IR (*.ll) directly with clang++,
    // so it must be specified explicitly; otherwise CMake aborts with
    // "requiring clang++".
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    if target_arch != "x86_64" {
        config.define("CMAKE_CXX_COMPILER", "clang++");
    }

    let dst = config.build();
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=mcl");
}
