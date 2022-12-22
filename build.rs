use std::panic;
use std::path::Path;
use std::process::Command;
use std::string::String;

const FOLDER: &str = "./target/thirdparty/mcl";

const CLONE_SCRIPTS: &str = "
git clone https://github.com/herumi/mcl ./target/thirdparty/mcl 
";

const BUILD_SCRIPTS: &str = "
cd ./target/thirdparty/mcl && make lib/libmcl.a lib/libmclbn384_256.a -j4 CXX=clang++
";

fn run_command(script: &str) {
    let error_msg = format!("failed to run: {}", script);
    let output = Command::new("bash")
        .args(["-c", script])
        .output()
        .expect(&error_msg);

    let stdout =
        String::from_utf8(output.stdout.clone()).expect("error: encode command output to utf-8");
    let stderr =
        String::from_utf8(output.stderr.clone()).expect("error: encode command output to utf-8");

    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);

    if !output.status.success() {
        panic!("error: failed to running commands\n{}", stderr);
    }
}

fn main() {
    if !Path::new(FOLDER).exists() {
        run_command(CLONE_SCRIPTS);
    }

    run_command(BUILD_SCRIPTS);
}
