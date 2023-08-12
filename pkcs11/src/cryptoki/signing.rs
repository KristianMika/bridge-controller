use std::ptr;

use crate::{state::session::session::Signer, STATE};

use super::bindings::{
    CKR_ARGUMENTS_BAD, CKR_CRYPTOKI_NOT_INITIALIZED, CKR_FUNCTION_FAILED, CKR_GENERAL_ERROR,
    CKR_OBJECT_HANDLE_INVALID, CKR_OK, CKR_OPERATION_NOT_INITIALIZED, CKR_SESSION_HANDLE_INVALID,
    CK_BYTE_PTR, CK_MECHANISM_PTR, CK_OBJECT_HANDLE, CK_RV, CK_SESSION_HANDLE, CK_ULONG,
    CK_ULONG_PTR,
};

/// Initializes a signature operation, where the signature is an appendix to the data
///
/// # Arguments
///
/// * `hSession` - the session’s handle
/// * `pMechanism` - points to the signature mechanism
/// * `hKey` - handle of the signature key
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn C_SignInit(
    hSession: CK_SESSION_HANDLE,
    pMechanism: CK_MECHANISM_PTR,
    hKey: CK_OBJECT_HANDLE,
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
    let Some(signing_key) = session.get_object(hKey) else {
        return CKR_OBJECT_HANDLE_INVALID as CK_RV;
    };

    session.set_signer(Signer::new(signing_key));

    CKR_OK as CK_RV
}

/// Signs data in a single part, where the signature is an appendix to the data
///
/// # Arguments
///
/// * `hSession` - the session’s handle
/// * `pData` - points to the data
/// * `ulDataLen` - the length of the data
/// * `pSignature` - points to the location that receives the signature
/// * `pulSignatureLen` - points to the location that holds the length of the signature
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn C_Sign(
    hSession: CK_SESSION_HANDLE,
    pData: CK_BYTE_PTR,
    ulDataLen: CK_ULONG,
    pSignature: CK_BYTE_PTR,
    pulSignatureLen: CK_ULONG_PTR,
) -> CK_RV {
    // TODO: refactor to avoid multiple mut references
    if pulSignatureLen.is_null() {
        return CKR_ARGUMENTS_BAD as CK_RV;
    }
    let mut signer_ = None;
    {
        let Ok(state) = STATE.read() else  {
            return CKR_GENERAL_ERROR as CK_RV;
        };
        let Some( state) = state.as_ref() else {
            return CKR_CRYPTOKI_NOT_INITIALIZED as CK_RV;
        };
        let Some(session) = state.get_session(&hSession) else{
            return CKR_SESSION_HANDLE_INVALID as CK_RV;
        };
        let Some(signer) = session.get_signer() else {
            return CKR_OPERATION_NOT_INITIALIZED as CK_RV;
        };
        signer_ = Some(signer);
    }
    let mut signer = signer_.unwrap();
    if signer.response.is_none() {
        // response not stored from the previous call, send the request
        let pubkey = signer.key.value.read().unwrap().get_data();

        let mut auth_data = Vec::with_capacity(ulDataLen as usize);
        unsafe {
            ptr::copy(pData, auth_data.as_mut_ptr(), ulDataLen as usize);
            auth_data.set_len(ulDataLen as usize);
        }

        let mut response_ = None;
        {
            let Ok(mut state) = STATE.write() else  {
                return CKR_GENERAL_ERROR as CK_RV;
            };
            let Some( state) = state.as_mut() else {
                return CKR_CRYPTOKI_NOT_INITIALIZED as CK_RV;
            };

            let Ok(Some(response)) = state.send_signing_request_wait_for_response(pubkey, auth_data) else {
                return CKR_FUNCTION_FAILED as CK_RV;
            };
            response_ = Some(response);
        }
        let Ok(mut state) = STATE.write() else  {
            return CKR_GENERAL_ERROR as CK_RV;
        };
        let Some( state) = state.as_mut() else {
            return CKR_CRYPTOKI_NOT_INITIALIZED as CK_RV;
        };
        let Some(mut session) = state.get_session_mut(&hSession) else{
            return CKR_SESSION_HANDLE_INVALID as CK_RV;
        };
        let response = response_.unwrap();
        session.store_signing_response(response.clone());
        signer.response = Some(response);
    }

    let Some(response) = signer.response.as_ref() else {panic!("Shouldn't happen");};
    unsafe {
        *pulSignatureLen = response.len() as CK_ULONG;
    }

    if !pSignature.is_null() {
        unsafe {
            ptr::copy(response.as_ptr(), pSignature, response.len());
        }
    }

    CKR_OK as CK_RV
}
