use serde::Deserialize;
use specta::Type;

#[derive(Eq, Hash, PartialEq, Deserialize, Type, Debug, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub(crate) enum CreatableInterface {
    Pcsc,
    Webauthn,
}
