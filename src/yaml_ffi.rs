use libc::c_char;
use yaml_rust::yaml::Yaml;
use yaml_rust::YamlLoader;
use std::mem::transmute;
use std::ffi::{CStr, CString};
use ffi_types::{FFIReturnValue, Error, YamlType};
use yaml::evaluate;
use environment::Environment;

#[no_mangle]
pub extern "C" fn yaml_create_from_string(s: *const c_char) -> *const Yaml {
    let yaml_str: String = unsafe { CStr::from_ptr(s).to_string_lossy().into_owned() };
    let mut docs = YamlLoader::load_from_str(yaml_str.as_str()).unwrap();

    let doc = docs.pop();
    
    let yaml_ptr = unsafe { transmute(box doc) };

    yaml_ptr
}

#[no_mangle]
pub extern "C" fn yaml_destroy(yaml: *const Yaml) {
    let yaml: Box<Yaml> = unsafe { transmute(yaml) };
}

#[no_mangle]
pub extern "C" fn yaml_evaluate(yaml: *const Yaml, env: *mut Environment) -> *const Yaml {
    let yaml = unsafe { &*yaml };
    let environment = unsafe { &mut *env };

    let result = evaluate(yaml, environment);

    let yaml_ptr = unsafe { transmute(box result) };

    yaml_ptr
}

#[no_mangle]
pub extern "C" fn yaml_type(yaml: *const Yaml) -> i32 {
    let yaml = unsafe { &*yaml };

    match yaml {
        &Yaml::Real(_) => YamlType::Real as i32,
        &Yaml::Integer(_) => YamlType::Integer as i32,
        &Yaml::String(_) => YamlType::String as i32,
        &Yaml::Boolean(_) => YamlType::Boolean as i32,
        &Yaml::Array(_) => YamlType::Array as i32,
        &Yaml::Hash(_) => YamlType::Hash as i32,
        &Yaml::Null => YamlType::Null as i32,
        _ => YamlType::Null as i32,
    }
}

#[no_mangle]
pub extern "C" fn yaml_integer_get(yaml: *const Yaml) -> FFIReturnValue<i32> {
    let yaml = unsafe { &*yaml };

    match yaml {
        &Yaml::Integer(i) => FFIReturnValue {
            value: i as i32,
            error: Error::None as i32,
        },
        _ => FFIReturnValue {
            value: 0,
            error: Error::WrongType as i32,
        },
    }
}

#[no_mangle]
pub extern "C" fn yaml_decimal_get(yaml: *const Yaml) -> FFIReturnValue<f64> {
    let yaml = unsafe { &*yaml };

    match yaml {
        &Yaml::Real(ref s) => FFIReturnValue {
            value: s.parse::<f64>().unwrap_or(0.0),
            error: Error::None as i32,
        },
        _ => FFIReturnValue {
            value: 0.0,
            error: Error::WrongType as i32,
        },
    }
}

#[no_mangle]
pub extern "C" fn yaml_string_get(yaml: *const Yaml) -> FFIReturnValue<*const c_char> {
    let yaml = unsafe { &*yaml };

    match yaml {
        &Yaml::String(ref s) => FFIReturnValue {
            value: CString::new(s.as_str()).unwrap().as_ptr(),
            error: Error::None as i32,
        },
        _ => FFIReturnValue {
            value: CString::new("").unwrap().as_ptr(),
            error: Error::WrongType as i32,
        },
    }
}

//#[no_mangle]
//pub extern "C" fn yaml_hash_keys(yaml: *const Yaml) -> FFIReturnValue<*const *const c_char> {
//}

//#[no_mangle]
//pub extern "C" fn yaml_hash_get(yaml: *const Yaml, key: *const c_char) -> FFIReturnValue<*const Yaml> {
//}

//#[no_mangle]
//pub extern "C" fn yaml_array_len(yaml: *const Yaml) -> FFIReturnValue<i32> {
//}

//#[no_mangle]
//pub extern "C" fn yaml_array_get(yaml: *const Yaml) -> FFIReturnValue<*const Yaml> {
//}
