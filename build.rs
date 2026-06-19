fn main() {
    let dst = cmake::Config::new("mcl")
        .define("MCL_STATIC_LIB", "ON")
        .define("MCL_STANDALONE", "ON")
        .build();
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=mcl");
}
