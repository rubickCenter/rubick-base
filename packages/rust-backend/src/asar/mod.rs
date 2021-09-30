#![allow(dead_code)]
mod error;
mod util;

pub use error::Error;
use prost::bytes::Buf;
use serde_json::{json, Value};
use std::{
    env,
    fs::File,
    fs::{self, OpenOptions},
    io::{self, Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};
use util::{align_size, read_u32, write_u32};

/// Maximum possible file size for files in asar archives.
const MAX_SIZE: u64 = std::u32::MAX as u64;

/// Read the header of an asar archive and extract the header size & json.
///
/// This may return an `io::Error` if there is an error reading the file.
fn read_header(reader: &mut File) -> Result<(u32, Value, bool), io::Error> {
    // read header bytes
    let mut header_buffer = vec![0u8; 16];
    reader.read_exact(&mut header_buffer)?;

    // grab sizes
    let header_size = read_u32(&header_buffer[4..8]);
    let json_size = read_u32(&header_buffer[12..]);

    // read json bytes
    let mut json_buffer = vec![0u8; json_size as usize];
    reader.read_exact(&mut json_buffer)?;

    // parse json
    let json: Value = serde_json::from_slice(&json_buffer)?;
    let compressed = if let Some(compressed) = json.get("compress") {
        if let Some(compressed) = compressed.as_bool() {
            compressed
        } else {
            false
        }
    } else {
        false
    };

    Ok((header_size + 8, json, compressed))
}

/// Iterate over all entries in an asar archive.
fn iterate_entries(json: &Value, mut callback: impl FnMut(&Value, &PathBuf)) {
    iterate_entries_err(json, |current, path| {
        callback(current, path);
        Ok(())
    })
    .expect("Unexpected error while iterating archive entries");
}

/// Iterate over all entries in an asar archive while forwarding errors from the passed closure.
fn iterate_entries_err(
    json: &Value,
    mut callback: impl FnMut(&Value, &PathBuf) -> Result<(), Error>,
) -> Result<(), Error> {
    fn helper(
        current: &Value,
        path: PathBuf,
        callback: &mut impl FnMut(&Value, &PathBuf) -> Result<(), Error>,
    ) -> Result<(), Error> {
        callback(current, &path)?;
        if current["files"] != Value::Null {
            for (key, val) in current["files"].as_object().unwrap() {
                helper(&val, path.join(key), callback)?;
            }
        }
        Ok(())
    }
    for (key, val) in json["files"].as_object().unwrap() {
        helper(val, PathBuf::new().join(key), &mut callback)?;
    }
    Ok(())
}

/// Get a list of all files in an asar archive.
///
/// # Examples
///
/// ```no_run
/// let file_entries = rasar::list("myarchive.asar").expect("Something went wrong");
/// ```
pub fn list(archive: &str) -> Result<Vec<PathBuf>, io::Error> {
    let mut file = File::open(archive)?;

    // read header
    let (_, json, _) = read_header(&mut file)?;

    // list files
    let mut files = vec![];
    iterate_entries(&json, |_, path| files.push(path.clone()));

    Ok(files)
}

/// Pack a directory into an asar archive.
///
/// # Examples
///
///  level 0-21, 0 is nocompress
///
/// ```no_run
/// match rasar::pack("myfolder", "myarchive.asar", 0) {
/// 	Ok(()) => println!("Success!"),
/// 	Err(err) => panic!("This should not have happened!")
/// }
/// ```
pub fn pack(path: &str, dest: &str, level: i32) -> Result<(), Error> {
    let mut header_json = json!({
        "files": {},
        "compress": if level == 0 {false} else {true}
    });
    let tmp_file_name = format!(".{}", dest);
    let mut tmp_file = fs::File::create(&tmp_file_name)?;
    let dir = PathBuf::from(path);

    if fs::try_exists(&path).unwrap() {
        fn walk_dir(
            dir: impl AsRef<Path>,
            json: &mut Value,
            mut offset: &mut usize,
            level: i32,
            mut archive: &mut File,
        ) -> Result<(), Error> {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let name = entry
                    .file_name()
                    .into_string()
                    .expect("Error converting OS path to string");
                let meta = entry.metadata()?;
                let entry_path = entry.path();
                if meta.is_file() {
                    if meta.len() > MAX_SIZE {
                        panic!(
                            "File {} is above the maximum possible size of {} GB",
                            name,
                            MAX_SIZE as f64 / 1e9
                        );
                    }
                    let mut buf = vec![];
                    let size;
                    if level == 0 {
                        io::copy(&mut File::open(entry_path)?, &mut archive)?;
                        size = meta.len() as usize;
                    } else {
                        // encoder;
                        zstd::stream::copy_encode(&mut File::open(entry_path)?, &mut buf, level)?;
                        size = archive.write(&buf)?;
                    }
                    json[&name] = json!({
                        "offset": offset,
                        "size": size
                    });
                    *offset += size;
                } else {
                    json[&name] = json!({
                        "files": {}
                    });
                    walk_dir(
                        entry_path,
                        &mut json[&name]["files"],
                        &mut offset,
                        level,
                        &mut archive,
                    )?
                }
            }
            Ok(())
        }
        walk_dir(dir, &mut header_json["files"], &mut 0, level, &mut tmp_file)?;
    } else {
        panic!("No such file or directory {}!", path);
    }

    // create header buffer with json
    let mut header = serde_json::to_vec(&header_json)?;

    // compute sizes
    let json_size = header.len();
    let size = align_size(json_size);

    // resize header
    header.resize(16 + size, 0);

    // copy json
    header.copy_within(0..json_size, 16);

    // write sizes into header
    write_u32(&mut header[0..4], 4);
    write_u32(&mut header[4..8], 8 + size as u32);
    write_u32(&mut header[8..12], 4 + size as u32);
    write_u32(&mut header[12..16], json_size as u32);

    let mut archive = fs::File::create(dest)?;
    // write header
    archive.write(&header)?;
    // write body
    io::copy(&mut File::open(&tmp_file_name)?, &mut archive)?;
    // remove tmp file
    fs::remove_file(tmp_file_name)?;

    Ok(())
}

/// Extract all files from an asar archive.
///
/// # Examples
///
/// ```no_run
/// match rasar::extract("myarchive.asar", "extracted") {
/// 	Ok(()) => println!("Success!"),
/// 	Err(err) => panic!("This should not have happened!")
/// }
/// ```
pub fn extract(archive: &str, dest: &str) -> Result<(), Error> {
    let mut file = File::open(archive)?;

    // read header
    let (header_size, json, compressed) = read_header(&mut file)?;

    // create destination folder
    let dest = PathBuf::from(dest);
    if !dest.exists() {
        fs::create_dir(&dest)?;
    }

    // file.seek(SeekFrom::Start(header_size as u64 + offset))?;
    // let a = file.read_to_end(&mut vec![])?;
    // println!("{}  {}  {}", a, header_size as u64 + offset, offset);

    // iterate over entries
    iterate_entries_err(&json, |val, path| {
        if val["offset"] != Value::Null {
            let offset = val.get("offset").unwrap().as_u64().unwrap();
            let size = val.get("size").unwrap().as_u64().unwrap();
            let mut buffer = vec![0u8; size as usize];
            file.seek(SeekFrom::Start(header_size as u64 + offset))?;
            file.read_exact(&mut buffer)?;
            if compressed {
                zstd::stream::copy_decode(&mut buffer.reader(), &mut fs::File::create(path)?)?;
            } else {
                fs::write(dest.join(path), buffer)?;
            };
        } else {
            let dir = dest.join(path);
            if !dir.exists() {
                fs::create_dir(dir)?;
            }
        }
        Ok(())
    })?;

    Ok(())
}

/// Extract a single file from an asar archive.
///
/// # Examples
///
/// ```no_run
/// match rasar::extract("myarchive.asar", "file.txt") {
/// 	Ok(()) => println!("Success!"),
/// 	Err(err) => panic!("This should not have happened!")
/// }
/// ```
pub fn extract_file(archive: &str, dest: &str) -> Result<(), Error> {
    let cwd = env::current_dir()?;
    let full_path = cwd.join(dest);
    let dest = cwd.join(Path::new(dest).file_name().unwrap());
    let mut file = File::open(archive)?;

    // read header
    let (header_size, json, compressed) = read_header(&mut file)?;

    // iterate over entries
    iterate_entries_err(&json, |val, path| {
        if cwd.join(path) == full_path {
            let offset = val.get("offset").unwrap().as_u64().unwrap();
            let size = val.get("size").unwrap().as_u64().unwrap();
            let mut buffer = vec![0u8; size as usize];
            file.seek(SeekFrom::Start(header_size as u64 + offset))?;
            file.read_exact(&mut buffer)?;
            if compressed {
                zstd::stream::copy_decode(&mut buffer.reader(), &mut fs::File::create(path)?)?;
            } else {
                fs::write(dest.join(path), buffer)?;
            };
        }
        Ok(())
    })?;

    Ok(())
}
