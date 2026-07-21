use std::path::Path;
use std::process::Command;

fn main() {
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    if target_arch == "wasm32" {
        build_wasm();
        return;
    }

    let mut config = cmake::Config::new("mcl");
    config
        .define("MCL_STATIC_LIB", "ON")
        .define("MCL_STANDALONE", "ON")
        .define("MCL_TEST_WITH_GMP", "OFF");

    // On non-x86_64 targets mcl compiles LLVM IR (*.ll) directly with clang++,
    // so it must be specified explicitly; otherwise CMake aborts with
    // "requiring clang++".
    if target_arch != "x86_64" {
        config.define("CMAKE_CXX_COMPILER", "clang++");
    }

    let dst = config.build();
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=mcl");
}

// Build mcl for wasm by compiling a single source file (src/fp.cpp) as a
// freestanding object, mirroring mcl/Makefile.wasm. fp.cpp includes
// bn_c_impl.hpp, so the whole C API ends up in the object. The object is built
// for wasm32-unknown-unknown and links into both wasm32-unknown-unknown and
// wasm32-wasi Rust binaries.
fn build_wasm() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let manifest = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let mcl = Path::new(&manifest).join("mcl");
    let fp_cpp = mcl.join("src/fp.cpp");
    let obj = Path::new(&out_dir).join("fp.o");
    let lib = Path::new(&out_dir).join("libmcl.a");

    println!("cargo:rerun-if-env-changed=CLANG_VER");
    println!("cargo:rerun-if-env-changed=CXX");
    println!("cargo:rerun-if-env-changed=AR");
    let clangxx = find_tool("CXX", "clang++");
    let llvm_ar = find_tool("AR", "llvm-ar");

    // Flags follow mcl/Makefile.wasm (CXXFLAGS). Use the freestanding stub
    // headers under src/wasm. MCL_STANDALONE disables OpenSSL/CSPRNG/string,
    // and config.hpp auto-detects __wasm__ (MCL_SIZEOF_UNIT=4, default BLS12-381).
    let status = Command::new(&clangxx)
        .args([
            "--target=wasm32-unknown-unknown",
            "-O3",
            "-DNDEBUG",
            "-flto",
            "-fvisibility=hidden",
            "-fno-threadsafe-statics",
            "-fno-rtti",
            "-fno-stack-protector",
            "-fno-exceptions",
            "-std=c++03",
            "-DMCL_SIZEOF_UNIT=4",
            "-DMCL_STANDALONE",
            "-DCYBOZU_MINIMUM_EXCEPTION",
            "-Wall",
            "-Wextra",
        ])
        .arg("-I").arg(mcl.join("src/wasm"))
        .arg("-I").arg(mcl.join("include"))
        .arg("-I").arg(mcl.join("src"))
        .arg("-c")
        .arg("-o").arg(&obj)
        .arg(&fp_cpp)
        .status()
        .expect("failed to run clang++ for wasm");
    assert!(status.success(), "clang++ failed to compile fp.cpp for wasm");

    let _ = std::fs::remove_file(&lib);
    let status = Command::new(&llvm_ar)
        .arg("crs")
        .arg(&lib)
        .arg(&obj)
        .status()
        .expect("failed to run llvm-ar for wasm");
    assert!(status.success(), "llvm-ar failed to create libmcl.a");

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=mcl");
    println!("cargo:rerun-if-changed={}", fp_cpp.display());
}

// Resolve a wasm build tool. Precedence:
//   1. an explicit override env (CXX / AR), used verbatim;
//   2. CLANG_VER as a suffix on the base name, e.g. CLANG_VER=-18 selects
//      clang++-18 / llvm-ar-18 (same convention as mcl/Makefile.wasm);
//   3. the bare name (clang++ / llvm-ar).
fn find_tool(override_env: &str, base: &str) -> String {
    if let Ok(v) = std::env::var(override_env) {
        if !v.is_empty() {
            return v;
        }
    }
    if let Ok(ver) = std::env::var("CLANG_VER") {
        if !ver.is_empty() {
            return format!("{}{}", base, ver);
        }
    }
    base.to_string()
}
