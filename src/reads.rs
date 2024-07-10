//! # POD5 File Format Utilities
//!
//! This module provides utilities for creating and manipulating Apache Arrow data structures,
//! particularly for handling read-related data in the POD5 file format. It includes functions for:
//!
//! - Building UUID arrays (`_build_read_id`) representing read identifiers.
//! - Constructing signal index arrays (`_build_signal_index`) that map read data to signal data.
//! - Creating a comprehensive Arrow schema (`create_reads_arrow_schema`) for reads data with detailed fields and metadata.
//! - Generating a dummy read row (`dummy_read_row`) for testing, which encompasses various data types and arrays as per the POD5 format specifications.
//!
//! This module is essential for managing read data in the context of POD5 files, leveraging Apache Arrow's capabilities in Rust.

use crate::{
    signal::{handle_signal_data, read_int16_from_file},
    POD5_VERSION, SOFTWARE,
};
use arrow::{
    array::{
        Array, BooleanArray, DictionaryArray, FixedSizeBinaryBuilder, Float32Array, Int16Array,
        ListArray, ListBuilder, StringArray, UInt16Array, UInt32Array, UInt64Array, UInt64Builder,
        UInt8Array,
    },
    datatypes::{DataType, Field, Int16Type, Schema},
    record_batch::RecordBatch,
};
// use log::debug;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt,
    sync::Arc,
};
use uuid::Uuid;

pub enum PoreType {
    R9,
    R10,
    NotSet,
}
impl fmt::Display for PoreType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PoreType::R9 => "R9.4.1",
                PoreType::R10 => "R10.4.1",
                PoreType::NotSet => "not-set",
            }
        )
    }
}

#[allow(non_camel_case_types)]
pub enum EndReason {
    UNKNOWN,
    MUX_CHANGE,
    UNBLOCK_MUX_CHANGE,
    DATA_SERVICE_UNBLOCK_MUX_CHANGE,
    SIGNAL_POSITIVE,
    SIGNAL_NEGATIVE,
}
impl fmt::Display for EndReason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                EndReason::UNKNOWN => "unknown",
                EndReason::MUX_CHANGE => "mux_change",
                EndReason::UNBLOCK_MUX_CHANGE => "unblock_mux_change",
                EndReason::DATA_SERVICE_UNBLOCK_MUX_CHANGE => "data_service_unblock_mux_change",
                EndReason::SIGNAL_POSITIVE => "signal_positive",
                EndReason::SIGNAL_NEGATIVE => "signal_negative",
            }
        )
    }
}

/// Constructs a `FixedSizeBinaryArray` from a given UUID.
///
/// This function takes a UUID and converts it into a 16-byte array.
/// It then builds a `FixedSizeBinaryArray` which is a suitable format
/// for handling fixed-size binary data in Apache Arrow, particularly UUIDs.
///
/// # Arguments
///
/// * `read_id` - A `Uuid` to be converted into a `FixedSizeBinaryArray`.
///
/// # Returns
///
/// An `arrow::error::Result` containing a `FixedSizeBinaryArray` or an error.
///
/// # Examples
///
/// ```rust
/// use uuid::Uuid;
/// use podders::reads::_build_read_id; // replace with actual module name
///
/// let uuid = Uuid::new_v4();
/// let fixed_size_binary_array = _build_read_id(uuid).unwrap();
/// assert_eq!(fixed_size_binary_array.value_length(), 16); // Corrected line
/// assert_eq!(fixed_size_binary_array.value(0), uuid.as_bytes());
/// ```
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

/// Builds a `GenericListArray<i32>` representing a list of `UInt64` values.
///
/// Demonstrates creating a list in Apache Arrow using a `ListBuilder`.
/// The list is explicitly populated with a series of `UInt64` values.
///
/// # Returns
///
/// An `arrow::error::Result` containing a `GenericListArray<i32>` or an error.
///
/// # Examples
///
/// ```rust
/// use podders::reads::_build_signal_index; // replace with actual module name
///
/// let signal_array = _build_signal_index().unwrap();
/// assert_eq!(signal_array.value_length(0), 1);
/// let values = signal_array.values();
/// let values = values.as_any().downcast_ref::<arrow::array::UInt64Array>().unwrap();
/// assert_eq!(values.value(0), 0);
///
/// ```
pub fn _build_signal_index<I>(
    values: I,
) -> arrow::error::Result<arrow::array::GenericListArray<i32>>
where
    I: Iterator<Item = usize>,
{
    let values_builder = UInt64Builder::new();

    let mut list_builder = ListBuilder::new(values_builder);

    // Building a list [1, 2, 3]
    for i in values {
        list_builder.values().append_value(i as u64);
    }
    list_builder.append(true);

    // Finish building and get the ListArray
    let signal: arrow::array::GenericListArray<i32> = list_builder.finish();
    Ok(signal)
}

/// Creates an Arrow schema for read data in the POD5 file format.
///
/// Defines a schema with multiple fields, each corresponding to a specific aspect
/// of read data, such as read ID, signal, channel, etc. Metadata is also attached to the schema
/// to provide additional context and conform to the POD5 format specifications.
///
/// Schema is defined [here](https://github.com/nanoporetech/pod5-file-format/blob/0ba232d6304dd1eebd60d331a6f7c15099dcd04f/docs/tables/reads.toml#L4)
///
/// # Returns
///
/// A `Result<Schema, Box<dyn Error>>` that contains either the constructed `Schema` or an error.
///
/// # Examples
///
/// ```
/// use podders::reads::create_reads_arrow_schema; // replace with the actual module name
///
/// let schema_result = create_reads_arrow_schema();
/// assert!(schema_result.is_ok());
/// let schema = schema_result.unwrap();
/// assert_eq!(schema.fields().len(), 21); // Asserting that 20 fields are defined
/// ```
pub fn create_reads_arrow_schema(file_identifier: &Uuid) -> Result<Schema, Box<dyn Error>> {
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
    metadata.insert("MINKNOW:pod5_version".to_string(), POD5_VERSION.to_string());
    metadata.insert("MINKNOW:software".to_string(), SOFTWARE.to_string());
    metadata.insert(
        "MINKNOW:file_identifier".to_string(),
        file_identifier.to_string(),
    );

    // Create a schema with metadata
    Ok(Schema::new_with_metadata(fields, metadata))
}

pub struct ReadInfo {
    pub read_id: Uuid,
    pub pore_type: PoreType,
    pub signal_: Vec<i16>,
    pub channel: u16,
    pub well: u8,
    pub calibration_offset: f32,
    pub calibration_scale: f32,
    pub read_number: u32,
    pub start: u64,
    pub median_before: f32,
    pub tracked_scaling_shift: f32,
    pub tracked_scaling_scale: f32,
    pub predicted_scaling_shift: f32,
    pub predicted_scaling_scale: f32,
    pub num_reads_since_mux_change: u32,
    pub time_since_mux_change: f32,
    pub num_minknow_events: u64,
    pub end_reason: EndReason,
    pub end_reason_forced: bool,
    pub run_info: String,
    pub num_samples: u64,
}

///
/// Generates a `RecordBatch` with predefined dummy values for each field defined in the provided schema.
/// It is particularly useful for testing and simulation purposes where actual data is not required.
///
/// # Arguments
///
/// * `schema` - An `Arc<Schema>` representing the schema of the record batch.
/// * `read_id` - A `FixedSizeBinaryArray` representing the read IDs.
///
/// # Returns
///
/// A `Result<RecordBatch, Box<dyn Error>>` containing the constructed `RecordBatch` or an error.
///
/// # Examples
///
/// ```
/// use arrow::datatypes::{Schema, Field, DataType};
/// use arrow::array::{FixedSizeBinaryArray, FixedSizeBinaryBuilder};
/// use std::sync::Arc;
/// use uuid::Uuid;
/// use podders::reads::{create_reads_arrow_schema, dummy_read_row}; // Replace with actual module name
///
/// let schema = create_reads_arrow_schema().unwrap();
/// let read_id = Uuid::new_v4();
/// let mut builder = FixedSizeBinaryBuilder::new(16);
/// builder.append_value(read_id.as_bytes()).unwrap();
/// let fixed_size_binary_array = builder.finish();
/// let record_batch = dummy_read_row(Arc::new(schema), fixed_size_binary_array).unwrap();
/// assert_eq!(record_batch.num_columns(), 21); // Number of fields in the schema
/// ```
pub fn create_read_batches(
    schema: Arc<Schema>,
    reads: &Vec<ReadInfo>,
    _signal: &mut Vec<RecordBatch>,
    signal_schema: Arc<Schema>,
) -> Result<Vec<RecordBatch>, Box<dyn Error>> {
    let mut pore_values: Vec<String> = reads.iter().map(|x| x.pore_type.to_string()).collect();
    let mut end_reasons: Vec<String> = reads.iter().map(|x| x.end_reason.to_string()).collect();
    let mut run_index: Vec<String> = reads
        .iter()
        .map(|x: &ReadInfo| x.run_info.to_string())
        .collect();
    pore_values.append(&mut end_reasons);
    pore_values.append(&mut run_index);
    let unique_values: HashSet<String> = HashSet::from_iter(pore_values.drain(0..));
    let pore_values: Vec<String> = Vec::from_iter(unique_values);
    let dict_map: HashMap<&String, i16> = pore_values
        .iter()
        .enumerate()
        .map(|(index, value)| (value, index as i16))
        .collect();
    let dict_values = Arc::new(StringArray::from(pore_values.clone()));

    let mut batches = vec![];
    for read in reads {
        let pt = Arc::new(
            DictionaryArray::<Int16Type>::try_new(
                Int16Array::from(vec![*dict_map.get(&read.pore_type.to_string()).unwrap()]),
                dict_values.clone(),
            )
            .unwrap(),
        );
        let er = Arc::new(
            DictionaryArray::<Int16Type>::try_new(
                Int16Array::from(vec![*dict_map.get(&read.end_reason.to_string()).unwrap()]),
                dict_values.clone(),
            )
            .unwrap(),
        );
        let rid = Arc::new(
            DictionaryArray::<Int16Type>::try_new(
                Int16Array::from(vec![*dict_map.get(&read.run_info).unwrap()]),
                dict_values.clone(),
            )
            .unwrap(),
        );
        batches.push(
            create_read_row(
                schema.clone(),
                read,
                _signal,
                signal_schema.clone(),
                pt,
                er,
                rid,
            )
            .unwrap(),
        )
    }
    Ok(batches)
}

pub fn create_read_row(
    schema: Arc<Schema>,
    read: &ReadInfo,
    _signal: &mut Vec<RecordBatch>,
    signal_schema: Arc<Schema>,
    pore_type: Arc<DictionaryArray<arrow::datatypes::Int16Type>>,
    end_reason: Arc<DictionaryArray<arrow::datatypes::Int16Type>>,
    run_info: Arc<DictionaryArray<arrow::datatypes::Int16Type>>,
) -> Result<RecordBatch, Box<dyn Error>> {
    // let keys = Int16Array::from(vec![read.pore_type]);
    // let values = StringArray::from(vec!["R10.4.1"]);
    let read_id = _build_read_id(read.read_id)?;
    // let pore_type: DictionaryArray<arrow::datatypes::Int16Type> =
    //     DictionaryArray::try_new(keys, Arc::new(values)).unwrap();

    // <-------------------- Handle Signal -------------------->
    let signal_batches = handle_signal_data(signal_schema, read_id.clone(), &read.signal_)?;
    let num_signal_rows = signal_batches.len();
    // debug!("Adding this many rows {num_signal_rows}");
    let offset = _signal.len();
    // debug!("Number of rows already - {offset}");
    _signal.extend(signal_batches);
    // debug!("now this many rows {}", _signal.len());
    let signal_: ListArray = _build_signal_index(offset..(offset + num_signal_rows))?;
    // debug!("Signal row indices for this read {:#?}", signal_);
    // <--------------------- Rest of the fields ---------------->
    let channel = UInt16Array::from(vec![read.channel]);
    let well = UInt8Array::from(vec![read.well]);
    let calibration_offset = Float32Array::from(vec![read.calibration_offset]);
    let calibration_scale = Float32Array::from(vec![read.calibration_scale]);
    let read_number = UInt32Array::from(vec![read.read_number]);
    let start = UInt64Array::from(vec![read.start]);
    let median_before = Float32Array::from(vec![read.median_before]);
    let tracked_scaling_scale = Float32Array::from(vec![read.tracked_scaling_scale]);
    let tracked_scaling_shift = Float32Array::from(vec![read.tracked_scaling_shift]);
    let predicted_scaling_scale = Float32Array::from(vec![read.predicted_scaling_scale]);
    let predicted_scaling_shift = Float32Array::from(vec![read.predicted_scaling_shift]);
    let num_reads_since_mux_change = UInt32Array::from(vec![read.num_reads_since_mux_change]);
    let time_since_mux_change = Float32Array::from(vec![read.time_since_mux_change]);
    let num_minknow_events = UInt64Array::from(vec![read.num_minknow_events]);
    // let keys = Int16Array::from(vec![read.end_reason]);
    // let values = StringArray::from(vec!["Stop Receiving"]);
    // let end_reason: DictionaryArray<arrow::datatypes::Int16Type> = DictionaryArray::try_new(keys, Arc::new(values)).unwrap();
    let end_reason_forced = BooleanArray::from(vec![read.end_reason_forced]);
    // let keys = Int16Array::from(vec![read.run_info]);
    // let values = StringArray::from(vec!["run_id_1"]);
    // let run_info = DictionaryArray::try_new(keys, Arc::new(values)).unwrap();
    let num_samples = UInt64Array::from(vec![read.num_samples]);
    let batch2 = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(read_id.clone()),
            Arc::new(signal_) as Arc<dyn Array>,
            Arc::new(channel),
            Arc::new(well),
            pore_type,
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
            end_reason,
            Arc::new(end_reason_forced) as Arc<dyn Array>,
            run_info,
            Arc::new(num_samples) as Arc<dyn Array>,
        ],
    )
    .unwrap();
    Ok(batch2)
}

pub fn dummy_read_row(read_id: Option<&str>) -> Result<ReadInfo, Box<dyn Error>> {
    let signal_data = read_int16_from_file("static/test_signal.bin")?;
    let signal_data: Vec<i16> = signal_data
        .iter()
        .cycle()
        .take(signal_data.len() * 10)
        .cloned()
        .collect();
    let num_samples = signal_data.len() as u64;
    let read: ReadInfo = ReadInfo {
        read_id: Uuid::parse_str(read_id.unwrap_or("56202382-7cda-49e4-9403-2a4f6acc22ab"))?,
        pore_type: PoreType::R10,
        signal_: signal_data,
        channel: 1,
        well: 1,
        calibration_offset: -264.0,
        calibration_scale: 0.187_069_85,
        read_number: 1,
        start: 1,
        median_before: 100.0,
        tracked_scaling_scale: 1.0,
        tracked_scaling_shift: 0.1,
        predicted_scaling_scale: 1.5,
        predicted_scaling_shift: 0.15,
        num_reads_since_mux_change: 10,
        time_since_mux_change: 5.0,
        num_minknow_events: 100,
        end_reason: EndReason::SIGNAL_POSITIVE,
        end_reason_forced: true,
        run_info: "value1".to_string(),
        num_samples,
    };
    Ok(read)
}
