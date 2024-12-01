pub struct WasmString<'a>(pub &'a str);

impl<'a> WasmString<'a> {
    pub fn load_memory<T>(
        &'a self,
        memory: &wasmtime::Memory,
        store: &mut wasmtime::Store<T>,
    ) -> anyhow::Result<(i32, i32)> {
        let bytes = self.0.as_bytes();

        let ptr = memory.data_size(&store);
        let delta = (bytes.len() + ptr) / 65536 + 1;
        memory.grow(&mut *store, delta.try_into()?)?;

        memory.write(&mut *store, ptr, bytes)?;

        Ok((ptr as i32, bytes.len() as i32))
    }
}
