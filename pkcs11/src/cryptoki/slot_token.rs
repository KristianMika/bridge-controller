use super::bindings::{
    CKR_ARGUMENTS_BAD, CKR_BUFFER_TOO_SMALL, CKR_OK, CK_BBOOL, CK_RV, CK_SLOT_ID, CK_SLOT_ID_PTR,
    CK_TOKEN_INFO_PTR, CK_ULONG_PTR,
};

/// Used to obtain a list of slots in the system
///
/// # Arguments
///
/// * `tokenPresent` - indicates whether the list obtained includes only those slots with a token present, or all slots
/// * `pSlotList` - points to the buffer for the slot list
/// * `pulCount` -  points to the location that receives the number of slots
#[no_mangle]
pub extern "C" fn C_GetSlotList(
    tokenPresent: CK_BBOOL,
    pSlotList: CK_SLOT_ID_PTR,
    pulCount: CK_ULONG_PTR,
) -> CK_RV {
    if pulCount.is_null() {
        return CKR_ARGUMENTS_BAD as CK_RV;
    }
    let slot_length = 0; // TODO
    if pSlotList.is_null() {
        unsafe {
            *pulCount = slot_length;
        }
        return CKR_OK as CK_RV;
    }
    if unsafe { *pulCount } < slot_length {
        return CKR_BUFFER_TOO_SMALL as CK_RV;
    }
    // TODO: set the slot list based on `tokenPresent`
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
