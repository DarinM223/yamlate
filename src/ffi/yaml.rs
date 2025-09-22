use crate::environment::ASTEnvironment;
use crate::ffi::types::{Error, FFIArrayReturnValue, FFIReturnValue, YamlType};
use crate::yaml::evaluate;
use libc::c_char;
use std::ffi::{CStr, CString};
use std::ptr;
use yaml_rust::YamlLoader;
use yaml_rust::yaml::Yaml;

/// # Safety
#[unsafe(no_mangle)]
pub unsafe extern "C" fn yaml_create_from_string(s: *const c_char) -> FFIReturnValue<*const Yaml> {
    let yaml_str: String = unsafe { CStr::from_ptr(s).to_string_lossy().into_owned() };
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn yaml_destroy(yaml: *mut Yaml) {
    assert!(!yaml.is_null());
    drop(unsafe { Box::from_raw(yaml) })
}

/// # Safety
#[unsafe(no_mangle)]
pub unsafe extern "C" fn yaml_evaluate(
    yaml: *const Yaml,
    env: *mut ASTEnvironment,
) -> FFIReturnValue<*const Yaml> {
    if let (Some(yaml), Some(environment)) = unsafe { (yaml.as_ref(), env.as_mut()) }
        && let Ok(result) = evaluate(yaml, environment)
    {
        return FFIReturnValue {
            value: Box::into_raw(Box::new(result)),
            error: Error::None as i32,
        };
    }

    FFIReturnValue {
        value: std::ptr::null::<Yaml>(),
        error: Error::EvalError as i32,
    }
}

/// # Safety
#[unsafe(no_mangle)]
pub unsafe extern "C" fn yaml_type(yaml: *const Yaml) -> i32 {
    match unsafe { yaml.as_ref() } {
        Some(Yaml::Real(_)) => YamlType::Real as i32,
        Some(Yaml::Integer(_)) => YamlType::Integer as i32,
        Some(Yaml::String(_)) => YamlType::String as i32,
        Some(Yaml::Boolean(_)) => YamlType::Boolean as i32,
        Some(Yaml::Array(_)) => YamlType::Array as i32,
        Some(Yaml::Hash(_)) => YamlType::Hash as i32,
        _ => YamlType::Null as i32,
    }
}

/// # Safety
#[unsafe(no_mangle)]
pub unsafe extern "C" fn yaml_integer_get(yaml: *const Yaml) -> FFIReturnValue<i32> {
    if let Some(Yaml::Integer(i)) = unsafe { yaml.as_ref() } {
        FFIReturnValue {
            value: *i as i32,
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn yaml_decimal_get(yaml: *const Yaml) -> FFIReturnValue<f64> {
    if let Some(Yaml::Real(s)) = unsafe { yaml.as_ref() } {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn yaml_string_get(yaml: *const Yaml) -> FFIReturnValue<*const c_char> {
    if let Some(Yaml::String(s)) = unsafe { yaml.as_ref() } {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn yaml_hash_keys(
    yaml: *const Yaml,
) -> FFIArrayReturnValue<*const *const c_char> {
    let mut keys: Vec<*const c_char> = Vec::new();

    if let Some(Yaml::Hash(h)) = unsafe { yaml.as_ref() } {
        for (key, _) in h {
            if let Yaml::String(ref s) = *key {
                let c_str = CString::new(s.as_str()).unwrap().into_raw();
                keys.push(c_str as *const c_char);
            }
        }

        keys.shrink_to_fit();
        let length = keys.len();

        FFIArrayReturnValue {
            value: keys.leak().as_ptr(),
            length: length as i32,
            error: Error::None as i32,
        }
    } else {
        FFIArrayReturnValue {
            value: ptr::null(),
            length: 0,
            error: Error::WrongType as i32,
        }
    }
}

/// # Safety
#[unsafe(no_mangle)]
pub unsafe extern "C" fn yaml_hash_get(
    yaml: *const Yaml,
    key: *const c_char,
) -> FFIReturnValue<*const Yaml> {
    let hash_key: String = unsafe { CStr::from_ptr(key).to_string_lossy().into_owned() };

    if let Some(Yaml::Hash(h)) = unsafe { yaml.as_ref() } {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn yaml_array_len(yaml: *const Yaml) -> FFIReturnValue<i32> {
    if let Some(Yaml::Array(a)) = unsafe { yaml.as_ref() } {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn yaml_array_get(
    yaml: *const Yaml,
    index: i32,
) -> FFIReturnValue<*const Yaml> {
    if let Some(Yaml::Array(a)) = unsafe { yaml.as_ref() } {
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
