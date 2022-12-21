use std::process::Command;
use std::string::String;

const SCRIPTS: &str = "
git clone https://github.com/herumi/mcl ./target/thirdparty/mcl
cd ./target/thirdparty/mcl
make lib/libmcl.a lib/libmclbn384_256.a -j4 CXX=clang++
";

fn main() {
    let output = Command::new("bash")
        .args(["-c", SCRIPTS])
        .output()
        .expect("error: running build.sh");

    let stdout = String::from_utf8(output.stdout).expect("error: encode command output to utf-8");

    let stderr = String::from_utf8(output.stderr).expect("error: encode command output to utf-8");

    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);
}
