use arrow::{
    array::{Array, Int16Builder, LargeListBuilder, UInt32Array},
    datatypes::{DataType, Field, Schema},
    record_batch::RecordBatch,
};
use std::fs::File;
use std::io::{self, Read};
use std::{collections::HashMap, error::Error, sync::Arc};

fn read_int16_from_file(filename: &str) -> io::Result<Vec<i16>> {
    let mut f = File::open(filename)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;

    // Assuming the system has the same endianness as the file was written with
    let data: Vec<i16> = buffer
        .chunks_exact(2)
        .map(|chunk| i16::from_ne_bytes([chunk[0], chunk[1]]))
        .collect();
    println!("Data length: {}", data.len());
    Ok(data)
}

pub fn signal_schema() -> Schema {
    let mut metadata: HashMap<String, String> = HashMap::new();

    metadata.insert("MINKNOW:pod5_version".to_string(), "1.0.0".to_string());
    metadata.insert("MINKNOW:software".to_string(), "Podders".to_string());
    metadata.insert(
        "MINKNOW:file_identifier".to_string(),
        "cbf91180-0684-4a39-bf56-41eaf437de9e".to_string(),
    );

    let mut read_metadata = HashMap::new();
    read_metadata.insert(
        "ARROW:extension:name".to_string(),
        "minknow.uuid".to_string(),
    );
    read_metadata.insert("ARROW:extension:metadata".to_string(), "".to_string());

    Schema::new_with_metadata(
        vec![
            Field::new("read_id", DataType::FixedSizeBinary(16), false)
                .with_metadata(read_metadata), // minknow.uuid as binary
            Field::new(
                "signal",
                DataType::LargeList(Arc::new(Field::new("item", DataType::Int16, true))),
                false,
            ), // Large list of int16 for signal
            Field::new("samples", DataType::UInt32, false), // uint32 for samples
        ],
        metadata,
    )
}

pub fn signal_data(
    schema: Arc<Schema>,
    read_id: arrow::array::FixedSizeBinaryArray,
) -> Result<Vec<RecordBatch>, Box<dyn Error>> {
    // Create dummy data
    // Create a LargeListBuilder
    let mut batches = vec![];
    // Append a list to the LargeListBuilder

    let data = read_int16_from_file("static/test_signal.bin")?;

    // println!("Data length after move {}", data.len());
    for chunk in data.chunks(1000) {
        let mut signal_builder = LargeListBuilder::new(Int16Builder::new());
        signal_builder.values().append_slice(chunk);
        signal_builder.append(true);
        let samples = UInt32Array::from(vec![chunk.len() as u32]);
        let r_id = read_id.clone();
        let signal = Arc::new(signal_builder.finish());
        let batch = RecordBatch::try_new(
            schema.clone(),
            vec![
                Arc::new(r_id) as Arc<dyn Array>,
                signal.clone() as Arc<dyn Array>,
                Arc::new(samples) as Arc<dyn Array>,
            ],
        )?;
        batches.push(batch)
    }
    // Create a RecordBatch

    Ok(batches)
}
