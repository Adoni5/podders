use arrow::array::cast::{as_generic_binary_array, as_large_list_array};
use arrow::ipc::reader::FileReader;
use arrow::ipc::writer::FileWriter;

use arrow::record_batch::RecordBatch;
use flatbuffers::WIPOffset;
use reads::{_build_read_id, create_reads_arrow_schema, dummy_read_row};
use run_info::{_run_info_batch, run_info_schema};
use signal::{signal_data, signal_schema};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
mod reads;
mod run_info;
mod signal;
use std::sync::Arc;
use uuid::Uuid;
extern crate flatbuffers;

// import the generated code
#[allow(dead_code, unused_imports)]
#[path = "../static/footer_generated.rs"]
mod footer_generated;
pub use footer_generated::minknow::reads_format::{
    root_as_footer, ContentType, EmbeddedFile, EmbeddedFileArgs, EmbeddedFileBuilder, Footer,
    FooterArgs, Format,
};

fn _generate_signature() {}
fn main() -> arrow::error::Result<()> {
    let uuid = Uuid::parse_str("56202382-7cda-49e4-9403-2a4f6acc22ab").unwrap();
    let read_id = _build_read_id(uuid).unwrap();

    let read_schema = Arc::new(create_reads_arrow_schema().unwrap());
    let read_data = dummy_read_row(read_schema.clone(), read_id.clone()).unwrap();

    let run_info_schema = Arc::new(run_info_schema().unwrap());
    let run_info_batch = _run_info_batch(run_info_schema.clone()).unwrap();

    let signal_schema = Arc::new(signal_schema());
    let signal_data_ = signal_data(signal_schema.clone(), read_id).unwrap();

    // Footer flatbuffer time
    let mut builder = flatbuffers::FlatBufferBuilder::new();

    // Example metadata
    let file_identifier = builder.create_string("cbf91180-0684-4a39-bf56-41eaf437de9e");
    let software = builder.create_string("Podders");
    let pod5_version = builder.create_string("0.3.2");

    // Create individual EmbeddedFiles
    let mut embedded_files: Vec<WIPOffset<EmbeddedFile>> = Vec::new();

    // Serialize to Arrow IPC format and write to a POD5 file
    let mut file = File::create("example.pod5")?;
    // ----------------------- Header ------------------ //
    // POD5 signature
    let signature = [0x8B, 0x50, 0x4F, 0x44, 0x0D, 0x0A, 0x1A, 0x0A];
    file.write_all(&signature)?;

    // Section marker (UUID)
    let section_marker = Uuid::new_v4().as_bytes().to_vec();
    file.write_all(&section_marker)?;
    // This works fine up to here

    // ----------------------- TABLES ------------------ //
    // READS TABLE
    let offset: i64 = file.stream_position().unwrap() as i64;
    println!("reads offset {offset}");
    // Serialize the read RecordBatch to Arrow IPC format and write to the file
    {
        let mut writer = FileWriter::try_new(&file, &read_schema)?;
        writer.write(&read_data).unwrap();
        writer.finish().unwrap();
    }
    let length = file.stream_position().unwrap() as i64 - offset; // Replace with actual length
    println!("reads length {length}");

    let current_pos = file.stream_position()?;
    let padding_needed = (8 - (current_pos % 8)) % 8; // Calculate padding to reach 8-byte boundary
    println!("reads padding {padding_needed}");

    // Write padding bytes
    for _ in 0..padding_needed {
        file.write_all(&[0])?;
    }
    file.write_all(&section_marker)?;
    // Example data for an EmbeddedFile

    let embedded_file = EmbeddedFile::create(
        &mut builder,
        &EmbeddedFileArgs {
            offset,
            length,
            format: Format::FeatherV2, // Example format, adjust as needed
            content_type: ContentType::ReadsTable, // Example content type
        },
    );
    embedded_files.push(embedded_file);
    // RUN INFO TABLE

    let offset: i64 = file.stream_position().unwrap() as i64;
    println!("runinfo offset {offset}");

    {
        let mut writer: FileWriter<&File> = FileWriter::try_new(&file, &run_info_schema)?;
        writer.write(&run_info_batch).unwrap();
        writer.finish()?;
    }
    let length = file.stream_position().unwrap() as i64 - offset; // Replace with actual length
    println!("runinfo length {length}");

    let current_pos = file.stream_position()?;
    let padding_needed = (8 - (current_pos % 8)) % 8; // Calculate padding to reach 8-byte boundary
    println!("runinfo padding {padding_needed}");

    // Write padding bytes
    for _ in 0..padding_needed {
        file.write_all(&[0])?;
    }
    file.write_all(&section_marker)?;

    let embedded_file = EmbeddedFile::create(
        &mut builder,
        &EmbeddedFileArgs {
            offset,
            length,
            format: Format::FeatherV2, // Example format, adjust as needed
            content_type: ContentType::RunInfoTable, // Example content type
        },
    );
    embedded_files.push(embedded_file);
    // SIGNAL TABLE
    let offset: i64 = file.stream_position().unwrap() as i64;
    println!("singal offset {offset}");
    {
        let mut writer: FileWriter<&File> = FileWriter::try_new(&file, &signal_schema)?;

        for signal_data in signal_data_ {
            writer.write(&signal_data)?;
        }
        writer.finish()?;
    }
    let length = file.stream_position().unwrap() as i64 - offset; // Replace with actual length
    println!("signal length {length}");

    let current_pos = file.stream_position()?;
    let padding_needed = (8 - (current_pos % 8)) % 8; // Calculate padding to reach 8-byte boundary
    println!("signal padding {padding_needed}");

    // Write padding bytes
    for _ in 0..padding_needed {
        file.write_all(&[0])?;
    }
    file.write_all(&section_marker)?;
    let embedded_file = EmbeddedFile::create(
        &mut builder,
        &EmbeddedFileArgs {
            offset,
            length,
            format: Format::FeatherV2, // Example format, adjust as needed
            content_type: ContentType::SignalTable, // Example content type
        },
    );
    embedded_files.push(embedded_file);
    // ----------------------- FOOTER ------------------ //
    // Footer magic "FOOTER" padded to 8 bytes with zeroes
    let footer_magic: &[u8; 8] = b"FOOTER\0\0";
    file.write_all(footer_magic)?;
    // Create the vector of EmbeddedFiles
    let contents = builder.create_vector(&embedded_files);

    // Build the Footer table
    let footer = Footer::create(
        &mut builder,
        &FooterArgs {
            file_identifier: Some(file_identifier),
            software: Some(software),
            pod5_version: Some(pod5_version),
            contents: Some(contents),
        },
    );

    builder.finish(footer, None);
    let current_pos = file.stream_position().unwrap();
    let offset = current_pos;
    println!("pre footer wrrite pos {current_pos}");

    // Get the final FlatBuffer byte array to write to the file
    let buf = builder.finished_data();
    file.write_all(buf)?;
    let current_pos = file.stream_position()?;
    println!("post footer wrrite {current_pos}");
    let padding_needed = (8 - (current_pos % 8)) % 8; // Calculate padding to reach 8-byte boundary

    println!("{padding_needed}");
    // Write padding bytes
    for _ in 0..padding_needed {
        println!("Adding padding");
        file.write_all(&[0])?;
    }
    // Footer length (dummy value for example)
    let footer_length: i64 = file.stream_position().unwrap() as i64 - offset as i64;
    println!("footer length {footer_length}");
    file.write_all(&footer_length.to_le_bytes())?;

    // Write the section marker again
    file.write_all(&section_marker)?;

    // Ending with the same signature
    file.write_all(&signature)?;
    file.flush()?;
    Ok(())
}

fn read_arrow_table(
    file_path: &str,
    offset: u64,
    length: u64,
) -> arrow::error::Result<Vec<RecordBatch>> {
    let mut file = File::open(file_path)?;

    // Seek to the start of the embedded table
    file.seek(SeekFrom::Start(offset))?;

    // Read the specified length of bytes
    let mut buffer = vec![0; length as usize];
    file.read_exact(&mut buffer)?;

    // Deserialize bytes into Arrow RecordBatch
    let cursor = std::io::Cursor::new(buffer);
    let reader = FileReader::try_new(cursor, None)?;

    let mut batches = Vec::new();
    for batch in reader {
        batches.push(batch?);
    }

    Ok(batches)
}

struct FileInfo {
    offset: u64,
    length: u64,
}

fn read_pod5_footer(filename: &str, table: ContentType) -> FileInfo {
    let mut file = File::open(filename).unwrap();
    let _end = file.seek(SeekFrom::End(0)).unwrap();
    file.seek(SeekFrom::Current(-32)).unwrap(); // Signature + Section marker + 8 bytes for footer length
    let mut buffer = [0; 8]; // Buffer for 8 bytes

    file.read_exact(&mut buffer).unwrap(); // Read 8 bytes

    // Convert bytes to little-endian i64
    let value = i64::from_le_bytes(buffer);
    println!("{value}");
    // Seek to the footer position
    file.seek(SeekFrom::Current(-(8 + value))).unwrap();

    // Read the footer data
    let mut buf = vec![0; value as usize];
    file.read_exact(&mut buf).unwrap();

    // Deserialize the FlatBuffer
    let footer = root_as_footer(&buf).unwrap();
    // Now you can access the data from the footer
    let x: Vec<EmbeddedFile> = footer
        .contents()
        .unwrap()
        .iter()
        .filter(|x| x.content_type() == table)
        .collect();
    FileInfo {
        offset: x[0].offset() as u64,
        length: x[0].length() as u64,
    }
}

#[cfg(test)]
mod tests {

    use arrow::array::{Array, BinaryArray, Int16Array};

    use super::*;

    #[test]
    fn does_it_work() {
        main().unwrap()
    }
    #[test]
    fn test_reading_signal_table() {
        let file_info = read_pod5_footer("example.pod5", ContentType::SignalTable);
        let batch = read_arrow_table("example.pod5", file_info.offset, file_info.length).unwrap();
        let schema = batch[0].schema();

        for field in schema.fields() {
            println!(
                "Field name: {}, Data type: {}",
                field.name(),
                field.data_type()
            );
        }
        let signal = batch[0].column_by_name("signal").unwrap();
        println!("{}", signal.data_type());
        let collected = as_large_list_array(signal).values();
        println!("{}", signal.len());
    }
    #[test]
    fn test_reading_real_signal_table() {
        let file_info = read_pod5_footer(
            "/home/adoni5/Projects/podders/static/PAS89372_pass_aa50cb53_71dbefbe_0.pod5",
            ContentType::SignalTable,
        );
        let batch = read_arrow_table(
            "/home/adoni5/Projects/podders/static/PAS89372_pass_aa50cb53_71dbefbe_0.pod5",
            file_info.offset,
            file_info.length,
        )
        .unwrap();
        let schema = batch[0].schema();

        for field in schema.fields() {
            println!(
                "Field name: {}, Data type: {}",
                field.name(),
                field.data_type()
            );
        }
        let signal = batch[0].column_by_name("signal").unwrap();
        println!("{}", signal.data_type());
        println!("{}", signal.len());
    }
    #[test]
    fn test_reading_read_table() {
        let file_info = read_pod5_footer("example.pod5", ContentType::ReadsTable);

        let batch = read_arrow_table("example.pod5", file_info.offset, file_info.length).unwrap();
        let schema = batch[0].schema();

        for field in schema.fields() {
            println!(
                "Field name: {}, Data type: {}",
                field.name(),
                field.data_type()
            );
            // Access metadata for each field
            for (key, value) in field.metadata() {
                println!("  Metadata - Key: {}, Value: {}", key, value);
            }
        }
    }
    #[test]
    fn test_reading_real_read_table() {
        let batch = read_arrow_table(
            "/home/adoni5/Projects/podders/static/PAS89372_pass_aa50cb53_71dbefbe_0.pod5",
            250987192,
            435458,
        )
        .unwrap();
        let schema = batch[0].schema();

        for field in schema.fields() {
            println!(
                "Field name: {}, Data type: {}",
                field.name(),
                field.data_type()
            );
            // Access metadata for each field
            for (key, value) in field.metadata() {
                println!("  Metadata - Key: {}, Value: {}", key, value);
            }
        }
    }
    #[test]
    fn test_reading_real_run_table() {
        let batch = read_arrow_table(
            "/home/adoni5/Projects/podders/static/PAS89372_pass_aa50cb53_71dbefbe_0.pod5",
            250979728,
            7442,
        )
        .unwrap();
        let schema = batch[0].schema();

        for field in schema.fields() {
            println!(
                "Field name: {}, Data type: {}",
                field.name(),
                field.data_type()
            );
        }
    }
    #[test]
    fn test_reading_my_run_table() {
        let file_info = read_pod5_footer("example.pod5", ContentType::RunInfoTable);

        let batch = read_arrow_table(
            "/home/adoni5/Projects/podders/example.pod5",
            file_info.offset,
            file_info.length,
        )
        .unwrap();
        let schema = batch[0].schema();

        for field in schema.fields() {
            println!(
                "Field name: {}, Data type: {}",
                field.name(),
                field.data_type()
            );
        }
    }
    #[test]
    fn test_deserialising_footer() {
        let mut file = File::open("/home/adoni5/Projects/podders/example.pod5").unwrap();
        let _end = file.seek(SeekFrom::End(0)).unwrap();
        file.seek(SeekFrom::Current(-32)).unwrap(); // Signature + Section marker + 8 bytes for footer length
        let mut buffer = [0; 8]; // Buffer for 8 bytes

        file.read_exact(&mut buffer).unwrap(); // Read 8 bytes

        // Convert bytes to little-endian i64
        let value = i64::from_le_bytes(buffer);
        println!("{value}");
        // Seek to the footer position
        file.seek(SeekFrom::Current(-(8 + value))).unwrap();

        // Read the footer data
        let mut buf = vec![0; value as usize];
        file.read_exact(&mut buf).unwrap();

        // Deserialize the FlatBuffer
        let footer = root_as_footer(&buf).unwrap();
        // Now you can access the data from the footer
        println!("{:?}", footer.contents());
    }

    #[test]
    fn test_deserialising_real_footer() {}
}
