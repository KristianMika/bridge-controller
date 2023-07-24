use crate::{
    bindings::{CKR_ARGUMENTS_BAD, CKR_OK, CK_FUNCTION_LIST_PTR, CK_FUNCTION_LIST_PTR_PTR, CK_RV},
    C_GetFunctionList,
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
