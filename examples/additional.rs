//! Showcases additional input options
use clap::Parser;
use klask::Settings;
use std::io::{stdin, Read};

#[derive(Parser)]
struct Additional {
    /// Hides environment variables from output
    #[arg(long)]
    hide_environment_variables: bool,
    /// Hides stdin from output
    #[arg(long)]
    hide_stdin: bool,
    /// Hides working directory from output
    #[arg(long)]
    hide_working_directory: bool,
}

fn main() {
    let mut settings = Settings::default();
    settings.enable_env = Some("Additional env description!".into());
    settings.enable_stdin = Some("Additional stdin description!".into());
    settings.enable_working_dir = Some("Additional working dir description!".into());
    let main = |additional: Additional| {
        if !additional.hide_environment_variables {
            let v = std::env::vars().collect::<Vec<_>>();
            println!(
                "Environment variables: {:?} and {} more\n",
                &v[0..4],
                v.len() - 5
            );
        }

        if !additional.hide_stdin {
            println!("Stdin: {}\n", {
                let mut buf = String::new();
                stdin().read_to_string(&mut buf).unwrap();
                buf
            });
        }

        if !additional.hide_working_directory {
            println!("Directory: {:?}", std::env::current_dir().unwrap());
        }
    };
    #[cfg(not(target_arch = "wasm32"))]
    klask::run_derived_native::<Additional, _>(settings, main);
    #[cfg(target_arch = "wasm32")]
    klask::run_derived_web::<Additional, _>(
        settings,
        move |additional| async move { main(additional) },
    );
}
