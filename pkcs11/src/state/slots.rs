use std::collections::HashMap;

use crate::cryptoki::bindings::{CK_SLOT_ID, CK_TOKEN_INFO};

use super::token::Token;

// TODO: hide behind a trait
#[derive(Default)]
pub(crate) struct Slots<T>
where
    T: Token,
{
    // TODO: allow dynamic dispatch to enable polymorphism
    tokens: HashMap<CK_SLOT_ID, T>,
    counter: usize,
}

impl<T> Slots<T>
where
    T: Token,
{
    pub(crate) fn insert_token(&mut self, token: T) -> CK_SLOT_ID {
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
            Some(token) => Some(token.get_token_info()),
            None => None,
        }
    }
}
