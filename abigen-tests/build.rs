use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    let abis = vec!["tests"];

    for abi in abis {
        // All `path` arguments is relative to crate's Cargo.toml directory, in this example, it's 'abigen'
        let in_path = format!("abi/{}.json", abi);
        let out_path = format!("src/abi/{}.rs", abi);
        let additional_derives = vec![];

        Abigen::new(abi, &in_path)?
            .generate(additional_derives)?
            .write_to_file(&out_path)?;
    }

    Ok(())
}
