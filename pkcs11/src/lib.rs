extern crate libc;

mod state;
#[cfg(test)]
mod test;

use crate::{bindings::CKR_BUFFER_TOO_SMALL, state::Pkcs11State};
use bindings::{
    CKM_SHA256, CKM_SHA384, CKM_SHA512, CKM_SHA_1, CKR_ARGUMENTS_BAD, CKR_FUNCTION_FAILED,
    CKR_GENERAL_ERROR, CKR_HOST_MEMORY, CKR_OK, CKR_OPERATION_NOT_INITIALIZED,
    CKR_SESSION_HANDLE_INVALID, CK_ATTRIBUTE_PTR, CK_BBOOL, CK_BYTE_PTR, CK_FLAGS,
    CK_FUNCTION_LIST, CK_FUNCTION_LIST_PTR_PTR, CK_MECHANISM_PTR, CK_NOTIFY, CK_OBJECT_HANDLE,
    CK_OBJECT_HANDLE_PTR, CK_RV, CK_SESSION_HANDLE, CK_SESSION_HANDLE_PTR, CK_SLOT_ID,
    CK_SLOT_ID_PTR, CK_TOKEN_INFO_PTR, CK_ULONG, CK_ULONG_PTR, CK_USER_TYPE, CK_UTF8CHAR_PTR,
    CK_VERSION, CK_VOID_PTR,
};
use lazy_static::lazy_static;
use openssl::hash::{Hasher, MessageDigest};
use rand::Rng;
use std::{collections::HashMap, mem, ptr, sync::Mutex};

mod bindings;

lazy_static! {
    static ref STATE: Mutex<HashMap::<CK_SESSION_HANDLE, Pkcs11State>> = Mutex::new(HashMap::new());
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
        C_Finalize: None,
        C_GetInfo: None,
        C_GetFunctionList: None,
        C_GetSlotList: None,
        C_GetSlotInfo: None,
        C_GetTokenInfo: None,
        C_GetMechanismList: None,
        C_GetMechanismInfo: None,
        C_InitToken: None,
        C_InitPIN: None,
        C_SetPIN: None,
        C_OpenSession: None,
        C_CloseSession: Some(C_CloseSession),
        C_CloseAllSessions: None,
        C_GetSessionInfo: None,
        C_GetOperationState: None,
        C_SetOperationState: None,
        C_Login: None,
        C_Logout: None,
        C_CreateObject: None,
        C_CopyObject: None,
        C_DestroyObject: None,
        C_GetObjectSize: None,
        C_GetAttributeValue: None,
        C_SetAttributeValue: None,
        C_FindObjectsInit: None,
        C_FindObjects: None,
        C_FindObjectsFinal: None,
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

/// Initializes the Cryptoki library
///
/// # Arguments
///
/// * `pInitArgs` - either has the value NULL_PTR or points to a CK_C_INITIALIZE_ARGS structure containing information on how the library should deal with multi-threaded access
#[no_mangle]
pub extern "C" fn C_Initialize(pInitArgs: CK_VOID_PTR) -> CK_RV {
    // TODO: check later if some actions are required
    CKR_OK as CK_RV
}

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

    let mut rng = rand::thread_rng();
    let mut session_handle = rng.gen_range(0..CK_SESSION_HANDLE::MAX);

    let Ok(mut state) = STATE.lock() else  {
        return CKR_GENERAL_ERROR as CK_RV;
   };
    while state.contains_key(&session_handle) {
        session_handle = rng.gen_range(0..CK_SESSION_HANDLE::MAX);
    }

    state.insert(session_handle, Pkcs11State::default());
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
    let Ok(mut state) = STATE.lock() else  {
        return CKR_GENERAL_ERROR as CK_RV;
    };

    if !state.contains_key(&hSession) {
        return CKR_SESSION_HANDLE_INVALID as CK_RV;
    }

    state.remove(&hSession);
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

/// Initializes a search for token and session objects that match a template
///
/// # Arguments
///
/// * `hSession` - the session’s handle
/// * `pTemplate` - points to a search template that specifies the attribute values to match
/// * `ulCount` - the number of attributes in the search template
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

/// Creates an object
///
/// # Arguments
///
/// * `hSession` - session's handle
/// * `pTemplate` - points to the object’s template
/// * `ulCount` - the number of attributes in the template
/// * `phObject` - points to the location that receives the new object’s handle
#[no_mangle]
pub extern "C" fn C_CreateObject(
    hSession: CK_SESSION_HANDLE,
    pTemplate: CK_ATTRIBUTE_PTR,
    ulCount: CK_ULONG,
    phObject: CK_OBJECT_HANDLE_PTR,
) -> CK_RV {
    unimplemented!()
}

/// Destroys an object
///
/// # Arguments
///
/// * `hSession` - the session’s handle
/// * `hObject` - the object’s handle
#[no_mangle]
pub extern "C" fn C_DestroyObject(hSession: CK_SESSION_HANDLE, hObject: CK_OBJECT_HANDLE) -> CK_RV {
    unimplemented!()
}

/// Generates a public/private key pair, creating new key objects
///
/// # Arguments
///
/// * `hSession` - the session’s handle
/// * `pMechanism` - points to the key generation mechanism
/// * `pPublicKeyTemplate` - points to the template for the public key
/// * `ulPublicKeyAttributeCount` - the number of attributes in the public-key template
/// * `pPrivateKeyTemplate` - points to the template for the private key
/// * `ulPrivateKeyAttributeCount` - the number of attributes in the private-key template
/// * `phPublicKey` - points to the location that receives the handle of the new public key
/// * `phPrivateKey` - points to the location that receives the handle of the new private key
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

/// Initializes an encryption operation
///
/// # Arguments
///
/// * `hSession` - the session’s handle
/// * `pMechanism` - points to the encryption mechanism
/// * ` hKey` - the handle of the encryption key
#[no_mangle]
pub extern "C" fn C_EncryptInit(
    hSession: CK_SESSION_HANDLE,
    pMechanism: CK_MECHANISM_PTR,
    hKey: CK_OBJECT_HANDLE,
) -> CK_RV {
    unimplemented!()
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
    unimplemented!()
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
    unimplemented!()
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
    unimplemented!()
}

/// Initializes a message-digesting operation
///
/// # Arguments
///
/// * `hSession` - the session’s handle
/// * `pMechanism` - points to the digesting mechanism
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn C_DigestInit(hSession: CK_SESSION_HANDLE, pMechanism: CK_MECHANISM_PTR) -> CK_RV {
    if pMechanism.is_null() {
        return CKR_ARGUMENTS_BAD as CK_RV;
    }

    let digest = match unsafe { (*pMechanism).mechanism as u32 } {
        CKM_SHA_1 => MessageDigest::sha1(),
        CKM_SHA256 => MessageDigest::sha256(),
        CKM_SHA384 => MessageDigest::sha384(),
        CKM_SHA512 => MessageDigest::sha512(),
        _ => {
            return CKR_ARGUMENTS_BAD as CK_RV;
        }
    };
    let Ok(hasher) = Hasher::new(digest) else {
        return CKR_FUNCTION_FAILED as CK_RV;
    };
    let Ok(mut state) = STATE.lock() else  {
         return CKR_GENERAL_ERROR as CK_RV;
    };

    match state.get_mut(&hSession) {
        Some(session_state) => session_state.set_hasher(hasher),
        None => return CKR_SESSION_HANDLE_INVALID as CK_RV,
    }
    CKR_OK as CK_RV
}

/// Digests data in a single part
///
/// # Arguments
///
/// * `hSession` - the session’s handle
/// * `pData` - points to the data
/// * `ulDataLen` - the length of the data
/// * `pDigest` - points to the location that receives the message digest
/// * `pulDigestLen` - points to the location that holds the length of the message digest
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn C_Digest(
    hSession: CK_SESSION_HANDLE,
    pData: CK_BYTE_PTR,
    ulDataLen: CK_ULONG,
    pDigest: CK_BYTE_PTR,
    pulDigestLen: CK_ULONG_PTR,
) -> CK_RV {
    let Ok( mut state) = STATE.lock() else  {
        return CKR_GENERAL_ERROR as CK_RV;
   };

    let hasher = match state.get_mut(&hSession) {
        Some(session_state) => session_state.get_hasher_mut(),
        None => return CKR_SESSION_HANDLE_INVALID as CK_RV,
    };
    if hasher.is_none() {
        return CKR_OPERATION_NOT_INITIALIZED as CK_RV;
    }
    let hasher = hasher.unwrap();

    let data_buffer: &[u8] = unsafe { std::slice::from_raw_parts(pData, ulDataLen as usize) };

    if hasher.update(data_buffer).is_err() {
        // TODO: reset hasher state
        return CKR_FUNCTION_FAILED as CK_RV;
    }

    let Ok(digest) = hasher.finish() else {
        return CKR_FUNCTION_FAILED as CK_RV;
    };

    let digest = digest.to_vec();
    unsafe {
        ptr::copy(digest.as_ptr(), pDigest, digest.len());
        *pulDigestLen = digest.len() as u64;
    }
    CKR_OK as CK_RV
}

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
    unimplemented!()
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
    unimplemented!()
}

/// Wraps (i.e., encrypts) a private or secret key
///
/// # Arguments
///
/// * `hSession` - the session’s handle
/// * `pMechanism` - points to the wrapping mechanism
/// * `hWrappingKey` - the handle of the wrapping key
/// * `hKey` - the handle of the key to be wrapped
/// * `pWrappedKey` - points to the location that receives the wrapped key
/// * `pulWrappedKeyLen` - points to the location that receives the length of the wrapped key
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

/// Unwraps (i.e. decrypts) a wrapped key, creating a new private key or secret key object
///
/// # Arguments
///
/// * `hSession` - the session’s handle
/// * `pMechanism` - points to the unwrapping mechanism
/// * `hUnwrappingKey` - the handle of the unwrapping key
/// * `pWrappedKey` - points to the wrapped key
/// * `ulWrappedKeyLen` - the length of the wrapped key
/// * `pTemplate` - points to the template for the new key
/// * `ulAttributeCount` - the number of attributes in the template
/// * `phKey` - points to the location that receives the handle of the recovered key
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
