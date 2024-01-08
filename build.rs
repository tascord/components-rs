use std::process::Command;

pub fn main() {
    let cwd = std::env::current_dir().unwrap();
    let tw = cwd
        .join("tailwind")
        .canonicalize()
        .expect("./tailwind not found");
    let assets = cwd.join("assets");
    println!("Compiling tailwind");

    Command::new(tw)
        .arg("--minify")
        .arg("-i")
        .arg(
            assets
                .clone()
                .join("basic.css")
                .canonicalize()
                .expect("basic.css not found"),
        )
        .arg("-o")
        .arg(
            assets
                .clone()
                .join("tw.css")
                .canonicalize()
                .expect("tw.css not found"),
        )
        .status()
        .unwrap();
}
