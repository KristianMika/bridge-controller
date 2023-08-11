use aes::{
    cipher::{generic_array::GenericArray, KeyInit},
    Aes128,
};

use crate::STATE;

use super::bindings::{
    CKM_AES_ECB, CKR_ARGUMENTS_BAD, CKR_CRYPTOKI_NOT_INITIALIZED, CKR_FUNCTION_NOT_SUPPORTED,
    CKR_GENERAL_ERROR, CKR_KEY_HANDLE_INVALID, CKR_MECHANISM_INVALID, CKR_OK,
    CKR_SESSION_HANDLE_INVALID, CK_BYTE_PTR, CK_MECHANISM_PTR, CK_OBJECT_HANDLE, CK_RV,
    CK_SESSION_HANDLE, CK_ULONG, CK_ULONG_PTR,
};

/// Initializes an encryption operation
///
/// # Arguments
///
/// * `hSession` - the session’s handle
/// * `pMechanism` - points to the encryption mechanism
/// * `hKey` - the handle of the encryption key
#[no_mangle]
pub extern "C" fn C_EncryptInit(
    hSession: CK_SESSION_HANDLE,
    pMechanism: CK_MECHANISM_PTR,
    hKey: CK_OBJECT_HANDLE,
) -> CK_RV {
    if pMechanism.is_null() {
        return CKR_ARGUMENTS_BAD as CK_RV;
    }
    let Ok(mut state) = STATE.write() else  {
        return CKR_GENERAL_ERROR as CK_RV;
    };
    let Some( state) = state.as_mut() else {
        return CKR_CRYPTOKI_NOT_INITIALIZED as CK_RV;
    };

    let Some(mut session) =  state.get_session_mut(&hSession) else {
            return CKR_SESSION_HANDLE_INVALID as CK_RV;
    };
    let mechanism = unsafe { *pMechanism };
    // todo: support other algos
    match mechanism.mechanism as u32 {
        CKM_AES_ECB => {}
        _ => return CKR_MECHANISM_INVALID as CK_RV,
    };
    let Some(key) = session.get_object(hKey) else {
        return CKR_KEY_HANDLE_INVALID as CK_RV;
    };
    let key = key.value.read().unwrap().get_data();
    let key = GenericArray::clone_from_slice(&key[0..16]);
    let encryptor = Aes128::new(&key);
    session.set_encryptor(encryptor);

    CKR_OK as CK_RV
}

/// Encrypts single-part data
///
/// # Arguments
///
/// * `hSession` - the session’s handle
/// * `pData` - points to the data
/// * `ulDataLen` - the length in bytes of the data
/// * `pEncryptedData` - points to the location that receives the encrypted data
/// * `pulEncryptedDataLen` - points to the location that holds the length in bytes of the encrypted data
#[no_mangle]
pub extern "C" fn C_Encrypt(
    hSession: CK_SESSION_HANDLE,
    pData: CK_BYTE_PTR,
    ulDataLen: CK_ULONG,
    pEncryptedData: CK_BYTE_PTR,
    pulEncryptedDataLen: CK_ULONG_PTR,
) -> CK_RV {
    CKR_FUNCTION_NOT_SUPPORTED as CK_RV
}

/// Continues a multiple-part encryption operation, processing another data part
///
/// # Arguments
///
/// * `hSession` - is the session’s handle
/// * `pPart` - points to the data part
/// * `ulPartLen` - the length of the data part
/// * `pEncryptedPart` - points to the location that receives the encrypted data part
/// * `pulEncryptedPartLen` - points to the location that holds the length in bytes of the encrypted data part
#[no_mangle]
pub extern "C" fn C_EncryptUpdate(
    hSession: CK_SESSION_HANDLE,
    pPart: CK_BYTE_PTR,
    ulPartLen: CK_ULONG,
    pEncryptedPart: CK_BYTE_PTR,
    pulEncryptedPartLen: CK_ULONG_PTR,
) -> CK_RV {
    CKR_FUNCTION_NOT_SUPPORTED as CK_RV
}

/// Finishes a multiple-part encryption operation
///
/// # Arguments
///
/// * `hSession` - the session’s handle
/// * `pLastEncryptedPart` - points to the location that receives the last encrypted data part, if any
/// * `pulLastEncryptedPartLen` - points to the location that holds the length of the last encrypted data part
#[no_mangle]
pub extern "C" fn C_EncryptFinal(
    hSession: CK_SESSION_HANDLE,
    pLastEncryptedPart: CK_BYTE_PTR,
    pulLastEncryptedPartLen: CK_ULONG_PTR,
) -> CK_RV {
    CKR_FUNCTION_NOT_SUPPORTED as CK_RV
}
