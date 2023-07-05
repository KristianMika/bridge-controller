extern crate libc;

use bindings::{
    C_Finalize, CKR_HOST_MEMORY, CKR_OK, CK_ATTRIBUTE_PTR, CK_BBOOL, CK_BYTE_PTR, CK_FLAGS,
    CK_FUNCTION_LIST, CK_FUNCTION_LIST_PTR_PTR, CK_MECHANISM_PTR, CK_NOTIFY, CK_OBJECT_HANDLE,
    CK_OBJECT_HANDLE_PTR, CK_RV, CK_SESSION_HANDLE, CK_SESSION_HANDLE_PTR, CK_SLOT_ID,
    CK_SLOT_ID_PTR, CK_TOKEN_INFO_PTR, CK_ULONG, CK_ULONG_PTR, CK_USER_TYPE, CK_UTF8CHAR_PTR,
    CK_VERSION, CK_VOID_PTR,
};

use std::mem;
mod bindings;

/// Obtains a pointer to the Cryptoki library’s list of function pointers
///
/// # Arguments
///
/// * `ppFunctionList` - points to a value which will receive a pointer to the library’s CK_FUNCTION_LIST structure
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn C_GetFunctionList(ppFunctionList: CK_FUNCTION_LIST_PTR_PTR) -> CK_RV {
    let version = CK_VERSION { major: 0, minor: 1 };
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
        C_CreateObject: Some(C_CreateObject),
        C_CopyObject: None,
        C_DestroyObject: Some(C_DestroyObject),
        C_GetObjectSize: None,
        C_GetAttributeValue: Some(C_GetAttributeValue),
        C_SetAttributeValue: None,
        C_FindObjectsInit: Some(C_FindObjectsInit),
        C_FindObjects: Some(C_FindObjects),
        C_FindObjectsFinal: Some(C_FindObjectsFinal),
        C_EncryptInit: Some(C_EncryptInit),
        C_Encrypt: Some(C_Encrypt),
        C_EncryptUpdate: Some(C_EncryptUpdate),
        C_EncryptFinal: Some(C_EncryptFinal),
        C_DecryptInit: Some(C_DecryptInit),
        C_Decrypt: Some(C_Decrypt),
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
        C_GenerateKeyPair: Some(C_GenerateKeyPair),
        C_WrapKey: Some(C_WrapKey),
        C_UnwrapKey: Some(C_UnwrapKey),
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
            return CKR_HOST_MEMORY as u64;
        }
        **ppFunctionList = function_list;
    }
    CKR_OK as u64
}

#[no_mangle]
pub extern "C" fn C_Initialize(pInitArgs: CK_VOID_PTR) -> CK_RV {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn C_GetSlotList(
    tokenPresent: CK_BBOOL,
    pSlotList: CK_SLOT_ID_PTR,
    pulCount: CK_ULONG_PTR,
) -> CK_RV {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn C_GetTokenInfo(slotID: CK_SLOT_ID, pInfo: CK_TOKEN_INFO_PTR) -> CK_RV {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn C_OpenSession(
    slotID: CK_SLOT_ID,
    flags: CK_FLAGS,
    pApplication: CK_VOID_PTR,
    Notify: CK_NOTIFY,
    phSession: CK_SESSION_HANDLE_PTR,
) -> CK_RV {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn C_CloseSession(hSession: CK_SESSION_HANDLE) -> CK_RV {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn C_Login(
    hSession: CK_SESSION_HANDLE,
    userType: CK_USER_TYPE,
    pPin: CK_UTF8CHAR_PTR,
    ulPinLen: CK_ULONG,
) -> CK_RV {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn C_Logout(hSession: CK_SESSION_HANDLE) -> CK_RV {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn C_FindObjectsInit(
    hSession: CK_SESSION_HANDLE,
    pTemplate: CK_ATTRIBUTE_PTR,
    ulCount: CK_ULONG,
) -> CK_RV {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn C_FindObjects(
    hSession: CK_SESSION_HANDLE,
    phObject: CK_OBJECT_HANDLE_PTR,
    ulMaxObjectCount: CK_ULONG,
    pulObjectCount: CK_ULONG_PTR,
) -> CK_RV {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn C_FindObjectsFinal(hSession: CK_SESSION_HANDLE) -> CK_RV {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn C_GetAttributeValue(
    hSession: CK_SESSION_HANDLE,
    hObject: CK_OBJECT_HANDLE,
    pTemplate: CK_ATTRIBUTE_PTR,
    ulCount: CK_ULONG,
) -> CK_RV {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn C_DestroyObject(hSession: CK_SESSION_HANDLE, hObject: CK_OBJECT_HANDLE) -> CK_RV {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn C_CreateObject(
    hSession: CK_SESSION_HANDLE,
    pTemplate: CK_ATTRIBUTE_PTR,
    ulCount: CK_ULONG,
    phObject: CK_OBJECT_HANDLE_PTR,
) -> CK_RV {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn C_GenerateKeyPair(
    hSession: CK_SESSION_HANDLE,
    pMechanism: CK_MECHANISM_PTR,
    pPublicKeyTemplate: CK_ATTRIBUTE_PTR,
    ulPublicKeyAttributeCount: CK_ULONG,
    pPrivateKeyTemplate: CK_ATTRIBUTE_PTR,
    ulPrivateKeyAttributeCount: CK_ULONG,
    phPublicKey: CK_OBJECT_HANDLE_PTR,
    phPrivateKey: CK_OBJECT_HANDLE_PTR,
) -> CK_RV {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn C_WrapKey(
    hSession: CK_SESSION_HANDLE,
    pMechanism: CK_MECHANISM_PTR,
    hWrappingKey: CK_OBJECT_HANDLE,
    hKey: CK_OBJECT_HANDLE,
    pWrappedKey: CK_BYTE_PTR,
    pulWrappedKeyLen: CK_ULONG_PTR,
) -> CK_RV {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn C_EncryptInit(
    hSession: CK_SESSION_HANDLE,
    pMechanism: CK_MECHANISM_PTR,
    hKey: CK_OBJECT_HANDLE,
) -> CK_RV {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn C_Encrypt(
    hSession: CK_SESSION_HANDLE,
    pData: CK_BYTE_PTR,
    ulDataLen: CK_ULONG,
    pEncryptedData: CK_BYTE_PTR,
    pulEncryptedDataLen: CK_ULONG_PTR,
) -> CK_RV {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn C_EncryptUpdate(
    hSession: CK_SESSION_HANDLE,
    pPart: CK_BYTE_PTR,
    ulPartLen: CK_ULONG,
    pEncryptedPart: CK_BYTE_PTR,
    pulEncryptedPartLen: CK_ULONG_PTR,
) -> CK_RV {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn C_EncryptFinal(
    hSession: CK_SESSION_HANDLE,
    pLastEncryptedPart: CK_BYTE_PTR,
    pulLastEncryptedPartLen: CK_ULONG_PTR,
) -> CK_RV {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn C_DigestInit(hSession: CK_SESSION_HANDLE, pMechanism: CK_MECHANISM_PTR) -> CK_RV {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn C_Digest(
    hSession: CK_SESSION_HANDLE,
    pData: CK_BYTE_PTR,
    ulDataLen: CK_ULONG,
    pDigest: CK_BYTE_PTR,
    pulDigestLen: CK_ULONG_PTR,
) -> CK_RV {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn C_DecryptInit(
    hSession: CK_SESSION_HANDLE,
    pMechanism: CK_MECHANISM_PTR,
    hKey: CK_OBJECT_HANDLE,
) -> CK_RV {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn C_Decrypt(
    hSession: CK_SESSION_HANDLE,
    pEncryptedData: CK_BYTE_PTR,
    ulEncryptedDataLen: CK_ULONG,
    pData: CK_BYTE_PTR,
    pulDataLen: CK_ULONG_PTR,
) -> CK_RV {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn C_UnwrapKey(
    hSession: CK_SESSION_HANDLE,
    pMechanism: CK_MECHANISM_PTR,
    hUnwrappingKey: CK_OBJECT_HANDLE,
    pWrappedKey: CK_BYTE_PTR,
    ulWrappedKeyLen: CK_ULONG,
    pTemplate: CK_ATTRIBUTE_PTR,
    ulAttributeCount: CK_ULONG,
    phKey: CK_OBJECT_HANDLE_PTR,
) -> CK_RV {
    unimplemented!()
}

#[cfg(test)]
mod test {
    use crate::{
        bindings::{CKR_OK, CK_FUNCTION_LIST, CK_FUNCTION_LIST_PTR},
        C_GetFunctionList,
    };

    #[test]
    fn c_get_function_list_returns_ckr_ok() {
        let mut funct_list_ptr: CK_FUNCTION_LIST_PTR = 0 as *mut CK_FUNCTION_LIST;
        let return_value = C_GetFunctionList(&mut funct_list_ptr);
        assert_eq!(
            return_value, CKR_OK as u64,
            "C_GetFunctionList didn't return CKR_OK",
        );
        assert!(
            !funct_list_ptr.is_null(),
            "C_GetFunctionList set null pointer"
        );
        unsafe { libc::free(funct_list_ptr as *mut libc::c_void) }
    }
}
