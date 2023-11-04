use serde::Serialize;

#[derive(Serialize, Debug)]
pub(super) struct CertificateResponse {
    certificate_path: Option<String>,
}

impl CertificateResponse {
    pub(crate) fn new(certificate_path: Option<String>) -> Self {
        Self { certificate_path }
    }
}
