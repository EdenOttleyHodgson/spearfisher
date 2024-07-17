use std::ffi::OsString;
use std::fs::{DirEntry, FileType, Metadata};
use std::io;
use std::path::Path;

use serde::de::value;
use serde::ser::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
enum FileTypeEnum {
    File,
    Directory,
    Symlink,
}
impl From<FileType> for FileTypeEnum {
    fn from(value: FileType) -> Self {
        if value.is_dir() {
            Self::Directory
        } else if value.is_symlink() {
            Self::Symlink
        } else {
            Self::File
        }
    }
}

struct SerializableMetadata(Metadata);

struct SerdeWrapper<T>(T);

impl Serialize for SerdeWrapper<Metadata> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let metadata = &self.0;
        let mut state = serializer.serialize_struct("Metadata", 10)?;
        state.serialize_field("length", &metadata.len())?;
        state.serialize_field("readonly", &metadata.permissions().readonly())?;
        state.serialize_field("modified", &metadata.modified().ok())?;
        state.serialize_field("accessed", &metadata.accessed().ok())?;
        state.serialize_field("created", &metadata.created().ok())?;
        state.end()
    }
}

#[derive(Serialize)]
pub struct FileData {
    name: String,
    full_path: String,
    file_type: FileTypeEnum,
    metadata: SerdeWrapper<Metadata>,
}
impl TryFrom<DirEntry> for FileData {
    type Error = io::Error;
    fn try_from(value: DirEntry) -> Result<Self, Self::Error> {
        fn conv_error(_: OsString) -> io::Error {
            io::Error::new(
                io::ErrorKind::Other,
                "could not convert os string to string",
            )
        }
        let name = value.file_name().into_string().map_err(conv_error)?;
        let full_path = value
            .path()
            .into_os_string()
            .into_string()
            .map_err(conv_error)?;
        let file_type = FileTypeEnum::from(value.file_type()?);
        let metadata = SerdeWrapper(value.metadata()?);
        let data = FileData {
            name,
            full_path,
            file_type,
            metadata,
        };
        Ok(data)
    }
}

#[derive(Serialize, Deserialize)]
struct FileManagerData {}

#[cfg(test)]
mod tests {
    use super::*;
}
