use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::cryptoki::bindings::{CK_SLOT_ID, CK_TOKEN_INFO};

use super::token::Token;

pub(crate) type TokenStore = Arc<RwLock<dyn Token>>;

// TODO: hide behind a trait
#[derive(Default)]
pub(crate) struct Slots {
    tokens: HashMap<CK_SLOT_ID, TokenStore>,
    counter: usize,
}

impl Slots {
    pub(crate) fn insert_token(&mut self, token: TokenStore) -> CK_SLOT_ID {
        self.counter += 1;
        self.tokens.insert(self.counter as CK_SLOT_ID, token);
        self.counter as CK_SLOT_ID
    }

    pub(crate) fn get_slot_list(&self) -> Vec<CK_SLOT_ID> {
        self.tokens.keys().cloned().collect()
    }

    pub(crate) fn new() -> Self {
        Self {
            tokens: HashMap::new(),
            counter: 0,
        }
    }

    pub(crate) fn get_token_info(&self, slot_id: &CK_SLOT_ID) -> Option<CK_TOKEN_INFO> {
        match self.tokens.get(slot_id) {
            Some(token) => Some(token.read().unwrap().get_token_info()),
            None => None,
        }
    }

    pub(crate) fn get_token(&self, slot_id: &CK_SLOT_ID) -> Option<TokenStore> {
        self.tokens.get(slot_id).cloned()
    }
}
