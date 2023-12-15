use arrow::{
    datatypes::{DataType, Field, Schema, TimeUnit},
    record_batch::RecordBatch,
};
use std::{collections::HashMap, error::Error, sync::Arc};

use arrow::array::{
    Array, Int16Array, MapBuilder, StringArray, StringBuilder, TimestampMillisecondArray,
    UInt16Array,
};

pub fn _run_info_batch(schema: Arc<Schema>) -> arrow::error::Result<RecordBatch> {
    // Create dummy data for each field
    let acquisition_id = StringArray::from(vec!["value1"]);
    let acquisition_start_time =
        TimestampMillisecondArray::from(vec![1625097600000]).with_timezone("UTC".to_string());
    let adc_max = Int16Array::from(vec![32767]);
    let adc_min = Int16Array::from(vec![-32768]);

    // For context_tags and tracking_id (Map type)

    let mut map_builder = MapBuilder::new(None, StringBuilder::new(), StringBuilder::new());
    map_builder.keys().append_value("key1");
    map_builder.values().append_value("value1");
    map_builder.append(true)?;

    let context_tags = map_builder.finish();

    let experiment_name = StringArray::from(vec!["Experiment 1"]);
    let flow_cell_id = StringArray::from(vec!["FCID123"]);
    let flow_cell_product_code = StringArray::from(vec!["PC123"]);
    let protocol_name = StringArray::from(vec!["Protocol 1"]);
    let protocol_run_id = StringArray::from(vec!["PRID123"]);
    let protocol_start_time =
        TimestampMillisecondArray::from(vec![1625097600000]).with_timezone("UTC".to_string());
    let sample_id = StringArray::from(vec!["Sample 1"]);
    let sample_rate = UInt16Array::from(vec![5000]);
    let sequencing_kit = StringArray::from(vec!["Kit X"]);
    let sequencer_position = StringArray::from(vec!["Position 1"]);
    let sequencer_position_type = StringArray::from(vec!["Type A"]);
    let software = StringArray::from(vec!["Software 1.0"]);
    let system_name = StringArray::from(vec!["System 1"]);
    let system_type = StringArray::from(vec!["System Type A"]);
    let tracking_id = context_tags.clone(); // Assuming similar structure as context_tags

    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(acquisition_id) as Arc<dyn Array>,
            Arc::new(acquisition_start_time) as Arc<dyn Array>,
            Arc::new(adc_max) as Arc<dyn Array>,
            Arc::new(adc_min) as Arc<dyn Array>,
            Arc::new(context_tags) as Arc<dyn Array>,
            Arc::new(experiment_name) as Arc<dyn Array>,
            Arc::new(flow_cell_id) as Arc<dyn Array>,
            Arc::new(flow_cell_product_code) as Arc<dyn Array>,
            Arc::new(protocol_name) as Arc<dyn Array>,
            Arc::new(protocol_run_id) as Arc<dyn Array>,
            Arc::new(protocol_start_time) as Arc<dyn Array>,
            Arc::new(sample_id) as Arc<dyn Array>,
            Arc::new(sample_rate) as Arc<dyn Array>,
            Arc::new(sequencing_kit) as Arc<dyn Array>,
            Arc::new(sequencer_position) as Arc<dyn Array>,
            Arc::new(sequencer_position_type) as Arc<dyn Array>,
            Arc::new(software) as Arc<dyn Array>,
            Arc::new(system_name) as Arc<dyn Array>,
            Arc::new(system_type) as Arc<dyn Array>,
            Arc::new(tracking_id) as Arc<dyn Array>,
        ],
    )?;
    Ok(batch)
}

fn _tags_field(name: &str) -> Field {
    Field::new(
        name,
        DataType::Map(
            Arc::new(Field::new(
                "entries",
                DataType::Struct(
                    vec![
                        Field::new("keys", DataType::Utf8, false),
                        Field::new("values", DataType::Utf8, true),
                    ]
                    .into(),
                ),
                false,
            )),
            false,
        ),
        false,
    )
}

pub fn run_info_schema() -> Result<Schema, Box<dyn Error>> {
    let mut metadata: HashMap<String, String> = HashMap::new();
    metadata.insert("MINKNOW:pod5_version".to_string(), "1.0.0".to_string());
    metadata.insert("MINKNOW:software".to_string(), "Podders".to_string());
    metadata.insert(
        "MINKNOW:file_identifier".to_string(),
        "cbf91180-0684-4a39-bf56-41eaf437de9e".to_string(),
    );

    // Create a schema with metadata
    let schema_with_metadata = Schema::new_with_metadata(
        vec![
            Field::new("acquisition_id", DataType::Utf8, false),
            Field::new(
                "acquisition_start_time",
                DataType::Timestamp(TimeUnit::Millisecond, Some("UTC".to_string().into())),
                false,
            ),
            Field::new("adc_max", DataType::Int16, false),
            Field::new("adc_min", DataType::Int16, false),
            _tags_field("context_tags"),
            Field::new("experiment_name", DataType::Utf8, false),
            Field::new("flow_cell_id", DataType::Utf8, false),
            Field::new("flow_cell_product_code", DataType::Utf8, false),
            Field::new("protocol_name", DataType::Utf8, false),
            Field::new("protocol_run_id", DataType::Utf8, false),
            Field::new(
                "protocol_start_time",
                DataType::Timestamp(TimeUnit::Millisecond, Some("UTC".to_string().into())),
                false,
            ),
            Field::new("sample_id", DataType::Utf8, false),
            Field::new("sample_rate", DataType::UInt16, false),
            Field::new("sequencing_kit", DataType::Utf8, false),
            Field::new("sequencer_position", DataType::Utf8, false),
            Field::new("sequencer_position_type", DataType::Utf8, false),
            Field::new("software", DataType::Utf8, false),
            Field::new("system_name", DataType::Utf8, false),
            Field::new("system_type", DataType::Utf8, false),
            _tags_field("tracking_id"),
        ],
        metadata,
    );
    Ok(schema_with_metadata)
}
