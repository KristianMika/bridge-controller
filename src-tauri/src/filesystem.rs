use std::{
    error::Error,
    fs::{self, copy},
    io,
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

    fn encode_url(&self, url: &str) -> String {
        url.replace(".", "_")
    }
    pub(crate) fn get_certificate_filepath(&self, url: &str) -> Result<PathBuf, Box<dyn Error>> {
        let encoded_url = self.encode_url(url);
        let certificate_filename = format!("{}_certificate.pem", encoded_url);
        Ok(self
            .get_certificate_directory()?
            .join(&certificate_filename))
    }

    pub(crate) fn copy_cerrtificate(
        &self,
        certificate_path: &str,
        url: &str,
    ) -> Result<u64, io::Error> {
        let destination_filepath = self.get_certificate_filepath(url).unwrap();
        copy(certificate_path, destination_filepath)
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
}
