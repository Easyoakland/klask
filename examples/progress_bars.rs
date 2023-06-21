use clap::{ArgMatches, Command};
use klask::Settings;
use std::thread;
use std::time::Duration;

fn main() {
    let main = |_: &ArgMatches| {
        const MAX: u64 = 100;

        for i in 0..=MAX {
            // You must pass in a value between [0, 1]
            klask::output::progress_bar("Static description", i as f32 / MAX as f32);
            klask::output::progress_bar_with_id(
                "Progress", // has to be a hashable id that identifies this progress bar
                &format!("Dynamic description [{i}/{MAX}]"),
                i as f32 / MAX as f32,
            );

            thread::sleep(Duration::from_millis(20));
        }

        println!("Finished!");
    };
    #[cfg(not(target_arch = "wasm32"))]
    klask::run_app_native(Command::new("Progress bars"), Settings::default(), main);
    #[cfg(target_arch = "wasm32")]
    klask::run_app_web(
        Command::new("Progress bars"),
        Settings::default(),
        move |matches| {
            let inner = |matches| async move { main(&matches) };
            inner(matches.clone())
        },
    );
}
