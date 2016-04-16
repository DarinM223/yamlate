import ctypes
import os
import ffi_types
from python_ffi import Yamlate

"""
Python code that tests the C FFI integration
Need to run cargo build --release before running this file
You also need to be in the python/ directory
"""

# load correct library depending on operating system
if os.uname()[0] == 'Darwin':
    lib = ctypes.cdll.LoadLibrary('../target/release/libyamlate.dylib')
else:
    lib = ctypes.cdll.LoadLibrary('../target/release/libyamlate.so')
yamlate = Yamlate(lib)

with yamlate.new_environment() as environment:
    environment.set_integer('hello', 2)
    environment.set_string('world', 'blah')
    environment.set_decimal('blah', 3.14)

    # should print 'blah'
    print 'Environment value for \'world\':', environment.get_string('world')
    # should print '2'
    print 'Environment value for \'hello\':', environment.get_integer('hello')
    # should print '3.14'
    print 'Environment value for \'blah\':', environment.get_decimal('blah')

    with open('../examples/example.yaml', 'r') as yaml_file:
        data = yaml_file.read()

        with yamlate.new_yaml_from_str(data) as root_yaml:
            with root_yaml.hash_get('blah') as blah_yaml:
                print 'Blah\'s type:', ffi_types.yaml_type_to_str(blah_yaml.type())
                # should print '2'
                print 'Blah\'s value:', blah_yaml.get_integer()

            with root_yaml.hash_get('foo') as foo_yaml:
                print 'Foo\'s type:', ffi_types.yaml_type_to_str(foo_yaml.type())
                print 'Foo array length:', foo_yaml.array_len()
                with foo_yaml.array_get(0) as sub_yaml:
                    print 'Foo\'s first array element type:', ffi_types.yaml_type_to_str(sub_yaml.type())

                with foo_yaml.evaluate(environment) as result:
                    # should print '10
                    print 'Foo\'s value:', result.get_integer()

            print 'Root\'s type:', ffi_types.yaml_type_to_str(root_yaml.type())
            # should print ['blah', 'foo']
            print 'Root keys:', root_yaml.hash_keys()
