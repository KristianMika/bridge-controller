use std::ptr;

use aes::cipher::{generic_array::GenericArray, BlockDecrypt};

use crate::STATE;

use super::{
    bindings::{
        CKR_ARGUMENTS_BAD, CKR_CRYPTOKI_NOT_INITIALIZED, CKR_FUNCTION_NOT_SUPPORTED,
        CKR_GENERAL_ERROR, CKR_OK, CKR_OPERATION_NOT_INITIALIZED, CKR_SESSION_HANDLE_INVALID,
        CK_BYTE_PTR, CK_MECHANISM_PTR, CK_OBJECT_HANDLE, CK_RV, CK_SESSION_HANDLE, CK_ULONG,
        CK_ULONG_PTR,
    },
    encryption::{C_Encrypt, C_EncryptInit},
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
    C_EncryptInit(hSession, pMechanism, hKey)
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
    // TODO: use C_Encrypt instead of copy-and-paste
    if pEncryptedData.is_null() || pulDataLen.is_null() {
        return CKR_ARGUMENTS_BAD as CK_RV;
    }
    let Ok(state) = STATE.read() else  {
        return CKR_GENERAL_ERROR as CK_RV;
    };
    let Some( state) = state.as_ref() else {
        return CKR_CRYPTOKI_NOT_INITIALIZED as CK_RV;
    };

    let Some(session) =  state.get_session(&hSession) else {
            return CKR_SESSION_HANDLE_INVALID as CK_RV;
    };

    let Some(encryptor)=session.get_encryptor() else {
        return CKR_OPERATION_NOT_INITIALIZED as CK_RV;
    };

    let mut data = Vec::with_capacity(ulEncryptedDataLen as usize);
    unsafe {
        ptr::copy(
            pEncryptedData,
            data.as_mut_ptr(),
            ulEncryptedDataLen as usize,
        );
        data.set_len(ulEncryptedDataLen as usize)
    };
    let mut cipher_length = 0;
    // TODO: check block length
    for block_i in 0..(data.len() / 16) {
        let mut block =
            GenericArray::from_slice(&data[(16 * block_i)..(16 * (block_i + 1))]).to_owned();
        encryptor.decrypt_block(&mut block);
        if !pData.is_null() {
            unsafe {
                ptr::copy(
                    block.as_ptr(),
                    pData.offset((block_i * 16) as isize),
                    block.len(),
                );
            }
        }
        cipher_length += block.len();
    }

    unsafe {
        *pulDataLen = cipher_length as u64;
    }

    CKR_OK as CK_RV
}
