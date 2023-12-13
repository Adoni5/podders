use arrow::{
    array::{Array, Int16Builder, LargeListBuilder, UInt32Array},
    datatypes::{DataType, Field, Schema},
    record_batch::RecordBatch,
};
use std::{error::Error, sync::Arc};

pub fn signal_schema() -> Schema {
    Schema::new(vec![
        Field::new("read_id", DataType::FixedSizeBinary(16), false), // minknow.uuid as binary
        Field::new(
            "signal",
            DataType::LargeList(Arc::new(Field::new("item", DataType::Int16, true))),
            false,
        ), // Large list of int16 for signal
        Field::new("samples", DataType::UInt32, false),              // uint32 for samples
    ])
}

pub fn signal_data(
    schema: Arc<Schema>,
    read_id: arrow::array::FixedSizeBinaryArray,
) -> Result<RecordBatch, Box<dyn Error>> {
    // Create dummy data
    // Create a LargeListBuilder
    let mut signal_builder = LargeListBuilder::new(Int16Builder::new());

    // Append a list to the LargeListBuilder
    signal_builder.values().append_value(100);
    signal_builder.values().append_value(200);
    signal_builder.values().append_value(300);
    signal_builder.append(true);
    let signal = Arc::new(signal_builder.finish());
    let samples = UInt32Array::from(vec![3]);

    // Create a RecordBatch
    let batch = RecordBatch::try_new(
        schema,
        vec![
            Arc::new(read_id) as Arc<dyn Array>,
            signal as Arc<dyn Array>,
            Arc::new(samples) as Arc<dyn Array>,
        ],
    )?;

    Ok(batch)
}
