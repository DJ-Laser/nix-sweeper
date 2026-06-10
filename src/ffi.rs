use std::{
    io,
    process::{self, Command},
};

use crate::random::random;

fn run_nix(nix_function: &str, json: bool) -> io::Result<String> {
    let mut command = Command::new(env!("NIX_BINARY"));
    command.args(["--extra-experimental-features", "nix-command flakes"]);
    command.arg("eval");
    command.arg(format!("{}#ffi", env!("NIX_CODE_SRC")));
    command.args(["--apply", nix_function]);

    #[cfg(debug_assertions)]
    command.arg("--show-trace");

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

pub enum Action {
    Up,
    Down,
    Left,
    Right,

    Flag,
    Expose,

    Restart,
}

impl Action {
    pub fn as_nix(&self) -> String {
        match self {
            Self::Up => r#""up""#.to_string(),
            Self::Down => r#""down""#.to_string(),
            Self::Left => r#""left""#.to_string(),
            Self::Right => r#""right""#.to_string(),
            Self::Flag => r#""flag""#.to_string(),
            Self::Expose => r#""expose""#.to_string(),
            Self::Restart => r#""restart""#.to_string(),
        }
    }
}

pub struct NixState {
    nix_state: String,
}

impl NixState {
    fn from_run_nix(nix_function: &str) -> io::Result<Self> {
        let nix_output = run_nix(nix_function, true)?;

        Ok(Self {
            nix_state: nix_output,
        })
    }

    pub fn initial() -> io::Result<Self> {
        let random_seed = random()?;

        Self::from_run_nix(&format!("ffi: ffi.initial {random_seed}"))
    }

    pub fn update(&self, action: Action) -> io::Result<Self> {
        let action = action.as_nix();

        Self::from_run_nix(&format!(
            "ffi: ffi.update {action} (builtins.fromJSON ''{}'')",
            self.nix_state
        ))
    }

    pub fn output(&self) -> io::Result<String> {
        run_nix(
            &format!("ffi: ffi.output (builtins.fromJSON ''{}'')", self.nix_state),
            false,
        )
    }
}
