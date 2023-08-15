use std::{
    ptr,
    sync::{Arc, RwLock},
};

use lazy_static::__Deref;

use crate::{communicator::Group, state::token::MeesignToken, STATE};

use super::bindings::{
    CKR_ARGUMENTS_BAD, CKR_BUFFER_TOO_SMALL, CKR_CRYPTOKI_NOT_INITIALIZED, CKR_GENERAL_ERROR,
    CKR_OK, CKR_SLOT_ID_INVALID, CKR_TOKEN_NOT_PRESENT, CK_BBOOL, CK_RV, CK_SLOT_ID,
    CK_SLOT_ID_PTR, CK_SLOT_INFO_PTR, CK_TOKEN_INFO_PTR, CK_ULONG, CK_ULONG_PTR,
};

/// Used to obtain a list of slots in the system
///
/// # Arguments
///
/// * `tokenPresent` - indicates whether the list obtained includes only those slots with a token present, or all slots
/// * `pSlotList` - points to the buffer for the slot list
/// * `pulCount` -  points to the location that receives the number of slots
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn C_GetSlotList(
    _tokenPresent: CK_BBOOL,
    pSlotList: CK_SLOT_ID_PTR,
    pulCount: CK_ULONG_PTR,
) -> CK_RV {
    if pulCount.is_null() {
        return CKR_ARGUMENTS_BAD as CK_RV;
    }
    let Ok(mut state) = STATE.write() else  {
        return CKR_GENERAL_ERROR as CK_RV;
    };

    let Some(state) = state.as_mut() else {
        return CKR_CRYPTOKI_NOT_INITIALIZED as CK_RV;
    };

    let groups = state.get_groups_blocking().unwrap();
    let slot_length = groups.len();

    if pSlotList.is_null() {
        unsafe {
            *pulCount = slot_length as CK_ULONG;
        }
        return CKR_OK as CK_RV;
    }
    if unsafe { *pulCount } < slot_length as CK_ULONG {
        return CKR_BUFFER_TOO_SMALL as CK_RV;
    }

    let slot_list: Vec<CK_SLOT_ID> = groups
        .into_iter()
        .map(|group: Group| group.into())
        .map(|token: MeesignToken| Arc::new(RwLock::new(token)))
        .map(|token| state.insert_token(token))
        .collect();

    unsafe {
        ptr::copy(slot_list.as_ptr(), pSlotList, slot_length);
    }
    CKR_OK as CK_RV
}

/// Obtains information about a particular token in the system
///
/// # Arguments
///
/// * `slotID` - the ID of the token’s slot
/// * `pInfo` - points to the location that receives the token information
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn C_GetTokenInfo(slotID: CK_SLOT_ID, pInfo: CK_TOKEN_INFO_PTR) -> CK_RV {
    if pInfo.is_null() {
        return CKR_ARGUMENTS_BAD as CK_RV;
    }
    let Ok(state) = STATE.read() else  {
        return CKR_GENERAL_ERROR as CK_RV;
   };

    let Some(state) = state.deref() else {
        return CKR_CRYPTOKI_NOT_INITIALIZED as CK_RV;
    };

    match state.get_token_info(&slotID) {
        Some(token_info) => unsafe { *pInfo = token_info },
        None => return CKR_TOKEN_NOT_PRESENT as CK_RV,
    }
    CKR_OK as CK_RV
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn C_GetSlotInfo(slotID: CK_SLOT_ID, pInfo: CK_SLOT_INFO_PTR) -> CK_RV {
    if pInfo.is_null() {
        return CKR_ARGUMENTS_BAD as CK_RV;
    }
    let Ok(state) = STATE.read() else  {
        return CKR_GENERAL_ERROR as CK_RV;
    };
    let Some(state) = state.deref() else {
        return CKR_CRYPTOKI_NOT_INITIALIZED as CK_RV;
    };
    let Some(token) = state.get_token(&slotID) else {
        return CKR_SLOT_ID_INVALID as CK_RV;
    };
    let slot_info = token.read().unwrap().get_slot_info();

    unsafe {
        *pInfo = slot_info;
    }

    CKR_OK as CK_RV
}
