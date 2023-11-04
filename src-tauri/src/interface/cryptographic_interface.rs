use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Serialize, Deserialize, Type, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub(crate) enum CryptographicInterface {
    Pcsc,
    Cryptoki,
    Webauthn,
}
