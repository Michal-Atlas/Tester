use ansi_term::Colour;
use std::ffi::{OsStr, OsString};
use std::time::SystemTime;
use tokio::{
    fs::{read_dir, File},
    io::Result,
    process::Command,
};

#[tokio::main]
async fn main() -> Result<()> {
    let yaml = clap::load_yaml!("cli.yml");
    let matches = clap::App::from_yaml(yaml).get_matches();

    let test_dir = &matches.args["test_dir"].vals[0];
    let run_cmd = &matches.args["run_cmd"].vals[0];

    let mut tests = read_dir(test_dir).await?;

    while let Some(d) = tests.next_entry().await? {
        test(run_cmd, d).await?;
    }

    Ok(())
}

async fn test(run_cmd: &std::ffi::OsString, test_mod: tokio::fs::DirEntry) -> Result<()> {
    let time_start = SystemTime::now();
    println!("Ran test: {:?}", test_mod.path());

    let mut dir = read_dir(test_mod.path()).await?;
    let mut f_in = None;
    let mut f_out = None;
    let mut f_exit = None;
    while let Some(f) = dir.next_entry().await? {
        match f.file_name().to_str() {
            Some("in") => {
                f_in = Some(f);
            }
            Some("out") => {
                f_out = Some(f);
            }
            Some("exit") => {
                f_exit = Some(f);
            }
            _ => {}
        }
    }
    let f_in = f_in.unwrap();
    let f_out = tokio::fs::read_to_string(f_out.unwrap().path());
    let f_exit = tokio::fs::read_to_string(f_exit.unwrap().path());

    let output = tokio::process::Command::new(run_cmd)
        .stdin(std::process::Stdio::from(std::fs::File::open(f_in.path())?))
        .output()
        .await?;

    let f_exit = f_exit.await?;
    let f_out = f_out.await?;
    let out_string = match output {
        _ if output.status.success() && f_exit.trim() != "0" => {
            "  - ".to_string() + &Colour::Red.paint("Succeed on supposed fail").to_string()
        }
        _ if !output.status.success() && f_exit.trim() == "0" => {
            "  - ".to_string() + &Colour::Red.paint("Fail on supposed Success").to_string()
        }
        _ if std::str::from_utf8(output.stdout.as_slice())
            .unwrap()
            .trim()
            != f_out.trim() =>
        {
            " - ".to_string()
                + &Colour::Red.paint("Output not the same\n\n").to_string()
                + "diff:\n"
                + &Colour::Green
                    .paint(f_out)
                    .to_string()
                + "\n"
                + &Colour::Red
                    .paint(std::str::from_utf8(output.stdout.as_slice()).unwrap())
                    .to_string()
        }
        _ => " - ".to_string() + &Colour::Green.paint("OK").to_string(),
    };
    println!(
        "- {} [{}ms]:\n{}",
        Colour::Cyan
            .bold()
            .paint(test_mod.file_name().to_str().unwrap()),
        SystemTime::now()
            .duration_since(time_start)
            .unwrap()
            .as_millis(),
        out_string
    );

    Ok(())
}
