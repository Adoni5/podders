//! `podders` - A Rust crate for handling Pod5 file format, written in native rust.
//!
//! This library offers comprehensive functionalities for the Pod5 file format,
//! focusing on data serialization with Apache Arrow and FlatBuffers.
//! It includes capabilities to create, and write structured data in the Pod5 format,
//! adhering to the format as of 20/12/23 (v0.3.2)
//!
//! Key Features:
//! - Matches schemas for official pod5 specification.
//! - Writing Pod5 files with efficient serialization.
//!
//! The library provides various submodules (`reads`, `run_info`, `signal`, `footer`)
//! each dedicated to handling different aspects of the Pod5 file format.

use arrow::datatypes::Schema;
use arrow::ipc::reader::FileReader;
use arrow::ipc::writer::FileWriter;

use arrow::record_batch::RecordBatch;
use footer::write_flatbuffer_footer;
use reads::{create_read_batches, create_reads_arrow_schema, ReadInfo};
use run_info::{create_run_info_batch, dummy_run_info, run_info_schema, RunInfoData};
use signal::signal_schema;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
pub mod footer;
pub mod reads;
pub mod run_info;
pub mod signal;
use std::sync::Arc;
use uuid::Uuid;
extern crate flatbuffers;

// import the generated code
#[allow(dead_code, unused_imports)]
#[path = "../static/footer_generated.rs"]
pub mod footer_generated;
pub use footer_generated::minknow::reads_format::{
    root_as_footer, ContentType, EmbeddedFile, EmbeddedFileArgs, EmbeddedFileBuilder, Footer,
    FooterArgs, Format,
};

use crate::reads::dummy_read_row;

/// Pod5 in hexadecimal - file signature
const SIGNATURE: [u8; 8] = [0x8B, 0x50, 0x4F, 0x44, 0x0D, 0x0A, 0x1A, 0x0A];

/// Pod5 version at time of writing
const POD5_VERSION: &str = "0.3.2";

/// Podders version that wrote the file
const SOFTWARE: &str = "PODDERS! v0.1.0";

/// Generates a unique section marker for a file.
///
/// This function creates a new V4 UUID and converts it to a byte vector.
/// This is called each time a Pod5 struct is initialised, to demark each section
/// of the pod5 file.
///
/// # Examples
///
/// ```rust
/// # fn main() {
/// let section_marker = _generate_section_marker();
/// assert_eq!(section_marker.len(), 16); // UUID v4 is always 16 bytes
/// # }
/// # fn _generate_section_marker() -> Vec<u8> {
/// #    uuid::Uuid::new_v4().as_bytes().to_vec()
/// # }
/// ```
fn _generate_section_marker() {
    Uuid::new_v4().as_bytes().to_vec();
}

/// Writes a vector of RecordBatches to a file and updates the passed EmbeddedFileArgs in place.
///
/// This function writes the given RecordBatches to the provided file handle,
/// and then updates the offset and length in the EmbeddedFileArgs to reflect
/// the written data. It ensures that the written data aligns to an 8-byte boundary.
///
/// # Arguments
/// * `file_handle` - A mutable reference to the file where the data will be written.
/// * `section_marker` - A 16-byte array to mark the end of the written section.
/// * `batches` - A vector of RecordBatches to be written to the file.
/// * `embedded_file` - A mutable reference to an EmbeddedFileArgs struct to be updated with the new offset and length.
///
/// # Returns
/// A Result<(), std::io::Error>, indicating the success or failure of the operation.
///
/// # Example
/// ```
/// # use arrow::ipc::writer::FileWriter;
/// # use arrow::record_batch::RecordBatch;
/// # use std::fs::File;
/// # use uuid::Uuid;
/// # fn main() -> std::io::Result<()> {
/// # let mut file_handle = File::create("example.arrow")?;
/// let section_marker = Uuid::new_v4();
/// let batches: Vec<RecordBatch> = vec![]; // Populate with actual RecordBatches
/// let mut embedded_file = EmbeddedFileArgs {
///     offset: 0,
///     length: 0,
///     format: Format::FeatherV2,
///     content_type: ContentType::ReadsTable,
/// };
///
/// _write_table(&mut file_handle, section_marker.as_bytes(), &batches, &mut embedded_file)?;
/// // Now embedded_file contains updated offset and length
/// # Ok(())
/// # }
/// # fn _write_table(file_handle: &mut File, section_marker: &[u8; 16], batches: &Vec<RecordBatch>, embedded_file: &mut EmbeddedFileArgs) -> std::io::Result<()> {
/// #     Ok(())
/// # }
/// # struct EmbeddedFileArgs {
/// #     offset: i64,
/// #     length: i64,
/// #     format: Format,
/// #     content_type: ContentType,
/// # }
/// # enum Format { FeatherV2 }
/// # enum ContentType { ReadsTable }
/// ```
fn _write_table(
    mut file_handle: &mut File,
    section_marker: &[u8; 16],
    batches: &Vec<RecordBatch>,
    embedded_file: &mut EmbeddedFileArgs,
) -> Result<(), std::io::Error> {
    let offset: i64 = file_handle.stream_position().unwrap() as i64;
    // Assuming all RecordBatches have the same schema
    let schema = batches[0].schema();
    {
        let mut writer = FileWriter::try_new(file_handle, &schema).unwrap();

        for batch in batches {
            writer.write(batch).expect("Failed to write batch to IPC");
        }

        writer.finish().unwrap();
        file_handle = writer.into_inner().unwrap();
    }
    let length = file_handle.stream_position().unwrap() as i64 - offset; // Replace with actual length

    let current_pos = file_handle.stream_position()?;
    let padding_needed = (8 - (current_pos % 8)) % 8; // Calculate padding to reach 8-byte boundary
                                                      // Write padding bytes
    for _ in 0..padding_needed {
        file_handle.write_all(&[0])?;
    }
    embedded_file.length = length;
    embedded_file.offset = offset;
    file_handle.write_all(section_marker)?;
    file_handle.flush()?;
    Ok(())
}

/// Represents a Pod5 file, encapsulating all necessary components and metadata for handling Pod5 data.
pub struct Pod5File {
    /// File handle for reading from or writing to the Pod5 file.
    filehandle: File,
    /// Metadata and positional information for the reads table embedded in the file.
    read_table: EmbeddedFileArgs,
    /// Metadata and positional information for the run information table embedded in the file.
    run_table: EmbeddedFileArgs,
    /// Metadata and positional information for the signal table embedded in the file.
    signal_table: EmbeddedFileArgs,
    /// Buffer to hold `ReadInfo` data before writing to the file.
    _reads: Vec<ReadInfo>,
    /// Buffer to hold signal `RecordBatch` data before writing to the file.
    _signal: Vec<RecordBatch>,
    /// Buffer to hold `RunInfoData` before writing to the file.
    _run_info: Vec<RunInfoData>,
    /// Schema for the reads data.
    _reads_schema: Arc<Schema>,
    /// Schema for the run information data.
    _run_schema: Arc<Schema>,
    /// Schema for the signal data.
    _signal_schema: Arc<Schema>,
    /// Unique identifier used as a section marker in the file.
    _section_marker: Uuid,
    /// Unique identifier for the file, used for internal tracking and referencing.
    _file_identifier: Uuid,
}

impl Pod5File {
    /// Creates a new `Pod5File` instance, initializing it for tracking embedded files.
    ///
    /// This function generates a new Pod5 file at the specified filepath.
    /// It initializes the file with a signature, section marker, and file identifier,
    /// and sets up the metadata for the embedded reads, run information, and signal tables.
    ///
    /// # Arguments
    /// * `filepath` - The path where the new Pod5 file will be created.
    ///
    /// # Returns
    /// A `Result<Pod5File, Box<dyn Error>>`, which is the new `Pod5File` instance on success,
    /// or an error if the file creation or initialization fails.
    ///
    /// # Example
    /// ```rust,ignore
    /// let pod5_file = Pod5File::new("path/to/file.pod5");
    /// match pod5_file {
    ///     Ok(file) => println!("Pod5 file created successfully."),
    ///     Err(e) => println!("Error creating Pod5 file: {}", e),
    /// }
    /// ```
    pub fn new(filepath: &str) -> Result<Self, Box<dyn Error>> {
        let mut file = File::create(filepath)?;
        file.write_all(&SIGNATURE)?;
        let section_marker = Uuid::new_v4();
        let file_identifier = Uuid::new_v4();
        file.write_all(section_marker.as_bytes())?;
        Ok(Pod5File {
            filehandle: file,
            read_table: EmbeddedFileArgs {
                format: Format::FeatherV2, // Example format, adjust as needed
                content_type: ContentType::ReadsTable,
                offset: 0,
                length: 0, // Example content type
            },
            run_table: EmbeddedFileArgs {
                format: Format::FeatherV2, // Example format, adjust as needed
                content_type: ContentType::RunInfoTable,
                offset: 0,
                length: 0, // Example content type
            },
            signal_table: EmbeddedFileArgs {
                format: Format::FeatherV2, // Example format, adjust as needed
                content_type: ContentType::SignalTable,
                offset: 0,
                length: 0, // Example content type
            },
            _reads: vec![],
            _signal: vec![],
            _run_info: vec![],
            _reads_schema: Arc::new(create_reads_arrow_schema(&file_identifier)?),
            _run_schema: Arc::new(run_info_schema(&file_identifier)?),
            _signal_schema: Arc::new(signal_schema(&file_identifier)),
            _section_marker: section_marker,
            _file_identifier: file_identifier,
        })
    }

    /// Adds a new `RunInfoData` instance to the Pod5 file.
    ///
    /// This method appends the provided `RunInfoData` to the internal run information buffer
    /// of the `Pod5File` struct. It is used to accumulate run information before writing it to the file.
    ///
    /// # Arguments
    /// * `run_info` - The `RunInfoData` instance to be added to the Pod5 file.
    ///
    /// # Example
    /// ```rust,ignore
    /// # use podders::Pod5File;
    /// # use podders::run_info::RunInfoData;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut pod5_file = Pod5File::new("my_file.pod5")?;
    /// let run_info = RunInfoData { /* fields */ };
    /// pod5_file.push_run_info(run_info);
    /// # Ok(())
    /// # }
    /// ```
    pub fn push_run_info(&mut self, run_info: RunInfoData) {
        self._run_info.push(run_info)
    }

    /// Dump all created Run info RecordBatches (tables) into the file, and set the offset and length correctly
    /// on the Embedded file args
    pub fn write_run_info_to_ipc(&mut self) {
        let batches = create_run_info_batch(self._run_schema.clone(), &self._run_info).unwrap();
        _write_table(
            &mut self.filehandle,
            self._section_marker.as_bytes(),
            &batches,
            &mut self.run_table,
        )
        .unwrap();
    }

    /// Push reads to internal buffer, ready to be written out
    /// By a call to write_reads_to_ipc
    pub fn push_read(&mut self, read: ReadInfo) {
        self._reads.push(read)
    }

    pub fn write_signal_to_ipc(&mut self) {
        _write_table(
            &mut self.filehandle,
            self._section_marker.as_bytes(),
            &self._signal,
            &mut self.signal_table,
        )
        .unwrap();
    }

    /// Write the reads and signal in the internal buffer into the file. DO NOT CALL MORE THAN ONCE YOU WILL MESS THINGS UP
    pub fn write_reads_to_ipc(&mut self) {
        let batches = create_read_batches(
            self._reads_schema.clone(),
            &self._reads,
            &mut self._signal,
            self._signal_schema.clone(),
        )
        .unwrap();
        _write_table(
            &mut self.filehandle,
            self._section_marker.as_bytes(),
            &batches,
            &mut self.read_table,
        )
        .unwrap();
    }

    /// Write the footer and finish the file. Again please ONLY CALL ONCE PER FILE
    pub fn write_footer(&mut self) {
        let embedded_args = vec![&self.read_table, &self.run_table, &self.signal_table];
        write_flatbuffer_footer(
            &mut self.filehandle,
            embedded_args,
            self._file_identifier,
            self._section_marker.as_bytes(),
        )
        .unwrap()
    }
}

#[cfg(test)]
mod tests {

    use arrow::array::Array;

    use crate::footer::read_pod5_footer;

    use super::*;
    use arrow::array::cast::as_large_list_array;

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

    #[test]
    fn does_it_work() {
        test().unwrap()
    }
    #[test]
    fn test_reading_signal_table() {
        let file_info = read_pod5_footer("test_builder.pod5", ContentType::SignalTable);
        println!("{}, length: {}", file_info.offset, file_info.length);
        let batch =
            read_arrow_table("test_builder.pod5", file_info.offset, file_info.length).unwrap();
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
