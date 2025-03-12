use environment::ASTEnvironment;
use ffi::types::{Error, FFIArrayReturnValue, FFIReturnValue, YamlType};
use libc::c_char;
use std::ffi::{CStr, CString};
use std::mem::forget;
use std::ptr;
use yaml::evaluate;
use yaml_rust::yaml::Yaml;
use yaml_rust::YamlLoader;

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn yaml_create_from_string(s: *const c_char) -> FFIReturnValue<*const Yaml> {
    let yaml_str: String = CStr::from_ptr(s).to_string_lossy().into_owned();
    let mut docs = YamlLoader::load_from_str(yaml_str.as_str()).unwrap();

    let doc_option = docs.pop();

    if let Some(doc) = doc_option {
        let yaml_ptr = Box::into_raw(Box::new(doc));

        FFIReturnValue {
            value: yaml_ptr,
            error: Error::None as i32,
        }
    } else {
        FFIReturnValue {
            value: ptr::null(),
            error: Error::InvalidString as i32,
        }
    }
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn yaml_destroy(yaml: *mut Yaml) {
    drop(Box::from_raw(yaml))
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn yaml_evaluate(
    yaml: *const Yaml,
    env: *mut ASTEnvironment,
) -> FFIReturnValue<*const Yaml> {
    let yaml = unsafe { &*yaml };
    let environment = unsafe { &mut *env };

    if let Ok(result) = evaluate(yaml, environment) {
        FFIReturnValue {
            value: Box::into_raw(Box::new(result)),
            error: Error::None as i32,
        }
    } else {
        FFIReturnValue {
            value: std::ptr::null::<Yaml>(),
            error: Error::EvalError as i32,
        }
    }
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn yaml_type(yaml: *const Yaml) -> i32 {
    match *yaml {
        Yaml::Real(_) => YamlType::Real as i32,
        Yaml::Integer(_) => YamlType::Integer as i32,
        Yaml::String(_) => YamlType::String as i32,
        Yaml::Boolean(_) => YamlType::Boolean as i32,
        Yaml::Array(_) => YamlType::Array as i32,
        Yaml::Hash(_) => YamlType::Hash as i32,
        Yaml::Null => YamlType::Null as i32,
        _ => YamlType::Null as i32,
    }
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn yaml_integer_get(yaml: *const Yaml) -> FFIReturnValue<i32> {
    if let Yaml::Integer(i) = *yaml {
        FFIReturnValue {
            value: i as i32,
            error: Error::None as i32,
        }
    } else {
        FFIReturnValue {
            value: 0,
            error: Error::WrongType as i32,
        }
    }
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn yaml_decimal_get(yaml: *const Yaml) -> FFIReturnValue<f64> {
    if let Yaml::Real(ref s) = *yaml {
        FFIReturnValue {
            value: s.parse::<f64>().unwrap_or(0.0),
            error: Error::None as i32,
        }
    } else {
        FFIReturnValue {
            value: 0.0,
            error: Error::WrongType as i32,
        }
    }
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn yaml_string_get(yaml: *const Yaml) -> FFIReturnValue<*const c_char> {
    if let Yaml::String(ref s) = *yaml {
        let c_str = CString::new(s.as_str()).unwrap().into_raw();

        FFIReturnValue {
            value: c_str as *const c_char,
            error: Error::None as i32,
        }
    } else {
        FFIReturnValue {
            value: CString::new("").unwrap().into_raw() as *const c_char,
            error: Error::WrongType as i32,
        }
    }
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn yaml_hash_keys(
    yaml: *const Yaml,
) -> FFIArrayReturnValue<*const *const c_char> {
    let mut keys: Vec<*const c_char> = Vec::new();

    if let Yaml::Hash(ref h) = *yaml {
        for (key, _) in h {
            if let Yaml::String(ref s) = *key {
                let c_str = CString::new(s.as_str()).unwrap().into_raw();
                keys.push(c_str as *const c_char);
            }
        }

        keys.shrink_to_fit();

        let arr_ptr = keys.as_ptr();
        let length = keys.len();

        forget(keys);

        FFIArrayReturnValue {
            value: arr_ptr,
            length: length as i32,
            error: Error::None as i32,
        }
    } else {
        FFIArrayReturnValue {
            value: keys.as_ptr(),
            length: 0,
            error: Error::WrongType as i32,
        }
    }
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn yaml_hash_get(
    yaml: *const Yaml,
    key: *const c_char,
) -> FFIReturnValue<*const Yaml> {
    let hash_key: String = unsafe { CStr::from_ptr(key).to_string_lossy().into_owned() };

    if let Yaml::Hash(ref h) = *yaml {
        if let Some(result) = h.get(&Yaml::String(hash_key)) {
            let yaml_ptr = Box::into_raw(Box::new(result.clone()));

            FFIReturnValue {
                value: yaml_ptr,
                error: Error::None as i32,
            }
        } else {
            FFIReturnValue {
                value: ptr::null(),
                error: Error::NotDefined as i32,
            }
        }
    } else {
        FFIReturnValue {
            value: ptr::null(),
            error: Error::WrongType as i32,
        }
    }
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn yaml_array_len(yaml: *const Yaml) -> FFIReturnValue<i32> {
    if let Yaml::Array(ref a) = *yaml {
        FFIReturnValue {
            value: a.len() as i32,
            error: Error::None as i32,
        }
    } else {
        FFIReturnValue {
            value: 0,
            error: Error::WrongType as i32,
        }
    }
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn yaml_array_get(
    yaml: *const Yaml,
    index: i32,
) -> FFIReturnValue<*const Yaml> {
    if let Yaml::Array(ref a) = *yaml {
        if let Some(result) = a.get(index as usize) {
            let yaml_ptr = Box::into_raw(Box::new(result.clone()));

            FFIReturnValue {
                value: yaml_ptr,
                error: Error::None as i32,
            }
        } else {
            FFIReturnValue {
                value: ptr::null(),
                error: Error::NotDefined as i32,
            }
        }
    } else {
        FFIReturnValue {
            value: ptr::null(),
            error: Error::WrongType as i32,
        }
    }
}
