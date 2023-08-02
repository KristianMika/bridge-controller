use std::{cmp::min, ptr};

use lazy_static::__Deref;

use crate::{state::object::ObjectSearch, STATE};

use super::bindings::{
    CKR_ARGUMENTS_BAD, CKR_CRYPTOKI_NOT_INITIALIZED, CKR_GENERAL_ERROR, CKR_OK,
    CKR_SESSION_HANDLE_INVALID, CK_ATTRIBUTE, CK_ATTRIBUTE_PTR, CK_OBJECT_HANDLE,
    CK_OBJECT_HANDLE_PTR, CK_RV, CK_SESSION_HANDLE, CK_ULONG, CK_ULONG_PTR,
};

/// Creates an object
///
/// # Arguments
///
/// * `hSession` - session's handle
/// * `pTemplate` - points to the object’s template
/// * `ulCount` - the number of attributes in the template
/// * `phObject` - points to the location that receives the new object’s handle
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn C_CreateObject(
    hSession: CK_SESSION_HANDLE,
    pTemplate: CK_ATTRIBUTE_PTR,
    ulCount: CK_ULONG,
    phObject: CK_OBJECT_HANDLE_PTR,
) -> CK_RV {
    if pTemplate.is_null() || phObject.is_null() {
        return CKR_ARGUMENTS_BAD as CK_RV;
    }

    let Ok(mut state) = STATE.write() else  {
        return CKR_GENERAL_ERROR as CK_RV;
    };
    let Some( state) = state.as_mut() else {
        return CKR_CRYPTOKI_NOT_INITIALIZED as CK_RV;
    };

    let mut template: Vec<CK_ATTRIBUTE> = Vec::with_capacity(ulCount as usize);
    unsafe {
        ptr::copy(pTemplate, template.as_mut_ptr(), ulCount as usize);
        template.set_len(ulCount as usize);
    }

    let return_code = match state.get_session_mut(&hSession) {
        Some(mut session) => {
            session.create_object(template.into());
            CKR_OK as CK_RV
        }
        None => CKR_SESSION_HANDLE_INVALID as CK_RV,
    };

    return_code
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

/// Initializes a search for token and session objects that match a template
///
/// # Arguments
///
/// * `hSession` - the session’s handle
/// * `pTemplate` - points to a search template that specifies the attribute values to match
/// * `ulCount` - the number of attributes in the search template
#[no_mangle]
pub extern "C" fn C_GetAttributeValue(
    hSession: CK_SESSION_HANDLE,
    hObject: CK_OBJECT_HANDLE,
    pTemplate: CK_ATTRIBUTE_PTR,
    ulCount: CK_ULONG,
) -> CK_RV {
    unimplemented!()
}

/// Initializes a search for token and session objects that match a template.
/// The matching criterion is an exact byte-for-byte match with all attributes in the template.
///
/// # Arguments
///
/// * `hSession` - the session’s handle
/// * `pTemplate` - points to a search template that specifies the attribute values to match
/// * `ulCount` - the number of attributes in the search template. If 0, find all objects
///
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn C_FindObjectsInit(
    hSession: CK_SESSION_HANDLE,
    pTemplate: CK_ATTRIBUTE_PTR,
    ulCount: CK_ULONG,
) -> CK_RV {
    if pTemplate.is_null() {
        return CKR_ARGUMENTS_BAD as CK_RV;
    }

    let Ok(mut state) = STATE.write() else  {
        return CKR_GENERAL_ERROR as CK_RV;
    };
    let Some( state) = state.as_mut() else {
        return CKR_CRYPTOKI_NOT_INITIALIZED as CK_RV;
    };

    let mut template: Vec<CK_ATTRIBUTE> = Vec::with_capacity(ulCount as usize);
    unsafe {
        ptr::copy(pTemplate, template.as_mut_ptr(), ulCount as usize);
        template.set_len(ulCount as usize);
    }

    let object_search = ObjectSearch::new(template.into());

    let return_code = match state.get_session_mut(&hSession) {
        Some(mut session) => {
            session.init_object_search(object_search);
            CKR_OK as CK_RV
        }
        None => CKR_SESSION_HANDLE_INVALID as CK_RV,
    };
    return_code
}

/// Continues a search for token and session objects that match a template, obtaining additional object handles
///
/// # Arguments
///
/// * `hSession` - the session’s handle
/// * `phObject` - points to the location that receives the list (array) of additional object handles
/// * `ulMaxObjectCount` - the maximum number of object handles to be returned
/// * `pulObjectCount` - points to the location that receives the actual number of object handles returned
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn C_FindObjects(
    hSession: CK_SESSION_HANDLE,
    phObject: CK_OBJECT_HANDLE_PTR,
    ulMaxObjectCount: CK_ULONG,
    pulObjectCount: CK_ULONG_PTR,
) -> CK_RV {
    if phObject.is_null() || pulObjectCount.is_null() {
        return CKR_ARGUMENTS_BAD as CK_RV;
    }

    let Ok(state) = STATE.read() else  {
        return CKR_GENERAL_ERROR as CK_RV;
    };
    let Some( state) = state.deref() else {
        return CKR_CRYPTOKI_NOT_INITIALIZED as CK_RV;
    };
    let filtered_handles = match state.get_session(&hSession) {
        Some(session) => session.get_filtered_handles(),
        None => return CKR_SESSION_HANDLE_INVALID as CK_RV,
    };

    let copy_count = min(ulMaxObjectCount as usize, filtered_handles.len());
    unsafe {
        ptr::copy(filtered_handles.as_ptr(), phObject, copy_count);
        *pulObjectCount = copy_count as u64;
    }
    CKR_OK as CK_RV
}

/// Terminates a search for token and session objects
///
/// # Arguments
///
/// * `hSession` - the session’s handle
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn C_FindObjectsFinal(hSession: CK_SESSION_HANDLE) -> CK_RV {
    let Ok(mut state) = STATE.write() else  {
        return CKR_GENERAL_ERROR as CK_RV;
    };
    let Some(state) = state.as_mut() else {
        return CKR_CRYPTOKI_NOT_INITIALIZED as CK_RV;
    };

    match state.get_session_mut(&hSession) {
        Some(mut session) => session.reset_object_search(),
        None => return CKR_SESSION_HANDLE_INVALID as CK_RV,
    };
    CKR_OK as CK_RV
}
