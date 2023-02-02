use std::process::Command;

fn main() {
    let cmd = "mkdir -p build && cd build && cmake ../mcl -DMCL_STATIC_LIB=ON && make -j";
    let output = Command::new("sh")
        .args(["-c", cmd])
        .output()
        .expect(&"fail");
    if !output.status.success() {
        panic!(
            "error:{}",
            String::from_utf8(output.stderr.clone()).expect("err")
        );
    }
}
