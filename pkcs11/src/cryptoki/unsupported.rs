#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]

use super::bindings::*;

/// Warning: Don't modify by hand, this file is generated
/// TODO: write a macro

#[no_mangle]
pub extern "C" fn C_GetInfo(pInfo: CK_INFO_PTR) -> CK_RV {
    CKR_FUNCTION_NOT_SUPPORTED as CK_RV
}

#[no_mangle]
pub extern "C" fn C_GetSlotInfo(slotID: CK_SLOT_ID, pInfo: CK_SLOT_INFO_PTR) -> CK_RV {
    CKR_FUNCTION_NOT_SUPPORTED as CK_RV
}

#[no_mangle]
pub extern "C" fn C_GetMechanismList(
    slotID: CK_SLOT_ID,
    pMechanismList: CK_MECHANISM_TYPE_PTR,
    pulCount: CK_ULONG_PTR,
) -> CK_RV {
    CKR_FUNCTION_NOT_SUPPORTED as CK_RV
}

#[no_mangle]
pub extern "C" fn C_GetMechanismInfo(
    slotID: CK_SLOT_ID,
    type_: CK_MECHANISM_TYPE,
    pInfo: CK_MECHANISM_INFO_PTR,
) -> CK_RV {
    CKR_FUNCTION_NOT_SUPPORTED as CK_RV
}

#[no_mangle]
pub extern "C" fn C_InitToken(
    slotID: CK_SLOT_ID,
    pPin: CK_UTF8CHAR_PTR,
    ulPinLen: CK_ULONG,
    pLabel: CK_UTF8CHAR_PTR,
) -> CK_RV {
    CKR_FUNCTION_NOT_SUPPORTED as CK_RV
}

#[no_mangle]
pub extern "C" fn C_InitPIN(
    hSession: CK_SESSION_HANDLE,
    pPin: CK_UTF8CHAR_PTR,
    ulPinLen: CK_ULONG,
) -> CK_RV {
    CKR_FUNCTION_NOT_SUPPORTED as CK_RV
}

#[no_mangle]
pub extern "C" fn C_SetPIN(
    hSession: CK_SESSION_HANDLE,
    pOldPin: CK_UTF8CHAR_PTR,
    ulOldLen: CK_ULONG,
    pNewPin: CK_UTF8CHAR_PTR,
    ulNewLen: CK_ULONG,
) -> CK_RV {
    CKR_FUNCTION_NOT_SUPPORTED as CK_RV
}

#[no_mangle]
pub extern "C" fn C_CloseAllSessions(slotID: CK_SLOT_ID) -> CK_RV {
    CKR_FUNCTION_NOT_SUPPORTED as CK_RV
}

#[no_mangle]
pub extern "C" fn C_GetSessionInfo(
    hSession: CK_SESSION_HANDLE,
    pInfo: CK_SESSION_INFO_PTR,
) -> CK_RV {
    CKR_FUNCTION_NOT_SUPPORTED as CK_RV
}

#[no_mangle]
pub extern "C" fn C_GetOperationState(
    hSession: CK_SESSION_HANDLE,
    pOperationState: CK_BYTE_PTR,
    pulOperationStateLen: CK_ULONG_PTR,
) -> CK_RV {
    CKR_FUNCTION_NOT_SUPPORTED as CK_RV
}

#[no_mangle]
pub extern "C" fn C_SetOperationState(
    hSession: CK_SESSION_HANDLE,
    pOperationState: CK_BYTE_PTR,
    ulOperationStateLen: CK_ULONG,
    hEncryptionKey: CK_OBJECT_HANDLE,
    hAuthenticationKey: CK_OBJECT_HANDLE,
) -> CK_RV {
    CKR_FUNCTION_NOT_SUPPORTED as CK_RV
}

#[no_mangle]
pub extern "C" fn C_CopyObject(
    hSession: CK_SESSION_HANDLE,
    hObject: CK_OBJECT_HANDLE,
    pTemplate: CK_ATTRIBUTE_PTR,
    ulCount: CK_ULONG,
    phNewObject: CK_OBJECT_HANDLE_PTR,
) -> CK_RV {
    CKR_FUNCTION_NOT_SUPPORTED as CK_RV
}

#[no_mangle]
pub extern "C" fn C_GetObjectSize(
    hSession: CK_SESSION_HANDLE,
    hObject: CK_OBJECT_HANDLE,
    pulSize: CK_ULONG_PTR,
) -> CK_RV {
    CKR_FUNCTION_NOT_SUPPORTED as CK_RV
}

#[no_mangle]
pub extern "C" fn C_SetAttributeValue(
    hSession: CK_SESSION_HANDLE,
    hObject: CK_OBJECT_HANDLE,
    pTemplate: CK_ATTRIBUTE_PTR,
    ulCount: CK_ULONG,
) -> CK_RV {
    CKR_FUNCTION_NOT_SUPPORTED as CK_RV
}

#[no_mangle]
pub extern "C" fn C_DecryptUpdate(
    hSession: CK_SESSION_HANDLE,
    pEncryptedPart: CK_BYTE_PTR,
    ulEncryptedPartLen: CK_ULONG,
    pPart: CK_BYTE_PTR,
    pulPartLen: CK_ULONG_PTR,
) -> CK_RV {
    CKR_FUNCTION_NOT_SUPPORTED as CK_RV
}

#[no_mangle]
pub extern "C" fn C_DecryptFinal(
    hSession: CK_SESSION_HANDLE,
    pLastPart: CK_BYTE_PTR,
    pulLastPartLen: CK_ULONG_PTR,
) -> CK_RV {
    CKR_FUNCTION_NOT_SUPPORTED as CK_RV
}

#[no_mangle]
pub extern "C" fn C_DigestUpdate(
    hSession: CK_SESSION_HANDLE,
    pPart: CK_BYTE_PTR,
    ulPartLen: CK_ULONG,
) -> CK_RV {
    CKR_FUNCTION_NOT_SUPPORTED as CK_RV
}

#[no_mangle]
pub extern "C" fn C_DigestKey(hSession: CK_SESSION_HANDLE, hKey: CK_OBJECT_HANDLE) -> CK_RV {
    CKR_FUNCTION_NOT_SUPPORTED as CK_RV
}

#[no_mangle]
pub extern "C" fn C_DigestFinal(
    hSession: CK_SESSION_HANDLE,
    pDigest: CK_BYTE_PTR,
    pulDigestLen: CK_ULONG_PTR,
) -> CK_RV {
    CKR_FUNCTION_NOT_SUPPORTED as CK_RV
}

#[no_mangle]
pub extern "C" fn C_Sign(
    hSession: CK_SESSION_HANDLE,
    pData: CK_BYTE_PTR,
    ulDataLen: CK_ULONG,
    pSignature: CK_BYTE_PTR,
    pulSignatureLen: CK_ULONG_PTR,
) -> CK_RV {
    CKR_FUNCTION_NOT_SUPPORTED as CK_RV
}

#[no_mangle]
pub extern "C" fn C_SignInit(
    hSession: CK_SESSION_HANDLE,
    pMechanism: CK_MECHANISM_PTR,
    hKey: CK_OBJECT_HANDLE,
) -> CK_RV {
    CKR_FUNCTION_NOT_SUPPORTED as CK_RV
}

#[no_mangle]
pub extern "C" fn C_SignUpdate(
    hSession: CK_SESSION_HANDLE,
    pPart: CK_BYTE_PTR,
    ulPartLen: CK_ULONG,
) -> CK_RV {
    CKR_FUNCTION_NOT_SUPPORTED as CK_RV
}

#[no_mangle]
pub extern "C" fn C_SignFinal(
    hSession: CK_SESSION_HANDLE,
    pSignature: CK_BYTE_PTR,
    pulSignatureLen: CK_ULONG_PTR,
) -> CK_RV {
    CKR_FUNCTION_NOT_SUPPORTED as CK_RV
}

#[no_mangle]
pub extern "C" fn C_SignRecover(
    hSession: CK_SESSION_HANDLE,
    pData: CK_BYTE_PTR,
    ulDataLen: CK_ULONG,
    pSignature: CK_BYTE_PTR,
    pulSignatureLen: CK_ULONG_PTR,
) -> CK_RV {
    CKR_FUNCTION_NOT_SUPPORTED as CK_RV
}

#[no_mangle]
pub extern "C" fn C_SignRecoverInit(
    hSession: CK_SESSION_HANDLE,
    pMechanism: CK_MECHANISM_PTR,
    hKey: CK_OBJECT_HANDLE,
) -> CK_RV {
    CKR_FUNCTION_NOT_SUPPORTED as CK_RV
}

#[no_mangle]
pub extern "C" fn C_Verify(
    hSession: CK_SESSION_HANDLE,
    pData: CK_BYTE_PTR,
    ulDataLen: CK_ULONG,
    pSignature: CK_BYTE_PTR,
    ulSignatureLen: CK_ULONG,
) -> CK_RV {
    CKR_FUNCTION_NOT_SUPPORTED as CK_RV
}

#[no_mangle]
pub extern "C" fn C_VerifyInit(
    hSession: CK_SESSION_HANDLE,
    pMechanism: CK_MECHANISM_PTR,
    hKey: CK_OBJECT_HANDLE,
) -> CK_RV {
    CKR_FUNCTION_NOT_SUPPORTED as CK_RV
}

#[no_mangle]
pub extern "C" fn C_VerifyUpdate(
    hSession: CK_SESSION_HANDLE,
    pPart: CK_BYTE_PTR,
    ulPartLen: CK_ULONG,
) -> CK_RV {
    CKR_FUNCTION_NOT_SUPPORTED as CK_RV
}

#[no_mangle]
pub extern "C" fn C_VerifyFinal(
    hSession: CK_SESSION_HANDLE,
    pSignature: CK_BYTE_PTR,
    ulSignatureLen: CK_ULONG,
) -> CK_RV {
    CKR_FUNCTION_NOT_SUPPORTED as CK_RV
}

#[no_mangle]
pub extern "C" fn C_VerifyRecover(
    hSession: CK_SESSION_HANDLE,
    pSignature: CK_BYTE_PTR,
    ulSignatureLen: CK_ULONG,
    pData: CK_BYTE_PTR,
    pulDataLen: CK_ULONG_PTR,
) -> CK_RV {
    CKR_FUNCTION_NOT_SUPPORTED as CK_RV
}

#[no_mangle]
pub extern "C" fn C_VerifyRecoverInit(
    hSession: CK_SESSION_HANDLE,
    pMechanism: CK_MECHANISM_PTR,
    hKey: CK_OBJECT_HANDLE,
) -> CK_RV {
    CKR_FUNCTION_NOT_SUPPORTED as CK_RV
}

#[no_mangle]
pub extern "C" fn C_DigestEncryptUpdate(
    hSession: CK_SESSION_HANDLE,
    pPart: CK_BYTE_PTR,
    ulPartLen: CK_ULONG,
    pEncryptedPart: CK_BYTE_PTR,
    pulEncryptedPartLen: CK_ULONG_PTR,
) -> CK_RV {
    CKR_FUNCTION_NOT_SUPPORTED as CK_RV
}

#[no_mangle]
pub extern "C" fn C_DecryptDigestUpdate(
    hSession: CK_SESSION_HANDLE,
    pEncryptedPart: CK_BYTE_PTR,
    ulEncryptedPartLen: CK_ULONG,
    pPart: CK_BYTE_PTR,
    pulPartLen: CK_ULONG_PTR,
) -> CK_RV {
    CKR_FUNCTION_NOT_SUPPORTED as CK_RV
}

#[no_mangle]
pub extern "C" fn C_SignEncryptUpdate(
    hSession: CK_SESSION_HANDLE,
    pPart: CK_BYTE_PTR,
    ulPartLen: CK_ULONG,
    pEncryptedPart: CK_BYTE_PTR,
    pulEncryptedPartLen: CK_ULONG_PTR,
) -> CK_RV {
    CKR_FUNCTION_NOT_SUPPORTED as CK_RV
}

#[no_mangle]
pub extern "C" fn C_DecryptVerifyUpdate(
    hSession: CK_SESSION_HANDLE,
    pEncryptedPart: CK_BYTE_PTR,
    ulEncryptedPartLen: CK_ULONG,
    pPart: CK_BYTE_PTR,
    pulPartLen: CK_ULONG_PTR,
) -> CK_RV {
    CKR_FUNCTION_NOT_SUPPORTED as CK_RV
}

#[no_mangle]
pub extern "C" fn C_DeriveKey(
    hSession: CK_SESSION_HANDLE,
    pMechanism: CK_MECHANISM_PTR,
    hBaseKey: CK_OBJECT_HANDLE,
    pTemplate: CK_ATTRIBUTE_PTR,
    ulAttributeCount: CK_ULONG,
    phKey: CK_OBJECT_HANDLE_PTR,
) -> CK_RV {
    CKR_FUNCTION_NOT_SUPPORTED as CK_RV
}

#[no_mangle]
pub extern "C" fn C_SeedRandom(
    hSession: CK_SESSION_HANDLE,
    pSeed: CK_BYTE_PTR,
    ulSeedLen: CK_ULONG,
) -> CK_RV {
    CKR_FUNCTION_NOT_SUPPORTED as CK_RV
}

#[no_mangle]
pub extern "C" fn C_GenerateRandom(
    hSession: CK_SESSION_HANDLE,
    RandomData: CK_BYTE_PTR,
    ulRandomLen: CK_ULONG,
) -> CK_RV {
    CKR_FUNCTION_NOT_SUPPORTED as CK_RV
}

#[no_mangle]
pub extern "C" fn C_GetFunctionStatus(hSession: CK_SESSION_HANDLE) -> CK_RV {
    CKR_FUNCTION_NOT_SUPPORTED as CK_RV
}

#[no_mangle]
pub extern "C" fn C_CancelFunction(hSession: CK_SESSION_HANDLE) -> CK_RV {
    CKR_FUNCTION_NOT_SUPPORTED as CK_RV
}

#[no_mangle]
pub extern "C" fn C_WaitForSlotEvent(
    flags: CK_FLAGS,
    pSlot: CK_SLOT_ID_PTR,
    pRserved: CK_VOID_PTR,
) -> CK_RV {
    CKR_FUNCTION_NOT_SUPPORTED as CK_RV
}
