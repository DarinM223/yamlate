
"""
Python code that tests the C FFI integration
Need to run cargo build --release before running this file
"""

import ctypes

lib = ctypes.cdll.LoadLibrary("target/release/libyamlate.dylib")

"""
Python library bindings for yaml_embed C FFI
"""

class ErrorCode:
    """
    The returned error code values
    """
    ERROR_NONE = 0
    ERROR_WRONGTYPE = -1
    ERROR_NOTDEFINED = -2
    ERROR_INVALIDSTRING = -3

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

# create an environment
environment = lib.environment_create()

# set some values in the environment
lib.environment_set_integer(environment, 'hello', 2)
lib.environment_set_string(environment, 'world', 'blah')
lib.environment_set_decimal(environment, 'blah', 3.14)

result = lib.environment_get_string(environment, 'world')
# should print out 'blah'
print result.value
# should print out '0'
print result.error

result = lib.environment_get_integer(environment, 'hello')
# should print out '2'
print result.value
# should print out '0'
print result.error

result = lib.environment_get_decimal(environment, 'blah')
# should print out '3.14'
print result.value
# should print out '0'
print result.error

# open up a yaml file
with open("./examples/example.yaml", "r") as yaml_file:
    data = yaml_file.read()

    # load with yaml string
    yaml_ret = lib.yaml_create_from_string(data)

    if yaml_ret.error == ErrorCode.ERROR_NONE:
        yaml = yaml_ret.value
        keys = lib.yaml_hash_keys(yaml)

        if keys.error == ErrorCode.ERROR_NONE:
            for i in range(0, keys.length):
                print keys.value[i]

        lib.yaml_destroy(yaml)

# cleanup environment after
lib.environment_destroy(environment)

