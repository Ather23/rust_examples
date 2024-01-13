use std::fs;
use std::process::Command;

macro_rules! run_python_script {
    ($script_path:expr, $($arg:expr),*) => {
        let script_path = $script_path;
        let script = fs::read_to_string(&script_path)
            .expect("Failed to read Python script file");

        let mut python_command = Command::new("python");
        python_command.arg("-c").arg(&script);

        $(
            python_command.arg($arg);
        )*

        let output = python_command.output().expect("Failed to execute Python script");

        if output.status.success() {
            println!("Python script executed successfully:");
            println!("{}", String::from_utf8_lossy(&output.stdout));
        } else {
            eprintln!("Error executing Python script:");
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        }
    };
}

fn main() {
    let script_path = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/python_example.py");
    let argument1 = "TestingArg!";
    let argument2 = "arg2";

    run_python_script!(script_path, argument1, argument2);
}
