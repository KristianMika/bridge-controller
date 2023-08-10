use crate::cryptoki::bindings::CK_BBOOL;

use super::template::Template;

// TODO: consider using bool at this lvl of abstraction
struct KeyObject {
    is_sensitive: CK_BBOOL,
    supports_encryption: CK_BBOOL,
    supports_decryption: CK_BBOOL,
    supports_signatures: CK_BBOOL,
    supports_verification: CK_BBOOL,
    supports_wrapping: CK_BBOOL,
    supports_unwrapping: CK_BBOOL,
    is_exctractable: CK_BBOOL,
    is_always_sensitive: CK_BBOOL,
    is_never_exctractable: CK_BBOOL,
    key_checksum: Vec<u8>,
    only_wrap_with_trusted_key: CK_BBOOL,
    is_key_trusted: CK_BBOOL,
    wrap_template: Option<Template>,
    unwrap_template: Option<Template>,
}
