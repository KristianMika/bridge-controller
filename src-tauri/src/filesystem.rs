mod filesystem_error;

#[cfg(not(debug_assertions))]
use std::fs::File;

use self::filesystem_error::FilesystemError;
use home::home_dir;
use std::{
    fs::{self, copy},
    io,
    path::{Path, PathBuf},
};

static CONTROLLER_DIRECTORY_NAME: &str = ".bridge-controller";

/// Manages all interactions with the filesystem
/// (directory creation, file creation, certificate storage and retrieval, etc.)
#[derive(Clone)]
pub(crate) struct FileSystem {}

impl FileSystem {
    fn get_controller_directory(&self) -> Result<PathBuf, FilesystemError> {
        let home_directory = match home_dir() {
            Some(home_directory) => home_directory,
            None => return Err(FilesystemError::HomeDirectoryError),
        };
        Ok(home_directory.join(CONTROLLER_DIRECTORY_NAME))
    }

    fn get_certificate_directory(&self) -> Result<PathBuf, FilesystemError> {
        Ok(Path::new(&self.get_controller_directory()?).join("certificates"))
    }

    fn encode_hostname(&self, hostname: &str) -> String {
        hostname.replace('.', "_")
    }

    /// Constructs a filepath for a certificate given a hostname. The path is of form [certificate directory]/hostnamecertificate.pem.
    /// Returning a valid path doesn't mean the certificate exists. To get an already present certificate, use `get_certificate_filepath`
    ///
    /// # Arguments
    ///
    /// * `hostname` - The hostname of the communicator.
    pub(crate) fn construct_certificate_filepath(
        &self,
        hostname: &str,
    ) -> Result<PathBuf, FilesystemError> {
        let encoded_hostname = self.encode_hostname(hostname);
        let certificate_filename = format!("{}_certificate.pem", encoded_hostname);
        Ok(self.get_certificate_directory()?.join(certificate_filename))
    }

    /// Gets a certificate filepath for a given hostname. If the certificate doesn't exist, returns `None`.
    ///
    /// # Arguments
    ///
    /// * `hostname` - The hostname of the communicator for which the certificate should be returned
    pub(crate) fn get_certificate_filepath(
        &self,
        hostname: &str,
    ) -> Result<Option<PathBuf>, FilesystemError> {
        let certificate_filepath = self.construct_certificate_filepath(hostname)?;
        if !certificate_filepath.exists() {
            return Ok(None);
        }
        Ok(Some(certificate_filepath))
    }

    pub(crate) fn copy_cerrtificate(
        &self,
        certificate_path: &str,
        hostname: &str,
    ) -> Result<u64, FilesystemError> {
        let destination_filepath = self.construct_certificate_filepath(hostname)?;
        Ok(copy(certificate_path, destination_filepath)?)
    }

    pub(crate) fn get_db_filepath(&self, db_filename: &str) -> Result<PathBuf, FilesystemError> {
        Ok(self.get_controller_directory()?.join(db_filename))
    }

    fn ensure_directory_exists(&self, directory: &Path) -> Result<(), io::Error> {
        fs::create_dir_all(directory)?;
        Ok(())
    }

    pub(crate) fn ensure_controller_directory_structure_exists(
        &self,
    ) -> Result<(), FilesystemError> {
        let controller_directory = self.get_controller_directory()?;
        self.ensure_directory_exists(&controller_directory)?;

        let certificate_directory = self.get_certificate_directory()?;
        self.ensure_directory_exists(&certificate_directory)?;
        Ok(())
    }

    #[cfg(not(debug_assertions))]
    pub(crate) fn get_log_file(&self) -> Result<File, FilesystemError> {
        let filepath = self.get_controller_directory()?.join("logs.txt");
        Ok(File::options().create(true).append(true).open(filepath)?)
    }
}
