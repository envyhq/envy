use wasmer::{imports, Instance, Module, Store};

fn main() -> anyhow::Result<()> {
    let wasm_bytes = include_bytes!("../../../../../target/out.wasm");

    let mut store = Store::default();
    let module = Module::new(&store, wasm_bytes)?;
    let import_object = imports! {};
    let instance = Instance::new(&mut store, &module, &import_object)?;

    let var_x = instance.exports.get_global("var_x.a")?;
    let var_b = instance.exports.get_global("var_b")?;

    let memory = instance.exports.get_memory("memory")?;

    let view = memory.view(&store);
    let mut buf: [u8; 4] = [0, 0, 0, 0];
    view.read(0, &mut buf)?;

    println!("{:?}", std::str::from_utf8(&buf));

    println!("{:?} - {:?}", var_x.get(&mut store), var_b.get(&mut store));

    Ok(())
}

#[test]
fn test_hello_world() -> anyhow::Result<()> {
    main()
}
