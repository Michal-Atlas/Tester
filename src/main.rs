use ansi_term::Colour;
use std::env::args;
use std::fs::{read_dir, File};
use std::process::{Command, Output, Stdio};
use std::time::SystemTime;
use std::{fs, thread};

#[derive(Debug)]
struct Test {
    input: Option<File>,
    output: String,
    exit_code: i32,
}

fn main() {
    let mut args = args();
    args.next(); // First is program name
    let test_dir = &args.next().expect("No Test Path Provided");
    let run_cmd = &args.next().expect("No Command To Run Provided");
    let tests = match read_dir(test_dir) {
        Ok(t) => t,
        Err(e) => {
            panic!("Problem reading directory: {}", e)
        }
    };
    let mut handles = Vec::new();
    for test_dir in tests {
        let run_cmd = run_cmd.clone();
        let time_start = SystemTime::now();
        handles.push(thread::spawn(move || {
            let test_dir = test_dir.unwrap();
            let path = test_dir.path().clone();
            let mut test = Test {
                input: None,
                output: "".to_string(),
                exit_code: 0,
            };
            // Extract Test Info from Files
            for f in path.read_dir().expect("Error reading TestDir") {
                let f = match f {
                    Ok(r) => r,
                    Err(_) => {
                        continue;
                    }
                };
                match f.file_name().to_str().unwrap() {
                    "in" => {
                        test.input = Some(fs::File::open(f.path()).unwrap());
                    }
                    "out" => {
                        test.output = fs::read_to_string(f.path()).unwrap();
                    }
                    "exit" => {
                        test.exit_code = fs::read_to_string(f.path())
                            .unwrap()
                            .trim()
                            .parse()
                            .unwrap();
                    }
                    &_ => {}
                }
            }
            let output = Command::new(run_cmd)
                .stdin(Stdio::from(test.input.unwrap()))
                .output()
                .expect("Failed to execute");
            // Run the program

            let out_string = match output {
                Output { .. } if output.status.success() && test.exit_code != 0 => {
                    "  - ".to_string() + &Colour::Red.paint("Succeed on supposed fail").to_string()
                }
                Output { .. } if !output.status.success() && test.exit_code == 0 => {
                    "  - ".to_string() + &Colour::Red.paint("Fail on supposed Success").to_string()
                }
                Output { .. }
                    if std::str::from_utf8(output.stdout.as_slice())
                        .unwrap()
                        .trim()
                        != test.output.trim() =>
                {
                    "  - ".to_string()
                        + &Colour::Red.paint("Output not the same\n\n").to_string()
                        + "diff:\n"
                        + &Colour::Green
                            .paint(
                                fs::read_to_string(path.to_str().unwrap().to_owned() + "/out")
                                    .unwrap(),
                            )
                            .to_string()
                        + "\n"
                        + &Colour::Red
                            .paint(std::str::from_utf8(output.stdout.as_slice()).unwrap())
                            .to_string()
                }
                Output { .. } => " - ".to_string() + &Colour::Green.paint("OK").to_string(),
            };
            println!(
                "- {} [{}ms]:\n{}",
                Colour::Cyan
                    .bold()
                    .paint(test_dir.file_name().to_str().unwrap()),
                SystemTime::now()
                    .duration_since(time_start)
                    .unwrap()
                    .as_millis(),
                out_string
            );
        }));
    }
    for i in handles {
        i.join().unwrap();
    }
}
