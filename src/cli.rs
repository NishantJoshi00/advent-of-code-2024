use std::path::PathBuf;

use anyhow::ensure;

#[derive(Debug)]
pub struct Config {
    pub main_file: FileType,
    pub include: Vec<PathBuf>,
    pub input: Input,
}

#[derive(Debug)]
pub enum FileType {
    Wat(PathBuf),
    Wasm(PathBuf),
}
impl FileType {
    pub fn path(&self) -> &PathBuf {
        match self {
            Self::Wat(pth) | Self::Wasm(pth) => pth,
        }
    }
}

#[derive(Debug)]
pub enum Input {
    File(PathBuf),
    Stdin,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let mut args = std::env::args().skip(1).peekable();
        let mut main_file = None;
        let mut input = None;
        let mut include_list = Vec::new();

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-h" | "--help" | "+help" => {
                    let msg = r#"
usage: 
    aoc-runner [options] +wat <filename>
options:
    +i +input <filename>  : read input from file
    +stdin                : read input from stdin (default)
    +I +include <path>    : include directory for modules
"#;
                    println!("{}", msg);
                    std::process::exit(0);
                }
                "+wat" => {
                    if main_file.is_some() {
                        return Err(anyhow::anyhow!("Multiple +wat-file arguments provided"));
                    }

                    let filename = args
                        .next()
                        .map(PathBuf::from)
                        .ok_or_else(|| anyhow::anyhow!("Expected filename after +wat"))?;

                    ensure!(
                        filename.is_file(),
                        "Wat file does not exist: {:?}",
                        filename
                    );

                    main_file = Some(FileType::Wat(filename));
                }
                "+wasm" => {
                    if main_file.is_some() {
                        return Err(anyhow::anyhow!("Multiple +wat-file arguments provided"));
                    }

                    let filename = args
                        .next()
                        .map(PathBuf::from)
                        .ok_or_else(|| anyhow::anyhow!("Expected filename after +wasm"))?;

                    ensure!(
                        filename.is_file(),
                        "Wat file does not exist: {:?}",
                        filename
                    );

                    main_file = Some(FileType::Wasm(filename));
                }
                "+i" | "+input" => {
                    if input.is_some() {
                        return Err(anyhow::anyhow!("Multiple +input-file arguments provided"));
                    }

                    let filename = args
                        .next()
                        .map(PathBuf::from)
                        .ok_or_else(|| anyhow::anyhow!("Expected filename after +input-file"))?;

                    ensure!(
                        filename.is_file(),
                        "Input file does not exist: {:?}",
                        filename
                    );

                    input = Some(Input::File(filename));
                }
                "+stdin" => {
                    if input.is_some() {
                        return Err(anyhow::anyhow!("Multiple +input-file arguments provided"));
                    }

                    input = Some(Input::Stdin);
                }
                "+I" | "+include" => {
                    let path = args
                        .next()
                        .map(PathBuf::from)
                        .ok_or_else(|| anyhow::anyhow!("Expected path after +include"))?;

                    ensure!(path.is_file(), "Include path does not exist: {:?}", path);

                    include_list.push(path);
                }
                _ => {
                    return Err(anyhow::anyhow!("Unexpected argument: {}", arg));
                }
            }
        }

        let main_file =
            main_file.ok_or_else(|| anyhow::anyhow!("Missing `+wat` or `+wasm` argument"))?;
        let input = input.unwrap_or(Input::Stdin);

        Ok(Self {
            main_file,
            input,
            include: include_list,
        })
    }
}
