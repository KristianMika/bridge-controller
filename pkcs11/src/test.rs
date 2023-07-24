use std::array;

use openssl::{
    error::ErrorStack,
    hash::{Hasher, MessageDigest},
    sha::Sha256,
};

use crate::{
    bindings::{
        CKM_SHA256, CKR_ARGUMENTS_BAD, CKR_OK, CK_BYTE_PTR, CK_FUNCTION_LIST_PTR,
        CK_FUNCTION_LIST_PTR_PTR, CK_MECHANISM, CK_MECHANISM_PTR, CK_RV, CK_SESSION_HANDLE_PTR,
        CK_ULONG, CK_ULONG_PTR, CK_VOID_PTR, NULL_PTR,
    },
    C_CloseSession, C_Digest, C_DigestInit, C_GetFunctionList, C_OpenSession,
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

#[test]
fn given_valid_data_c_digest_produces_valid_hash() -> Result<(), ErrorStack> {
    let mut session_handle = 0;
    assert_eq!(
        CKR_OK as CK_RV,
        C_OpenSession(
            0,
            0,
            NULL_PTR as CK_VOID_PTR,
            None,
            &mut session_handle as CK_SESSION_HANDLE_PTR
        )
    );
    let mut digest_mechanism = CK_MECHANISM {
        mechanism: CKM_SHA256 as u64,
        pParameter: NULL_PTR as CK_VOID_PTR,
        ulParameterLen: 0,
    };

    assert_eq!(
        C_DigestInit(session_handle, &mut digest_mechanism as CK_MECHANISM_PTR),
        CKR_OK as CK_RV
    );

    let mut data: Vec<u8> = vec![1, 2, 3, 4, 5];
    let mut digest: Vec<u8> = vec![0; MessageDigest::sha256().size() + 1];
    let mut digest_len: CK_ULONG = 0;
    assert_eq!(
        C_Digest(
            session_handle,
            data.as_mut_ptr() as CK_BYTE_PTR,
            data.len() as u64,
            digest.as_mut_ptr() as CK_BYTE_PTR,
            &mut digest_len as CK_ULONG_PTR
        ),
        CKR_OK as CK_RV
    );
    let digest: Vec<u8> = digest.iter().cloned().take(digest_len as usize).collect();
    let mut hasher = Hasher::new(MessageDigest::sha256())?;
    hasher.update(&data)?;
    let target_digest = hasher.finish()?.to_vec();

    assert_eq!(target_digest, digest);

    assert_eq!(CKR_OK as CK_RV, C_CloseSession(session_handle));
    Ok(())
}
