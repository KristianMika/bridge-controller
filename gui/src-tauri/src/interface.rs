use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Serialize, Deserialize, Type)]
pub(crate) enum CryptographicInterface {
    Pcsc,
    Cryptoki,
    Webauthn,
}
