//! Deal with creating or reading a pod5 file footer

use std::{
    error::Error,
    fs::File,
    io::{Read, Seek, SeekFrom, Write},
};

use flatbuffers::WIPOffset;
use uuid::Uuid;

use crate::{
    root_as_footer, ContentType, EmbeddedFile, EmbeddedFileArgs, Footer, FooterArgs, POD5_VERSION,
    SIGNATURE, SOFTWARE,
};
const FOOTER_MAGIC: &[u8; 8] = b"FOOTER\0\0";
pub struct FileInfo {
    pub offset: u64,
    pub length: u64,
}

pub fn read_pod5_footer(filename: &str, table: ContentType) -> FileInfo {
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
pub fn write_flatbuffer_footer(
    file_handle: &mut File,
    embedded_args: Vec<&EmbeddedFileArgs>,
    file_identifer: Uuid,
    section_marker: &[u8],
) -> Result<(), Box<dyn Error>> {
    let mut builder = flatbuffers::FlatBufferBuilder::new();
    // metadata
    let file_identifier = builder.create_string(&file_identifer.to_string());
    let software = builder.create_string(SOFTWARE);
    let pod5_version = builder.create_string(POD5_VERSION);
    // Create EmbeddedFiles
    let mut embedded_files: Vec<WIPOffset<EmbeddedFile>> = Vec::new();
    for embedded in embedded_args {
        let embedded_file = EmbeddedFile::create(&mut builder, embedded);
        embedded_files.push(embedded_file);
    }
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
    file_handle.write_all(FOOTER_MAGIC).unwrap();
    builder.finish(footer, None);
    let current_pos = file_handle.stream_position().unwrap();
    let offset = current_pos;
    println!("pre footer wrrite pos {current_pos}");

    // Get the final FlatBuffer byte array to write to the file
    let buf = builder.finished_data();
    file_handle.write_all(buf)?;
    let current_pos = file_handle.stream_position()?;
    println!("post footer wrrite {current_pos}");
    let padding_needed = (8 - (current_pos % 8)) % 8; // Calculate padding to reach 8-byte boundary

    println!("{padding_needed}");
    // Write padding bytes
    for _ in 0..padding_needed {
        println!("Adding padding");
        file_handle.write_all(&[0])?;
    }
    // Footer length (dummy value for example)
    let footer_length: i64 = file_handle.stream_position().unwrap() as i64 - offset as i64;
    println!("footer length {footer_length}");
    file_handle.write_all(&footer_length.to_le_bytes())?;

    // Write the section marker again
    file_handle.write_all(section_marker)?;

    // Ending with the same signature
    file_handle.write_all(&SIGNATURE)?;
    file_handle.flush()?;
    Ok(())
}
