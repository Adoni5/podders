use arrow::{
    array::{
        Array, BooleanArray, DictionaryArray, FixedSizeBinaryBuilder, Float32Array, Int16Array,
        ListArray, ListBuilder, StringArray, UInt16Array, UInt32Array, UInt64Array, UInt64Builder,
        UInt8Array,
    },
    datatypes::{DataType, Field, Schema},
    record_batch::RecordBatch,
};
use std::{collections::HashMap, error::Error, sync::Arc};
use uuid::Uuid;

pub fn _build_read_id(read_id: Uuid) -> arrow::error::Result<arrow::array::FixedSizeBinaryArray> {
    // Convert UUID to a 16-byte array
    let bytes = read_id.as_bytes();

    // Create a FixedSizeBinaryBuilder with element size 16
    let mut builder = FixedSizeBinaryBuilder::new(16);

    // Append the UUID bytes
    builder.append_value(bytes)?;

    // Build the FixedSizeBinaryArray
    let read_id: arrow::array::FixedSizeBinaryArray = builder.finish();
    Ok(read_id)
}

pub fn _build_signal_index() -> arrow::error::Result<arrow::array::GenericListArray<i32>> {
    let values_builder = UInt64Builder::new();

    let mut list_builder = ListBuilder::new(values_builder);

    // Building a list [1, 2, 3]
    list_builder.values().append_value(0);
    list_builder.values().append_value(1);
    list_builder.values().append_value(2);
    list_builder.values().append_value(3);
    list_builder.values().append_value(4);
    list_builder.values().append_value(6);
    list_builder.values().append_value(7);
    list_builder.values().append_value(8);
    list_builder.values().append_value(9);
    list_builder.values().append_value(10);
    list_builder.values().append_value(11);
    list_builder.values().append_value(12);
    list_builder.values().append_value(13);
    list_builder.append(true);

    // Finish building and get the ListArray
    let signal: arrow::array::GenericListArray<i32> = list_builder.finish();
    Ok(signal)
}

pub fn create_reads_arrow_schema() -> Result<Schema, Box<dyn Error>> {
    let signal_field = Arc::new(Field::new("item", DataType::UInt64, true));
    // Define the fields as per the TOML specification
    // Create a metadata map
    let mut metadata = HashMap::new();
    metadata.insert(
        "ARROW:extension:name".to_string(),
        "minknow.uuid".to_string(),
    );
    metadata.insert("ARROW:extension:metadata".to_string(), "".to_string());

    let fields = vec![
        Field::new("read_id", DataType::FixedSizeBinary(16), false).with_metadata(metadata), // minknow.uuid as binary
        Field::new("signal", DataType::List(signal_field), false),
        Field::new("channel", DataType::UInt16, false),
        Field::new("well", DataType::UInt8, false),
        Field::new(
            "pore_type",
            DataType::Dictionary(Box::new(DataType::Int16), Box::new(DataType::Utf8)),
            false,
        ),
        Field::new("calibration_offset", DataType::Float32, false),
        Field::new("calibration_scale", DataType::Float32, false),
        Field::new("read_number", DataType::UInt32, false),
        Field::new("start", DataType::UInt64, false),
        Field::new("median_before", DataType::Float32, false),
        Field::new("tracked_scaling_scale", DataType::Float32, false),
        Field::new("tracked_scaling_shift", DataType::Float32, false),
        Field::new("predicted_scaling_scale", DataType::Float32, false),
        Field::new("predicted_scaling_shift", DataType::Float32, false),
        Field::new("num_reads_since_mux_change", DataType::UInt32, false),
        Field::new("time_since_mux_change", DataType::Float32, false),
        Field::new("num_minknow_events", DataType::UInt64, false),
        Field::new(
            "end_reason",
            DataType::Dictionary(Box::new(DataType::Int16), Box::new(DataType::Utf8)),
            false,
        ),
        Field::new("end_reason_forced", DataType::Boolean, false),
        Field::new(
            "run_info",
            DataType::Dictionary(Box::new(DataType::Int16), Box::new(DataType::Utf8)),
            false,
        ),
        Field::new("num_samples", DataType::UInt64, false),
    ];
    // Define custom metadata
    let mut metadata = HashMap::new();
    metadata.insert("MINKNOW:pod5_version".to_string(), "1.0.0".to_string());
    metadata.insert("MINKNOW:software".to_string(), "Podders".to_string());
    metadata.insert(
        "MINKNOW:file_identifier".to_string(),
        "cbf91180-0684-4a39-bf56-41eaf437de9e".to_string(),
    );

    // Create a schema with metadata
    Ok(Schema::new_with_metadata(fields, metadata))
}

pub fn dummy_read_row(
    schema: Arc<Schema>,
    read_id: arrow::array::FixedSizeBinaryArray,
) -> Result<RecordBatch, Box<dyn Error>> {
    let keys = Int16Array::from(vec![0]);
    let values = StringArray::from(vec!["value1"]);

    let pore_type = DictionaryArray::try_new(keys, Arc::new(values)).unwrap();
    // Create dummy data for each field
    // Parse the UUID string
    let signal_: ListArray = _build_signal_index()?;
    let channel = UInt16Array::from(vec![1]);
    let well = UInt8Array::from(vec![1]);
    let calibration_offset = Float32Array::from(vec![-264.0]);
    let calibration_scale = Float32Array::from(vec![0.187_069_85]);
    let read_number = UInt32Array::from(vec![1]);
    let start = UInt64Array::from(vec![1_u64]);
    let median_before = Float32Array::from(vec![100.0]);
    // Dummy data for 'tracked_scaling_scale'
    let tracked_scaling_scale = Float32Array::from(vec![1.0]);

    // Dummy data for 'tracked_scaling_shift'
    let tracked_scaling_shift = Float32Array::from(vec![0.1]);

    // Dummy data for 'predicted_scaling_scale'
    let predicted_scaling_scale = Float32Array::from(vec![1.5]);

    // Dummy data for 'predicted_scaling_shift'
    let predicted_scaling_shift = Float32Array::from(vec![0.15]);

    // Dummy data for 'num_reads_since_mux_change'
    let num_reads_since_mux_change = UInt32Array::from(vec![10]);

    // Dummy data for 'time_since_mux_change'
    let time_since_mux_change = Float32Array::from(vec![5.0]);

    // Dummy data for 'num_minknow_events'
    let num_minknow_events = UInt64Array::from(vec![100]);
    let keys = Int16Array::from(vec![0]);
    let values = StringArray::from(vec!["value1"]);

    let end_reason = DictionaryArray::try_new(keys, Arc::new(values)).unwrap();
    let end_reason_forced = BooleanArray::from(vec![true]);
    let keys = Int16Array::from(vec![0]);
    let values = StringArray::from(vec!["value1"]);

    let run_info = DictionaryArray::try_new(keys, Arc::new(values)).unwrap();
    let num_samples = UInt64Array::from(vec![13872]);
    let batch2 = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(read_id.clone()),
            Arc::new(signal_) as Arc<dyn Array>,
            Arc::new(channel),
            Arc::new(well),
            Arc::new(pore_type) as Arc<dyn Array>,
            Arc::new(calibration_offset) as Arc<dyn Array>,
            Arc::new(calibration_scale) as Arc<dyn Array>,
            Arc::new(read_number) as Arc<dyn Array>,
            Arc::new(start) as Arc<dyn Array>,
            Arc::new(median_before) as Arc<dyn Array>,
            Arc::new(tracked_scaling_scale) as Arc<dyn Array>,
            Arc::new(tracked_scaling_shift) as Arc<dyn Array>,
            Arc::new(predicted_scaling_scale) as Arc<dyn Array>,
            Arc::new(predicted_scaling_shift) as Arc<dyn Array>,
            Arc::new(num_reads_since_mux_change) as Arc<dyn Array>,
            Arc::new(time_since_mux_change) as Arc<dyn Array>,
            Arc::new(num_minknow_events) as Arc<dyn Array>,
            Arc::new(end_reason) as Arc<dyn Array>,
            Arc::new(end_reason_forced) as Arc<dyn Array>,
            Arc::new(run_info) as Arc<dyn Array>,
            Arc::new(num_samples) as Arc<dyn Array>,
        ],
    )
    .unwrap();
    Ok(batch2)
}
