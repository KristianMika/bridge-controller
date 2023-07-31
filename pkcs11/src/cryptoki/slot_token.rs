use std::ptr;

use tokio::runtime::Runtime;

use crate::{communicator::GroupId, state::token::MeesignToken, STATE};

use super::bindings::{
    CKR_ARGUMENTS_BAD, CKR_BUFFER_TOO_SMALL, CKR_GENERAL_ERROR, CKR_OK, CK_BBOOL, CK_RV,
    CK_SLOT_ID, CK_SLOT_ID_PTR, CK_TOKEN_INFO_PTR, CK_ULONG, CK_ULONG_PTR,
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
        .map(|groupId: GroupId| MeesignToken::new(groupId))
        .map(|token: MeesignToken| state.insert_token(token))
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
pub extern "C" fn C_GetTokenInfo(slotID: CK_SLOT_ID, pInfo: CK_TOKEN_INFO_PTR) -> CK_RV {
    unimplemented!()
}
