use core::str::FromStr;
use std::io::Write;

use wasmtime::Caller;

pub struct Loader;

impl Loader {
    const MODULE_NAME: &'static str = "env";

    pub fn bind<T: 'static>(linker: &mut wasmtime::Linker<T>) -> anyhow::Result<()> {
        linker.func_wrap(Loader::MODULE_NAME, "print", print::<T>)?;
        linker.func_wrap(Loader::MODULE_NAME, "load_input_day1", load_input_day1::<T>)?;

        Ok(())
    }
}

fn print<T>(mut caller: Caller<'_, T>, ptr: i32, len: i32) -> anyhow::Result<()> {
    let memory = caller
        .get_export("memory")
        .and_then(|m| m.into_memory())
        .ok_or_else(|| anyhow::anyhow!("No memory"))?;

    let mut buffer = vec![0u8; len as usize];
    memory.read(&mut caller, ptr as usize, &mut buffer)?;

    let string = std::str::from_utf8(&buffer)?;

    std::io::stdout().write_all(string.as_bytes())?;
    std::io::stdout().flush()?;

    Ok(())
}

fn load_input_day1<T>(mut caller: Caller<'_, T>, ptr: i32, len: i32) -> anyhow::Result<()> {
    let memory = caller
        .get_export("memory")
        .and_then(|m| m.into_memory())
        .ok_or_else(|| anyhow::anyhow!("No memory"))?;

    let mut buffer = vec![0u8; len as usize];
    memory.read(&mut caller, ptr as usize, &mut buffer)?;

    let string = std::str::from_utf8(&buffer)?;

    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in string.lines() {
        let data = line
            .split_whitespace()
            .map(FromStr::from_str)
            .collect::<Result<Vec<i32>, _>>()?;

        if data.len() != 2 {
            return Err(anyhow::anyhow!("Invalid input"));
        }

        left.push(data[0]);
        right.push(data[1]);
    }
    Ok(())
}
