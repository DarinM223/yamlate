
"""
Python code that tests the C FFI integration
Need to run cargo build --release before running this file
"""

import ctypes

lib = ctypes.cdll.LoadLibrary("target/release/libyaml_embed.dylib")

class Environment(ctypes.Structure):
    pass

env_p = ctypes.POINTER(Environment)

lib.environment_create.restype = env_p
lib.environment_set_integer.argtypes = [env_p, ctypes.c_char_p, ctypes.c_int]

lib.environment_get_integer.argtypes = [env_p, ctypes.c_char_p]
lib.environment_get_integer.restype = ctypes.c_int

# create an environment
environment = lib.environment_create()

# set an integer in the environment
lib.environment_set_integer(environment, 'hello', 2)

# should print out '2'
print lib.environment_get_integer(environment, 'hello')

# cleanup environment after
lib.environment_destroy(environment)

