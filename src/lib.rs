use arrow::array::{
    BooleanArray, DictionaryArray, FixedSizeBinaryBuilder, Float32Array, ListArray, ListBuilder,
    UInt16Array, UInt32Array, UInt64Array, UInt64Builder, UInt8Array,
};

use arrow::ipc::writer::FileWriter;
use arrow::{
    array::{Array, StringArray},
    datatypes::{DataType, Field, Schema},
    record_batch::RecordBatch,
};
use run_info::{_run_info_batch, run_info_schema};
use signal::{signal_data, signal_schema};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
mod run_info;
mod signal;
use std::sync::Arc;
use uuid::Uuid;
fn _build_read_id(read_id: Uuid) -> arrow::error::Result<arrow::array::FixedSizeBinaryArray> {
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
fn _build_signal_index() -> arrow::error::Result<arrow::array::GenericListArray<i32>> {
    let values_builder = UInt64Builder::new();

    let mut list_builder = ListBuilder::new(values_builder);

    // Building a list [1, 2, 3]
    list_builder.values().append_value(1);
    list_builder.append(true);

    // Finish building and get the ListArray
    let signal: arrow::array::GenericListArray<i32> = list_builder.finish();
    Ok(signal)
}

fn main() -> arrow::error::Result<()> {
    let signal_field = Arc::new(Field::new("item", DataType::UInt64, true));
    // Define the fields as per the TOML specification

    let fields = vec![
        Field::new("read_id", DataType::FixedSizeBinary(16), false), // minknow.uuid as binary
        Field::new("signal", DataType::List(signal_field), false),
        Field::new("channel", DataType::UInt16, false),
        Field::new("well", DataType::UInt8, false),
        Field::new(
            "pore_type",
            DataType::Dictionary(Box::new(DataType::UInt32), Box::new(DataType::Utf8)),
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
            DataType::Dictionary(Box::new(DataType::UInt32), Box::new(DataType::Utf8)),
            false,
        ),
        Field::new("end_reason_forced", DataType::Boolean, false),
        Field::new(
            "run_info",
            DataType::Dictionary(Box::new(DataType::UInt32), Box::new(DataType::Utf8)),
            false,
        ),
        Field::new("num_samples", DataType::UInt64, false),
    ];
    // Define custom metadata
    let mut metadata = HashMap::new();
    metadata.insert("MINKNOW:pod5_version".to_string(), "1.0.0".to_string());
    metadata.insert(
        "MINKNOW:software".to_string(),
        "MinNOW Core 5.2.3".to_string(),
    );
    metadata.insert(
        "MINKNOW:file_identifier".to_string(),
        "cbf91180-0684-4a39-bf56-41eaf437de9e".to_string(),
    );

    // Create a schema with metadata
    let schema_with_metadata = Schema::new_with_metadata(fields, metadata);

    // Create the schema
    let schema = Arc::new(schema_with_metadata);

    let keys = UInt32Array::from(vec![0]);
    let values = StringArray::from(vec!["value1"]);

    let pore_type = DictionaryArray::try_new(keys, Arc::new(values)).unwrap();
    // Create dummy data for each field
    // Parse the UUID string
    let uuid = Uuid::parse_str("56202382-7cda-49e4-9403-2a4f6acc22ab").unwrap();
    let read_id = _build_read_id(uuid)?;
    let signal_: ListArray = _build_signal_index()?;
    let channel = UInt16Array::from(vec![1]);
    let well = UInt8Array::from(vec![1]);
    let calibration_offset = Float32Array::from(vec![100.0]);
    let calibration_scale = Float32Array::from(vec![1.0]);
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
    let keys = UInt32Array::from(vec![0]);
    let values = StringArray::from(vec!["value1"]);

    let end_reason = DictionaryArray::try_new(keys, Arc::new(values)).unwrap();
    let end_reason_forced = BooleanArray::from(vec![true]);
    let keys = UInt32Array::from(vec![0]);
    let values = StringArray::from(vec!["value1"]);

    let run_info = DictionaryArray::try_new(keys, Arc::new(values)).unwrap();
    let num_samples = UInt64Array::from(vec![1000]);
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
    )?;
    let run_info_schema = Arc::new(run_info_schema().unwrap());
    let run_info_batch = _run_info_batch(run_info_schema.clone()).unwrap();

    let signal_schema = Arc::new(signal_schema());
    let signal_data_ = signal_data(signal_schema.clone(), read_id).unwrap();
    // Serialize to Arrow IPC format and write to a POD5 file
    let mut file = File::create("example.pod5")?;

    // POD5 signature
    let signature = [0x8B, 0x50, 0x4F, 0x44, 0x0D, 0x0A, 0x1A, 0x0A];
    file.write_all(&signature)?;

    // Section marker (UUID)
    let section_marker = Uuid::new_v4().as_bytes().to_vec();
    file.write_all(&section_marker)?;

    // Serialize the RecordBatch to Arrow IPC format and write to the file
    {
        let mut writer = FileWriter::try_new(&file, &schema)?;
        writer.write(&batch2)?;
        writer.finish()?;
    }
    file.write_all(&section_marker)?;
    {
        let mut writer: FileWriter<&File> = FileWriter::try_new(&file, &run_info_schema)?;
        writer.write(&run_info_batch)?;
        writer.finish()?;
    }
    file.write_all(&section_marker)?;
    {
        let mut writer: FileWriter<&File> = FileWriter::try_new(&file, &signal_schema)?;
        writer.write(&signal_data_)?;
        writer.finish()?;
    }
    file.write_all(&section_marker)?;

    // Footer magic "FOOTER" padded to 8 bytes with zeroes
    let footer_magic = b"FOOTER\0\0";
    file.write_all(footer_magic)?;

    // Footer length (dummy value for example)
    let footer_length: i64 = 0; // Adjust this according to your actual footer length
    file.write_all(&footer_length.to_le_bytes())?;

    // Write the section marker again
    file.write_all(&section_marker)?;

    // Ending with the same signature
    file.write_all(&signature)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        main().unwrap()
    }
}
