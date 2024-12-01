use std::io::Read;

pub mod builtins;
pub mod cli;
pub mod types;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("wasmtime=off".parse().unwrap())
                .add_directive("wit_parser=off".parse().unwrap())
                .add_directive("cranelift=off".parse().unwrap()),
        )
        .init();

    let config = cli::Config::from_env()?;
    tracing::debug!(?config, "loading config");

    let engine = wasmtime::Engine::default();
    let mut store = wasmtime::Store::new(&engine, ());
    let mut linker = wasmtime::Linker::new(&engine);
    tracing::debug!("loading runtime");

    // attach builtins
    builtins::Loader::bind(&mut linker)?;
    tracing::debug!("attaching builtins");

    // load modules
    for pth in config.include {
        let filename = pth
            .file_name()
            .and_then(|f| f.to_str())
            .ok_or_else(|| anyhow::anyhow!("Invalid filename"))?;

        let filename = filename.split('.').next().unwrap();

        let module = wasmtime::Module::from_file(&engine, &pth)?;

        linker.module(&mut store, filename, &module)?;
        tracing::debug!(?pth, "loaded module");
    }

    // load main module
    let main_module = wasmtime::Module::from_file(&engine, config.main_file.path())?;
    let instance = linker.instantiate(&mut store, &main_module)?;
    tracing::debug!("loaded main module");

    // get memory
    let memory = instance
        .get_memory(&mut store, "memory")
        .ok_or_else(|| anyhow::anyhow!("No memory"))?;
    tracing::debug!("accessing runtime memory");

    let input = match config.input {
        cli::Input::File(pth) => std::fs::read_to_string(pth)?,
        cli::Input::Stdin => {
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf)?;
            buf
        }
    };
    tracing::debug!(?input, "loading input");

    let input = types::WasmString(&input);
    let input_string = input.load_memory(&memory, &mut store)?;
    tracing::debug!(?input_string, "loaded input into the runtime");

    let main = instance.get_typed_func::<(i32, i32), ()>(&mut store, "main")?;
    tracing::debug!("running main function");

    main.call(&mut store, input_string)?;

    Ok(())
}
