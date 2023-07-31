use lazy_static::__Deref;

use crate::STATE;

use super::bindings::{
    CKR_ARGUMENTS_BAD, CKR_CRYPTOKI_NOT_INITIALIZED, CKR_GENERAL_ERROR, CKR_OK, CK_FLAGS,
    CK_NOTIFY, CK_RV, CK_SESSION_HANDLE, CK_SESSION_HANDLE_PTR, CK_SLOT_ID, CK_ULONG, CK_USER_TYPE,
    CK_UTF8CHAR_PTR, CK_VOID_PTR,
};

/// Opens a session between an application and a token in a particular slot
///
/// # Arguments
///
/// * `slotID` - the slot’s ID
/// * `flags` - indicates the type of session
/// * `pApplication` - an application-defined pointer to be passed to the notification callback
/// * `Notify` - the address of the notification callback function
/// * `phSession` - points to the location that receives the handle for the new session
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn C_OpenSession(
    slotID: CK_SLOT_ID,
    flags: CK_FLAGS,
    pApplication: CK_VOID_PTR,
    Notify: CK_NOTIFY,
    phSession: CK_SESSION_HANDLE_PTR,
) -> CK_RV {
    // TODO: finish implementation
    if phSession.is_null() {
        return CKR_ARGUMENTS_BAD as CK_RV;
    }

    let Ok(mut state) = STATE.write() else  {
        return CKR_GENERAL_ERROR as CK_RV;
   };
    let Some( state) = state.as_mut() else {
        return CKR_CRYPTOKI_NOT_INITIALIZED as CK_RV;
    };

    let session_handle = state.create_session();
    unsafe {
        *phSession = session_handle;
    }
    CKR_OK as CK_RV
}

/// Closes a session between an application and a token
///
/// # Arguments
///
/// * `hSession` - the session’s handle
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn C_CloseSession(hSession: CK_SESSION_HANDLE) -> CK_RV {
    let Ok(mut state) = STATE.write() else  {
        return CKR_GENERAL_ERROR as CK_RV;
    };
    let Some( state) = state.as_mut() else {
        return CKR_CRYPTOKI_NOT_INITIALIZED as CK_RV;
    };

    // TODO: check if session exists
    // if !state.contains_key(&hSession) {
    //     return CKR_SESSION_HANDLE_INVALID as CK_RV;
    // }
    state.close_session(&hSession);

    CKR_OK as CK_RV
}

/// Logs a user into a token
///
/// # Arguments
///
/// `hSession` - a session handle
/// `userType` - the user type
/// `pPin` - points to the user’s PIN
/// `ulPinLen` - the length of the PIN
#[no_mangle]
pub extern "C" fn C_Login(
    hSession: CK_SESSION_HANDLE,
    userType: CK_USER_TYPE,
    pPin: CK_UTF8CHAR_PTR,
    ulPinLen: CK_ULONG,
) -> CK_RV {
    unimplemented!()
}

/// Logs a user out from a token
///
/// # Arguments
///
/// * `hSession` - the session’s handle
#[no_mangle]
pub extern "C" fn C_Logout(hSession: CK_SESSION_HANDLE) -> CK_RV {
    unimplemented!()
}
