use std::mem;

use super::{
    bindings::{
        CKR_ARGUMENTS_BAD, CKR_CRYPTOKI_NOT_INITIALIZED, CKR_GENERAL_ERROR, CKR_HOST_MEMORY,
        CKR_OK, CK_FUNCTION_LIST, CK_FUNCTION_LIST_PTR_PTR, CK_RV, CK_VERSION, CK_VOID_PTR,
    },
    message_digesting::{C_Digest, C_DigestInit},
    object_management::{C_FindObjects, C_FindObjectsFinal, C_FindObjectsInit},
    session_management::{C_CloseSession, C_Login, C_Logout, C_OpenSession},
    slot_token::{C_GetSlotList, C_GetTokenInfo},
};
use crate::{state::state::CryptokiState, STATE};

/// Initializes the Cryptoki library
///
/// # Arguments
///
/// * `pInitArgs` - either has the value NULL_PTR or points to a CK_C_INITIALIZE_ARGS structure containing information on how the library should deal with multi-threaded access
#[no_mangle]
pub extern "C" fn C_Initialize(pInitArgs: CK_VOID_PTR) -> CK_RV {
    // TODO: check later if some actions are required
    let Ok(mut state) = STATE.write() else  {
        return CKR_GENERAL_ERROR as CK_RV;
   };
    let _ = state.insert(CryptokiState::default());
    CKR_OK as CK_RV
}

/// The function is called to indicate that an application is finished with the Cryptoki library.
/// It should be the last Cryptoki call made by an application
///
/// # Arguments
///
/// * `pReserved` - reserved for future versions; for this version, it should be set to NULL_PTR
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn C_Finalize(pReserved: CK_VOID_PTR) -> CK_RV {
    if !pReserved.is_null() {
        return CKR_ARGUMENTS_BAD as CK_RV;
    }
    let Ok(mut state) = STATE.write() else  {
        return CKR_GENERAL_ERROR as CK_RV;
   };
    let Some(state) = state.as_mut() else {
       return CKR_CRYPTOKI_NOT_INITIALIZED as CK_RV;
   };

    state.finalize();

    CKR_OK as CK_RV
}

/// Obtains a pointer to the Cryptoki library’s list of function pointers
///
/// # Arguments
///
/// * `ppFunctionList` - points to a value which will receive a pointer to the library’s CK_FUNCTION_LIST structure
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn C_GetFunctionList(ppFunctionList: CK_FUNCTION_LIST_PTR_PTR) -> CK_RV {
    if ppFunctionList.is_null() {
        return CKR_ARGUMENTS_BAD as CK_RV;
    }
    let version = CK_VERSION { major: 0, minor: 1 };
    // TODO: add functions when implemented
    let function_list = CK_FUNCTION_LIST {
        version,
        C_Initialize: Some(C_Initialize),
        C_Finalize: Some(C_Finalize),
        C_GetInfo: None,
        C_GetFunctionList: Some(C_GetFunctionList),
        C_GetSlotList: Some(C_GetSlotList),
        C_GetSlotInfo: None,
        C_GetTokenInfo: Some(C_GetTokenInfo),
        C_GetMechanismList: None,
        C_GetMechanismInfo: None,
        C_InitToken: None,
        C_InitPIN: None,
        C_SetPIN: None,
        C_OpenSession: Some(C_OpenSession),
        C_CloseSession: Some(C_CloseSession),
        C_CloseAllSessions: None,
        C_GetSessionInfo: None,
        C_GetOperationState: None,
        C_SetOperationState: None,
        C_Login: Some(C_Login),
        C_Logout: Some(C_Logout),
        C_CreateObject: None,
        C_CopyObject: None,
        C_DestroyObject: None,
        C_GetObjectSize: None,
        C_GetAttributeValue: None,
        C_SetAttributeValue: None,
        C_FindObjectsInit: Some(C_FindObjectsInit),
        C_FindObjects: Some(C_FindObjects),
        C_FindObjectsFinal: Some(C_FindObjectsFinal),
        C_EncryptInit: None,
        C_Encrypt: None,
        C_EncryptUpdate: None,
        C_EncryptFinal: None,
        C_DecryptInit: None,
        C_Decrypt: None,
        C_DecryptUpdate: None,
        C_DecryptFinal: None,
        C_DigestInit: Some(C_DigestInit),
        C_Digest: Some(C_Digest),
        C_DigestUpdate: None,
        C_DigestKey: None,
        C_DigestFinal: None,
        C_SignInit: None,
        C_Sign: None,
        C_SignUpdate: None,
        C_SignFinal: None,
        C_SignRecoverInit: None,
        C_SignRecover: None,
        C_VerifyInit: None,
        C_Verify: None,
        C_VerifyUpdate: None,
        C_VerifyFinal: None,
        C_VerifyRecoverInit: None,
        C_VerifyRecover: None,
        C_DigestEncryptUpdate: None,
        C_DecryptDigestUpdate: None,
        C_SignEncryptUpdate: None,
        C_DecryptVerifyUpdate: None,
        C_GenerateKey: None,
        C_GenerateKeyPair: None,
        C_WrapKey: None,
        C_UnwrapKey: None,
        C_DeriveKey: None,
        C_SeedRandom: None,
        C_GenerateRandom: None,
        C_GetFunctionStatus: None,
        C_CancelFunction: None,
        C_WaitForSlotEvent: None,
    };

    unsafe {
        *ppFunctionList = libc::malloc(mem::size_of::<CK_FUNCTION_LIST>() as libc::size_t)
            as *mut CK_FUNCTION_LIST;
        if (*ppFunctionList).is_null() {
            return CKR_HOST_MEMORY as CK_RV;
        }
        **ppFunctionList = function_list;
    }
    CKR_OK as CK_RV
}

#[cfg(test)]
mod test {
    use crate::cryptoki::{
        bindings::{
            CKR_ARGUMENTS_BAD, CKR_OK, CK_FUNCTION_LIST_PTR, CK_FUNCTION_LIST_PTR_PTR, CK_RV,
        },
        general_purpose::C_GetFunctionList,
    };

    #[test]
    fn c_get_function_list_returns_ckr_ok() {
        let mut funct_list_ptr: CK_FUNCTION_LIST_PTR = 0 as CK_FUNCTION_LIST_PTR;
        let return_value = C_GetFunctionList(&mut funct_list_ptr);
        assert_eq!(
            return_value, CKR_OK as CK_RV,
            "C_GetFunctionList didn't return CKR_OK",
        );
        assert!(
            !funct_list_ptr.is_null(),
            "C_GetFunctionList set null pointer"
        );
        unsafe { libc::free(funct_list_ptr as *mut libc::c_void) }
    }

    #[test]
    fn given_nullptr_c_get_function_list_returns_ckr_arguments_bad() {
        let return_value = C_GetFunctionList(0 as CK_FUNCTION_LIST_PTR_PTR);
        assert_eq!(
            return_value, CKR_ARGUMENTS_BAD as CK_RV,
            "C_GetFunctionList didn't return CKR_ARGUMENTS_BAD",
        );
    }
}
