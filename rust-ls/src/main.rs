use std::fs::ReadDir;
use std::{env, fs};
use std::env::consts::OS;

use clap::Parser;
use colored::Colorize;
use log::{trace, debug, info, warn};

mod os_types;

/// ls implementation in Rust
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Present directories first
    // By default both directories and files are presented in alphanumerical order
    #[arg(short, long, default_value_t = false)]
    sort_by_dir: bool,

    #[arg()]
    dir: String,
}

const WHITESPACES: &str = "                                                                                                                                                     ";
const WHITESPACES_PER_FIELD: usize = 40;

// Copy the amount of whitespaces needed and append them to the string
fn fill_whitespaces(string: &mut String){
    let space_count = WHITESPACES_PER_FIELD - string.len();
    let spaces = WHITESPACES[..space_count].to_string().clone();
    string.push_str(&spaces);
}

fn main() {
    let args = Args::parse();
    let os_args: Vec<_> = env::args().collect();

    // Get target arg and ignore all clap args
    // Turns out this is unnecessary: clap does that by default?
    if os_args.len() > 1{
        for val in &os_args[2..]{
            let first = val.chars().next().unwrap();
            let second = val.chars().next().unwrap();

            // If first 2 characters are "--" assume clap args
            // This never prints, so clap::Parser does it by default?
            if first == '-' && second == '-'{
                debug!("Ignored arg: {}, assumed clap Arg", val);
                continue;
            }

            trace!(target: "os args", "Received arg: {}", val);
        }
    }
    warn!("Going to dir: {}", args.dir);

    let dir_contents: ReadDir;
    match fs::read_dir(args.dir.as_str()){
        Ok(contents) => dir_contents = contents,
        Err(error) => {
            println!("Could not go to dir '{}'\nerror: '{}'", args.dir, error);
            return;
        }
    }

    // Setup based on OS right now unused
    let _use_filetypes: bool;
    let _use_simlink: bool;

    trace!("OS is: {}", OS);
    match OS{
        os_types::OsTypes::WINDOWS => {
            _use_filetypes = true;
            _use_simlink = false;
        },
        os_types::OsTypes::LINUX => {
            _use_filetypes = false;
            _use_simlink = true;
        },
        _ => {
            trace!("Could not detect OS, got: {}", OS);
            _use_filetypes = false;
            _use_simlink = false;
        }
    }

    let mut output: String = "filename".to_string().to_owned();
    fill_whitespaces(& mut output);
    output.push_str("type\n");
    println!("{}", output);

    for val in dir_contents{
        let value = val.as_ref().unwrap();
        let is_dir: bool = value.file_type().unwrap().is_dir();
        let is_link: bool = value.file_type().unwrap().is_symlink();

        let color =  if is_dir {colored::Color::Green} else {colored::Color::White};

        let filename = value.file_name().into_string().unwrap();
        let mut output: String = filename;
        fill_whitespaces(&mut output);
        let mut filetype: String = String::new();
        if is_dir{
            filetype.push_str("dir");
        } else if is_link{
            filetype.push_str("link");
        } else{
            filetype.push_str("file");
        }
        fill_whitespaces(&mut filetype);

        print!("{}{}\n", output.color(color), filetype.color(color));
    }

}