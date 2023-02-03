use std::process::Command;

fn main() {
    let opt = if cfg!(target_arch = "x86_64") {
        ""
    } else {
        "-DCMAKE_CXX_COMPILER=clang++"
    };

    let cmd = format!(
        "mkdir -p build && cd build && cmake ../mcl -DMCL_STATIC_LIB=ON {} && make -j",
        opt
    );
    let output = Command::new("sh")
        .args(["-c", &cmd])
        .output()
        .expect("fail");
    if !output.status.success() {
        panic!(
            "error:{}",
            String::from_utf8(output.stderr.clone()).expect("err")
        );
    }
}
