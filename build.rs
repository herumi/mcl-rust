use cmake::Config;

fn main() {
    let mut config = Config::new("mcl");

    config
        .define("MCL_STATIC_LIB", "ON")
        .define("MCL_STANDALONE", "ON");

    if cfg!(target_arch = "x86_64") {
        config.define("-DCMAKE_CXX_COMPILER", "clang++");
    }

    let dst = config.build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
}
