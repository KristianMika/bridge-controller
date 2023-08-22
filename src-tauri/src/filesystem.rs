use std::{
    error::Error,
    fs::copy,
    io,
    path::{Path, PathBuf},
};

pub(crate) struct FileSystem {}

impl FileSystem {
    // TODO: custom error
    fn get_controller_directory(&self) -> Result<PathBuf, Box<dyn Error>> {
        // TODO
        Ok(PathBuf::from("/home/kiko/Desktop/bridge"))
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
}
