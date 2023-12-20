# Podders
![podchamp](static/64e9a8e2-6dab-4ce5-8b82-705f004eda3b.webp)
>[!NOTE]
> The pod champ commeth
> Pronounced pɒdəɹz 

Very simple native rust lib for writing POD5 files. No FFI Woooooooo LET'S GOOOOO

## Limitations
Many!
* ** Uncompressed signal** - I blasted this out in 7 days, so currently we are limited to uncompressed Signal.
* **Writing only** - Again does what I need it to do. I would like to add more features at some point, but for now we are stuck with this.

## Example usage
```rust

fn test() -> arrow::error::Result<()> {
    let mut pod5 = Pod5File::new("test_builder.pod5").unwrap();

    pod5.push_run_info(dummy_run_info());
    pod5.write_run_info_to_ipc();
    println!("{:#?}", pod5.run_table.length);

    let read = dummy_read_row(None).unwrap();
    let read_2 = dummy_read_row(Some("9e81bb6a-8610-4907-b4dd-4ed834fc414d")).unwrap();

    pod5.push_read(read);
    pod5.push_read(read_2);
    pod5.write_reads_to_ipc();
    // println!("{:#?}", pod5._signal);
    pod5.write_signal_to_ipc();
    pod5.write_footer();

    Ok(())
}
```

