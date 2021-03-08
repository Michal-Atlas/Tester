use ansi_term::Colour;
use std::env::args;
use std::fs;
use std::fs::{read_dir, File};
use std::io::Stdout;
use std::process::{Command, Output, Stdio};

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
    let mut failures: Vec<String> = vec![];
    'test: for test_dir in tests {
        let test_dir = test_dir.unwrap();
        let path = test_dir.path().clone();
        println!(
            "- {}:",
            Colour::Cyan
                .bold()
                .paint(test_dir.file_name().to_str().unwrap())
        );
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
        failures.push(path.to_str().unwrap().to_string());
        match output {
            Output { .. } if output.status.success() && test.exit_code != 0 => {
                println!("  - Succeed on supposed fail");
            }
            Output { .. } if !output.status.success() && test.exit_code == 0 => {
                println!("  - Fail on supposed Success");
            }
            Output { .. }
                if std::str::from_utf8(output.stdout.as_slice()).unwrap() != test.output =>
            {
                println!("  - Output not the same");
                let input = fs::read_to_string(path.to_str().unwrap().to_owned() + "/in").unwrap();
                println!(
                    "\n```\ndiff:\n{}\n{}\n```\n",
                    Colour::Green.paint(input),
                    Colour::Red.paint(std::str::from_utf8(output.stdout.as_slice()).unwrap())
                );
            }
            Output { .. } => {
                failures.pop();
                println!("  - {}", Colour::Green.paint("OK"))
            }
        }
    }
    if !failures.is_empty() {
        println!(
            "- {}:\n{:?}",
            Colour::Red.bold().paint("Failures"),
            failures
        )
    }
}
