use std::ffi::OsString;
use std::fs::{DirEntry, File, FileType, Metadata};
use std::io;
use std::path::{Path, PathBuf};

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
        let mut state = serializer.serialize_struct("metadata", 10)?;
        state.serialize_field("length", &metadata.len())?;
        state.serialize_field("readonly", &metadata.permissions().readonly())?;
        state.serialize_field("modified", &metadata.modified().ok())?;
        state.serialize_field("accessed", &metadata.accessed().ok())?;
        state.serialize_field("created", &metadata.created().ok())?;
        state.end()
    }
}
impl<'de> Deserialize<'de> for SerdeWrapper<Metadata> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        todo!()
    }
}
fn to_os_string_conv_error(_: OsString) -> io::Error {
    io::Error::new(
        io::ErrorKind::Other,
        "could not convert os string to string",
    )
}

#[derive(Serialize, Deserialize)]
pub struct FileData {
    name: String,
    full_path: String,
    file_type: FileTypeEnum,
    metadata: SerdeWrapper<Metadata>,
}
impl FileData {
    pub fn make_from_file(
        name: PathBuf,
        full_path: PathBuf,
        file: File,
    ) -> Result<FileData, io::Error> {
        let name = name
            .into_os_string()
            .into_string()
            .map_err(to_os_string_conv_error)?;
        let full_path = full_path
            .into_os_string()
            .into_string()
            .map_err(to_os_string_conv_error)?;
        let metadata = file.metadata()?;
        let file_type = FileTypeEnum::from(metadata.file_type());
        let data = FileData {
            name,
            full_path,
            file_type,
            metadata: SerdeWrapper(metadata),
        };
        Ok(data)
    }
}

impl TryFrom<DirEntry> for FileData {
    type Error = io::Error;
    fn try_from(value: DirEntry) -> Result<Self, Self::Error> {
        let name = value
            .file_name()
            .into_string()
            .map_err(to_os_string_conv_error)?;
        let full_path = value
            .path()
            .into_os_string()
            .into_string()
            .map_err(to_os_string_conv_error)?;
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

#[cfg(test)]
mod tests {
    use super::*;
}
