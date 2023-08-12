use std::{
    ptr,
    sync::{Arc, RwLock},
};

use rand::{rngs::OsRng, Rng};

use crate::{
    state::object::{
        private_key_object::PrivateKeyObject, public_key_object::PublicKeyObject,
        template::Template, CryptokiArc,
    },
    STATE,
};

use super::bindings::{
    CKM_AES_KEY_GEN, CKR_ARGUMENTS_BAD, CKR_CRYPTOKI_NOT_INITIALIZED, CKR_FUNCTION_NOT_SUPPORTED,
    CKR_GENERAL_ERROR, CKR_OK, CKR_SESSION_HANDLE_INVALID, CKR_TEMPLATE_INCOMPLETE, CK_ATTRIBUTE,
    CK_ATTRIBUTE_PTR, CK_BYTE_PTR, CK_MECHANISM_PTR, CK_OBJECT_HANDLE, CK_OBJECT_HANDLE_PTR, CK_RV,
    CK_SESSION_HANDLE, CK_ULONG, CK_ULONG_PTR,
};

/// Generates a secret key or set of domain parameters, creating a new object
///
/// # Arguments
///
/// * `hSession` - the session’s handle
/// * `pMechanism` - points to the generation mechanism
/// * `pTemplate` - points to the template for the new key or set of domain parameters
/// * `ulCount` - the number of attributes in the template
/// * `phKey` - points to the location that receives the handle of the new key or set of domain parameters
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn C_GenerateKey(
    hSession: CK_SESSION_HANDLE,
    pMechanism: CK_MECHANISM_PTR,
    pTemplate: CK_ATTRIBUTE_PTR,
    ulCount: CK_ULONG,
    phKey: CK_OBJECT_HANDLE_PTR,
) -> CK_RV {
    if pMechanism.is_null() || pTemplate.is_null() || phKey.is_null() {
        return CKR_ARGUMENTS_BAD as CK_RV;
    }
    let Ok(mut state) = STATE.write() else  {
        return CKR_GENERAL_ERROR as CK_RV;
    };
    let Some( state) = state.as_mut() else {
        return CKR_CRYPTOKI_NOT_INITIALIZED as CK_RV;
    };

    let mechanism = unsafe { *pMechanism };
    // todo: implement others
    if mechanism.mechanism as u32 != CKM_AES_KEY_GEN {
        return CKR_FUNCTION_NOT_SUPPORTED as CK_RV;
    }
    let mut template: Vec<CK_ATTRIBUTE> = Vec::with_capacity(ulCount as usize);
    unsafe {
        ptr::copy(pTemplate, template.as_mut_ptr(), ulCount as usize);
        template.set_len(ulCount as usize);
    }
    let template = Template::from(template);
    let Some(mut object):Option<CryptokiArc> = template.into() else {
        return CKR_TEMPLATE_INCOMPLETE as CK_RV;
    };

    let key: [u8; 32] = OsRng.gen();
    object.store_data(key.into());

    let return_code = match state.get_session_mut(&hSession) {
        Some(mut session) => {
            let handle = session.create_object(object);
            unsafe { *phKey = handle };
            CKR_OK as CK_RV
        }
        None => CKR_SESSION_HANDLE_INVALID as CK_RV,
    };

    return_code
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
    let Ok(mut state) = STATE.write() else  {
        return CKR_GENERAL_ERROR as CK_RV;
    };
    let Some( state) = state.as_mut() else {
        return CKR_CRYPTOKI_NOT_INITIALIZED as CK_RV;
    };

    let Some(mut session) = state.get_session_mut(&hSession) else{
       return CKR_SESSION_HANDLE_INVALID as CK_RV;
    };
    let token = session.get_token();
    let token = token.read().unwrap();
    let pubkey = token.get_public_key();
    let pubkey_object = PublicKeyObject::new(pubkey.into());
    let pubkey_handle = session.create_object(CryptokiArc {
        value: Arc::new(RwLock::new(pubkey_object)),
    });
    unsafe { *phPublicKey = pubkey_handle };
    let private_key = PrivateKeyObject::new(pubkey.into());
    let private_key_handle = session.create_object(CryptokiArc {
        value: Arc::new(RwLock::new(private_key)),
    });
    unsafe { *phPrivateKey = private_key_handle };

    CKR_OK as CK_RV
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
    if pulWrappedKeyLen.is_null() {
        return CKR_ARGUMENTS_BAD as CK_RV;
    }
    unsafe {
        *pulWrappedKeyLen = 8;
    }

    if pWrappedKey.is_null() {
        return CKR_OK as CK_RV;
    }
    let key_handle = hKey.to_le_bytes();
    unsafe {
        ptr::copy(key_handle.as_ptr(), pWrappedKey, key_handle.len());
    }

    CKR_OK as CK_RV
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
    if pWrappedKey.is_null() {
        return CKR_ARGUMENTS_BAD as CK_RV;
    }

    unsafe {
        ptr::copy(pWrappedKey, phKey as *mut u8, ulWrappedKeyLen as usize);
    }

    CKR_OK as CK_RV
}
