
import ctypes

"""
Python C FFI types for Yamlate
"""

class ErrorCode:
    """
    The returned error code values
    """
    ERROR_NONE = 0
    ERROR_WRONGTYPE = -1
    ERROR_NOTDEFINED = -2
    ERROR_INVALIDSTRING = -3

class YamlType:
    """
    The returned YAML type code values
    """
    INTEGER = 0
    REAL = 1
    STRING = 2
    BOOLEAN = 3
    ARRAY = 4
    HASH = 5
    NULL = 6

def yaml_type_to_str(yaml):
    if yaml == YamlType.INTEGER:
        return 'Integer'
    elif yaml == YamlType.REAL:
        return 'Real'
    elif yaml == YamlType.STRING:
        return 'String'
    elif yaml == YamlType.BOOLEAN:
        return 'Boolean'
    elif yaml == YamlType.ARRAY:
        return 'Array'
    elif yaml == YamlType.HASH:
        return 'Hash'
    elif yaml == YamlType.NULL:
        return 'Null'
    else:
        return 'Unknown type'

class Environment(ctypes.Structure):
    pass

class Yaml(ctypes.Structure):
    pass

class IntReturnType(ctypes.Structure):
    _fields_ = [("value", ctypes.c_int),
                ("error", ctypes.c_int)]

class StringReturnType(ctypes.Structure):
    _fields_ = [("value", ctypes.c_char_p),
                ("error", ctypes.c_int)]

class DecimalReturnType(ctypes.Structure):
    _fields_ = [("value", ctypes.c_double),
                ("error", ctypes.c_int)]

class ArrayStringReturnType(ctypes.Structure):
    _fields_ = [("value", ctypes.POINTER(ctypes.c_char_p)),
                ("length", ctypes.c_int),
                ("error", ctypes.c_int)]

env_p = ctypes.POINTER(Environment)
yaml_p = ctypes.POINTER(Yaml)

class YamlReturnType(ctypes.Structure):
    _fields_ = [("value", yaml_p),
                ("error", ctypes.c_int)]

def ffi_function_signatures(lib):
    """
    Defines the C FFI function signatures
    """
    lib.environment_create.restype = env_p
    
    lib.environment_set_integer.argtypes = [env_p, ctypes.c_char_p, ctypes.c_int]
    lib.environment_get_integer.argtypes = [env_p, ctypes.c_char_p]
    lib.environment_get_integer.restype = IntReturnType
    
    lib.environment_set_string.argtypes = [env_p, ctypes.c_char_p, ctypes.c_char_p]
    lib.environment_get_string.argtypes = [env_p, ctypes.c_char_p]
    lib.environment_get_string.restype = StringReturnType
    
    lib.environment_set_decimal.argtypes = [env_p, ctypes.c_char_p, ctypes.c_double]
    lib.environment_get_decimal.argtypes = [env_p, ctypes.c_char_p]
    lib.environment_get_decimal.restype = DecimalReturnType
    
    lib.yaml_create_from_string.argtypes = [ctypes.c_char_p]
    lib.yaml_create_from_string.restype = YamlReturnType 
    
    lib.yaml_destroy.argtypes = [yaml_p]
    
    lib.yaml_evaluate.argtypes = [yaml_p, env_p]
    lib.yaml_evaluate.restype = yaml_p
    
    lib.yaml_type.argtypes = [yaml_p]
    lib.yaml_type.restype = ctypes.c_int
    
    lib.yaml_integer_get.argtypes = [yaml_p]
    lib.yaml_integer_get.restype = IntReturnType
    
    lib.yaml_decimal_get.argtypes = [yaml_p]
    lib.yaml_decimal_get.restype = DecimalReturnType
    
    lib.yaml_string_get.argtypes = [yaml_p]
    lib.yaml_string_get.restype = StringReturnType
    
    lib.yaml_hash_keys.argtypes = [yaml_p]
    lib.yaml_hash_keys.restype = ArrayStringReturnType
    
    lib.yaml_hash_get.argtypes = [yaml_p, ctypes.c_char_p]
    lib.yaml_hash_get.restype = YamlReturnType

    lib.yaml_array_len.argtypes = [yaml_p]
    lib.yaml_array_len.restype = IntReturnType

    lib.yaml_array_get.argtypes = [yaml_p, ctypes.c_int]
    lib.yaml_array_get.restype = YamlReturnType

