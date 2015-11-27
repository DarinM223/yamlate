import ffi_types

"""
Python library bindings for yaml_embed C FFI
"""

class WrongTypeError(Exception):
    def __str__(self):
        return 'Wrong type error with Yamlate FFI API'

class NotDefinedError(Exception):
    def __str__(self):
        return 'Not defined error with Yamlate FFI API'

class InvalidStringError(Exception):
    def __str__(self):
        return 'Invalid string error with Yamlate FFI API'

def handle_ffi_error(code):
    if code == ffi_types.ErrorCode.ERROR_INVALIDSTRING:
        raise InvalidStringError()
    elif code == ffi_types.ErrorCode.ERROR_NOTDEFINED:
        raise NotDefinedError()
    elif code == ffi_types.ErrorCode.ERROR_WRONGTYPE:
        raise WrongTypeError()

class Environment:
    def __init__(self, lib, environment):
        self.environment = environment
        self.lib = lib
    
    def set_integer(self, key, val):
        """
        Sets an integer in the environment
        :param string: key
        :param integer: val
        """
        self.lib.environment_set_integer(self.environment, key, val)

    def set_decimal(self, key, val):
        """
        Sets decimal in the environment
        :param string: key
        :param double: val
        """
        self.lib.environment_set_decimal(self.environment, key, val)

    def set_string(self, key, val):
        """
        Sets string in the environment
        :param string: key
        :param string: val
        """
        self.lib.environment_set_string(self.environment, key, val)

    def get_integer(self, key):
        """
        Gets an integer in the environment
        :param string: key
        :return: integer
        """
        result = self.lib.environment_get_integer(self.environment, key)
        if result.error != ffi_types.ErrorCode.ERROR_NONE:
            handle_ffi_error(result.error)

        return result.value

    def get_decimal(self, key):
        """
        Gets an decimal in the environment
        :param string: key
        :return: double
        """
        result = self.lib.environment_get_decimal(self.environment, key)
        if result.error != ffi_types.ErrorCode.ERROR_NONE:
            handle_ffi_error(result.error)

        return result.value

    def get_string(self, key):
        """
        Gets an string in the environment
        :param string: key
        :return: string
        """
        result = self.lib.environment_get_string(self.environment, key)
        if result.error != ffi_types.ErrorCode.ERROR_NONE:
            handle_ffi_error(result.error)

        return result.value

class Yaml:
    def __init__(self, lib, yaml):
        self.yaml = yaml
        self.lib = lib

    def type(self):
        return self.lib.yaml_type(yaml)

    def evaluate(self, env):
        return CopyYaml(self.lib, self.lib.yaml_evaluate(self.yaml, env.environment))

    def get_integer(self):
        result = self.lib.yaml_integer_get(self.yaml)
        if result.error != ffi_types.ErrorCode.ERROR_NONE:
            handle_ffi_error(result.error)

        return result.value

    def get_decimal(self):
        result = self.lib.yaml_decimal_get(self.yaml)
        if result.error != ffi_types.ErrorCode.ERROR_NONE:
            handle_ffi_error(result.error)

        return result.value


    def get_string(self):
        result = self.lib.yaml_string_get(self.yaml)
        if result.error != ffi_types.ErrorCode.ERROR_NONE:
            handle_ffi_error(result.error)

        return result.value

    def hash_keys(self):
        result = self.lib.yaml_hash_keys(self.yaml)
        if result.error != ffi_types.ErrorCode.ERROR_NONE:
            handle_ffi_error(result.error)

        ret_value = []
        for i in range(0, result.length):
            ret_value.append(result.value[i])

        return ret_value

    def hash_get(self, key):
        result = self.lib.yaml_hash_get(self.yaml, key)
        if result.error != ffi_types.ErrorCode.ERROR_NONE:
            handle_ffi_error(result.error)

        return CopyYaml(self.lib, result.value)

class NewEnv:
    def __init__(self, lib):
        self.lib = lib

    def __enter__(self):
        self.environment = self.lib.environment_create()
        return Environment(self.lib, self.environment)

    def __exit__(self, ex_type, ex_val, traceback):
        self.lib.environment_destroy(self.environment)
        return True

class NewYaml:
    def __init__(self, lib, s):
        self.lib = lib
        self.yaml_str = s

    def __enter__(self):
        result = self.lib.yaml_create_from_string(self.yaml_str)
        if result.error != ffi_types.ErrorCode.ERROR_NONE:
            handle_ffi_error(result.error)

        self.yaml = result.value
        return Yaml(self.lib, self.yaml)

    def __exit__(self, ex_type, ex_val, traceback):
        self.lib.yaml_destroy(self.yaml)
        return True

class CopyYaml:
    def __init__(self, lib, yaml):
        self.lib = lib
        self.yaml = yaml

    def __enter__(self):
        return Yaml(self.lib, self.yaml)

    def __exit__(self, ex_type, ex_val, traceback):
        self.lib.yaml_destroy(self.yaml)
        return True

class Yamlate:
    """
    Main class for Yamlate
    """

    def __init__(self, lib):
        self.lib = lib
        ffi_types.ffi_function_signatures(self.lib)

    def new_environment(self):
        return NewEnv(self.lib)

    def new_yaml_from_str(self, s):
        return NewYaml(self.lib, s)

