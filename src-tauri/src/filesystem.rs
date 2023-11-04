#[cfg(not(debug_assertions))]
use std::fs::File;

use std::{
    error::Error,
    fs::{self, copy},
    path::{Path, PathBuf},
};

use home::home_dir;

static CONTROLLER_DIRECTORY_NAME: &str = ".bridge-controller";

#[derive(Clone)]
pub(crate) struct FileSystem {}

impl FileSystem {
    // TODO: custom error
    fn get_controller_directory(&self) -> Result<PathBuf, Box<dyn Error>> {
        let home_directory = home_dir().expect("Couldn't get home directory");
        Ok(home_directory.join(CONTROLLER_DIRECTORY_NAME))
    }

    fn get_certificate_directory(&self) -> Result<PathBuf, Box<dyn Error>> {
        Ok(Path::new(&self.get_controller_directory()?).join("certificates"))
    }

    fn encode_hostname(&self, hostname: &str) -> String {
        hostname.replace('.', "_")
    }
    pub(crate) fn get_certificate_filepath(
        &self,
        hostname: &str,
    ) -> Result<PathBuf, Box<dyn Error>> {
        let encoded_hostname = self.encode_hostname(hostname);
        let certificate_filename = format!("{}_certificate.pem", encoded_hostname);
        Ok(self.get_certificate_directory()?.join(certificate_filename))
    }

    pub(crate) fn copy_cerrtificate(
        &self,
        certificate_path: &str,
        hostname: &str,
    ) -> Result<u64, Box<dyn Error>> {
        let destination_filepath = self.get_certificate_filepath(hostname)?;
        Ok(copy(certificate_path, destination_filepath)?)
    }

    pub(crate) fn get_db_filepath(&self, db_filename: &str) -> Result<PathBuf, Box<dyn Error>> {
        Ok(self.get_controller_directory()?.join(db_filename))
    }

    fn ensure_controller_directory_exists(&self) -> Result<(), Box<dyn Error>> {
        let controller_directory = self.get_controller_directory()?;
        fs::create_dir_all(controller_directory)?;
        Ok(())
    }

    fn ensure_certificates_directory_exists(&self) -> Result<(), Box<dyn Error>> {
        let certificate_directory = self.get_certificate_directory()?;
        fs::create_dir_all(certificate_directory)?;
        Ok(())
    }

    pub(crate) fn ensure_controller_directory_structure_exists(
        &self,
    ) -> Result<(), Box<dyn Error>> {
        self.ensure_controller_directory_exists()?;
        self.ensure_certificates_directory_exists()
    }

    #[cfg(not(debug_assertions))]
    pub(crate) fn get_log_file(&self) -> Result<File, Box<dyn Error>> {
        let filepath = self.get_controller_directory()?.join("logs.txt");
        Ok(File::options().create(true).append(true).open(filepath)?)
    }
}
