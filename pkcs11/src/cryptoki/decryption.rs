use super::bindings::{
    CKR_FUNCTION_NOT_SUPPORTED, CK_BYTE_PTR, CK_MECHANISM_PTR, CK_OBJECT_HANDLE, CK_RV,
    CK_SESSION_HANDLE, CK_ULONG, CK_ULONG_PTR,
};

/// Initializes a decryption operation
///
/// # Arguments
///
/// `hSession` - the session’s handle
/// `pMechanism` - points to the decryption mechanism
/// `hKey` - the handle of the decryption key
#[no_mangle]
pub extern "C" fn C_DecryptInit(
    hSession: CK_SESSION_HANDLE,
    pMechanism: CK_MECHANISM_PTR,
    hKey: CK_OBJECT_HANDLE,
) -> CK_RV {
    CKR_FUNCTION_NOT_SUPPORTED as CK_RV
}

/// Decrypts encrypted data in a single part
///
/// # Arguments
///
/// * `hSession` - the session’s handle
/// * `pEncryptedData` - points to the encrypted data
/// * `ulEncryptedDataLen` - the length of the encrypted data
/// * `pData` - points to the location that receives the recovered data
/// * `pulDataLen` - points to the location that holds the length of the recovered data

#[no_mangle]
pub extern "C" fn C_Decrypt(
    hSession: CK_SESSION_HANDLE,
    pEncryptedData: CK_BYTE_PTR,
    ulEncryptedDataLen: CK_ULONG,
    pData: CK_BYTE_PTR,
    pulDataLen: CK_ULONG_PTR,
) -> CK_RV {
    CKR_FUNCTION_NOT_SUPPORTED as CK_RV
}
