use openssl::hash::Hasher;

/// Holds the current state of PKCS#11 lib
#[derive(Default)]
pub(crate) struct Session {
    /// Holds the object managed by functions C_Digest*
    hasher: Option<Hasher>,
}

impl Session {
    pub fn get_hasher_mut(&mut self) -> Option<&mut Hasher> {
        self.hasher.as_mut()
    }

    pub fn set_hasher(&mut self, hasher: Hasher) {
        self.hasher = Some(hasher)
    }
}
