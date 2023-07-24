use openssl::hash::Hasher;

/// Holds the current state of PKCS#11 lib
#[derive(Default)]
pub struct Pkcs11State {
    /// Holds the object managed by functions C_Digest*
    hasher: Option<Hasher>,
}

impl Pkcs11State {
    pub fn get_hasher(&mut self) -> &Option<Hasher> {
        &self.hasher
    }

    pub fn set_hasher(&mut self, hasher: Hasher) {
        self.hasher = Some(hasher)
    }
}
