use serde::Deserialize;
use specta::Type;

/// Represents an interface that a process can be spawned for.
#[derive(Eq, Hash, PartialEq, Deserialize, Type, Debug, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub(crate) enum CreatableInterface {
    Pcsc,
    Webauthn,
}
