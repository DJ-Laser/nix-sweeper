use std::{
    io,
    process::{self, Command},
};

use crate::random::random;

fn run_nix(function: &str, json: bool) -> io::Result<String> {
    let mut command = Command::new(env!("NIX_BINARY"));
    command.args(["--extra-experimental-features", "nix-command flakes"]);
    command.arg("eval");
    command.arg(format!("{}#ffi", env!("NIX_CODE_SRC")));
    command.args(["--apply", function]);

    if json {
        command.arg("--json");
    } else {
        command.arg("--raw");
    }

    let output = match command.output() {
        Ok(output) => output,
        Err(error) => {
            return Err(io::Error::new(
                error.kind(),
                format!("Error spawning nix: {:?}", error),
            ));
        }
    };

    if !output.status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "Error running nix. Latest log lines:\n{}",
                String::from_utf8_lossy(&output.stderr)
            ),
        ));
    }

    let Ok(output_nix) = String::from_utf8(output.stdout) else {
        eprintln!("Error running nix. Non-utf8 output.");
        process::exit(1);
    };

    return Ok(output_nix.trim().to_string());
}

pub fn initial(width: usize, height: usize, num_mines: usize) -> io::Result<String> {
    let random_seed = random()?;

    run_nix(
        &format!(
            "ffi: ffi.initial {{
          board_width = {width};
          board_height = {height};
          num_mines = {num_mines};
          random_seed = {random_seed};
      }}"
        ),
        true,
    )
}

pub fn output(state: &str) -> io::Result<String> {
    run_nix(
        &format!("ffi: ffi.output (builtins.fromJSON ''{state}'')"),
        false,
    )
}
