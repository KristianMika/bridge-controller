use serde::{Deserialize, Serialize};
use specta::Type;

use super::CreatableInterface;

#[derive(Serialize, Deserialize, Type, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub(crate) enum CryptographicInterface {
    Pcsc,
    Cryptoki,
    Webauthn,
}

impl From<CreatableInterface> for CryptographicInterface {
    fn from(value: CreatableInterface) -> Self {
        match value {
            CreatableInterface::Pcsc => Self::Pcsc,
            CreatableInterface::Webauthn => Self::Webauthn,
        }
    }
}
