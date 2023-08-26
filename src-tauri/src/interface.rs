use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Serialize, Deserialize, Type)]
#[serde(rename_all = "lowercase")]
pub(crate) enum CryptographicInterface {
    Pcsc,
    Cryptoki,
    Webauthn,
}
