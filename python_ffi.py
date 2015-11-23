
"""
Python code that tests the C FFI integration
Need to run cargo build --release before running this file
"""

import ctypes

lib = ctypes.cdll.LoadLibrary("target/release/libyaml_embed.dylib")

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

class Environment(ctypes.Structure):
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

env_p = ctypes.POINTER(Environment)

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

# cleanup environment after
lib.environment_destroy(environment)

