use std::{
    env,
    process::{self, Command},
};

fn run_nix(function: &str) -> String {
    let mut command = Command::new(env!("NIX_BINARY"));
    command.arg("eval");
    command.arg(format!("{}#ffi", env!("NIX_CODE_SRC")));
    command.args(["--apply", function]);

    let output = match command.output() {
        Ok(output) => output,
        Err(error) => {
            eprintln!("Error spawning nix: {:?}", error);
            process::exit(-1);
        }
    };

    if !output.status.success() {
        eprintln!(
            "Error running nix. Latest log lines:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
        process::exit(-1);
    }

    let Ok(output_nix) = String::from_utf8(output.stdout) else {
        eprintln!("Error running nix. Non-utf8 output.");
        process::exit(-1);
    };

    return output_nix.trim().to_string();
}

fn initial() -> String {
    run_nix("ffi: ffi.initial")
}

fn update(state: &str) -> String {
    run_nix(&format!("ffi: ffi.update {}", state))
}

fn main() {
    println!("{:?}", initial());
    println!("{:?}", update("5"));
}
