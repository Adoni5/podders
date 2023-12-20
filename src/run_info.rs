use arrow::{
    array::MapArray,
    datatypes::{DataType, Field, Schema, TimeUnit},
    record_batch::RecordBatch,
};
use std::{collections::HashMap, error::Error, sync::Arc};
use uuid::Uuid;

use arrow::array::{
    Array, Int16Array, MapBuilder, StringArray, StringBuilder, TimestampMillisecondArray,
    UInt16Array,
};

use crate::{POD5_VERSION, SOFTWARE};

pub struct RunInfoData {
    pub acquisition_id: String,
    pub acquisition_start_time: i64, // Timestamp in milliseconds
    pub adc_max: i16,
    pub adc_min: i16,
    pub context_tags: HashMap<String, String>, // Simplified representation of MapArray
    pub experiment_name: String,
    pub flow_cell_id: String,
    pub flow_cell_product_code: String,
    pub protocol_name: String,
    pub protocol_run_id: String,
    pub protocol_start_time: i64, // Timestamp in milliseconds
    pub sample_id: String,
    pub sample_rate: u16,
    pub sequencing_kit: String,
    pub sequencer_position: String,
    pub sequencer_position_type: String,
    pub software: String,
    pub system_name: String,
    pub system_type: String,
    pub tracking_id: HashMap<String, String>, // Assuming similar structure as context_tags
}

fn convert_hashmap_to_maparray(map: &HashMap<String, String>) -> arrow::error::Result<MapArray> {
    let mut map_builder = MapBuilder::new(None, StringBuilder::new(), StringBuilder::new());

    for (key, value) in map {
        map_builder.keys().append_value(key);
        map_builder.values().append_value(value);
    }
    map_builder.append(true)?;

    Ok(map_builder.finish())
}

pub fn create_run_info_batch(
    schema: Arc<Schema>,
    run_infos: &Vec<RunInfoData>,
) -> arrow::error::Result<Vec<RecordBatch>> {
    let mut batches = vec![];
    for run_info in run_infos {
        // Create dummy data for each field
        let acquisition_id = StringArray::from(vec![run_info.acquisition_id.clone()]);
        let acquisition_start_time =
            TimestampMillisecondArray::from(vec![run_info.acquisition_start_time])
                .with_timezone("UTC".to_string());
        let adc_max = Int16Array::from(vec![run_info.adc_max]);
        let adc_min = Int16Array::from(vec![run_info.adc_min]);
        let context_tags = convert_hashmap_to_maparray(&run_info.context_tags)?;
        let experiment_name = StringArray::from(vec![run_info.experiment_name.clone()]);
        let flow_cell_id = StringArray::from(vec![run_info.flow_cell_id.clone()]);
        let flow_cell_product_code =
            StringArray::from(vec![run_info.flow_cell_product_code.clone()]);
        let protocol_name = StringArray::from(vec![run_info.protocol_name.clone()]);
        let protocol_run_id = StringArray::from(vec![run_info.protocol_run_id.clone()]);
        let protocol_start_time =
            TimestampMillisecondArray::from(vec![run_info.protocol_start_time])
                .with_timezone("UTC".to_string());
        let sample_id = StringArray::from(vec![run_info.sample_id.clone()]);
        let sample_rate = UInt16Array::from(vec![run_info.sample_rate]);
        let sequencing_kit = StringArray::from(vec![run_info.sequencing_kit.clone()]);
        let sequencer_position = StringArray::from(vec![run_info.sequencer_position.clone()]);
        let sequencer_position_type =
            StringArray::from(vec![run_info.sequencer_position_type.clone()]);
        let software = StringArray::from(vec![run_info.software.clone()]);
        let system_name = StringArray::from(vec![run_info.system_name.clone()]);
        let system_type = StringArray::from(vec![run_info.system_type.clone()]);
        let tracking_id = convert_hashmap_to_maparray(&run_info.tracking_id)?;

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
        batches.push(batch);
    }
    Ok(batches)
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

pub fn run_info_schema(file_identifier: &Uuid) -> Result<Schema, Box<dyn Error>> {
    let mut metadata: HashMap<String, String> = HashMap::new();
    metadata.insert("MINKNOW:pod5_version".to_string(), POD5_VERSION.to_string());
    metadata.insert("MINKNOW:software".to_string(), SOFTWARE.to_string());
    metadata.insert(
        "MINKNOW:file_identifier".to_string(),
        file_identifier.to_string(),
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

pub fn dummy_run_info() -> RunInfoData {
    RunInfoData {
        acquisition_id: "value1".to_string(),
        acquisition_start_time: 1625097600000,
        adc_max: 32767,
        adc_min: -32768,
        context_tags: [
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string()),
        ]
        .iter()
        .cloned()
        .collect(),
        experiment_name: "Experiment 1".to_string(),
        flow_cell_id: "FCID123".to_string(),
        flow_cell_product_code: "PC123".to_string(),
        protocol_name: "Protocol 1".to_string(),
        protocol_run_id: "PRID123".to_string(),
        protocol_start_time: 1625097600000,
        sample_id: "Sample 1".to_string(),
        sample_rate: 5000,
        sequencing_kit: "Kit X".to_string(),
        sequencer_position: "Position 1".to_string(),
        sequencer_position_type: "Type A".to_string(),
        software: "Software 1.0".to_string(),
        system_name: "System 1".to_string(),
        system_type: "System Type A".to_string(),
        tracking_id: [("key1".to_string(), "value1".to_string())]
            .iter()
            .cloned()
            .collect(),
    }
}
