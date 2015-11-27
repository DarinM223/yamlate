import ctypes
from python_ffi import Yamlate 

"""
Python code that tests the C FFI integration
Need to run cargo build --release before running this file
"""

lib = ctypes.cdll.LoadLibrary("../target/release/libyamlate.dylib")
yamlate = Yamlate(lib)

with yamlate.new_environment() as environment:
    environment.set_integer('hello', 2)
    environment.set_string('world', 'blah')
    environment.set_decimal('blah', 3.14)

    # should print 'blah'
    print environment.get_string('world')
    # should print '2'
    print environment.get_integer('hello')
    # should print '3.14'
    print environment.get_decimal('blah')

    with open('../examples/example.yaml', 'r') as yaml_file:
        data = yaml_file.read()

        with yamlate.new_yaml_from_str(data) as root_yaml:
            with root_yaml.hash_get('blah') as blah_yaml:
                # should print '2'
                print blah_yaml.get_integer()

            with root_yaml.hash_get('foo') as foo_yaml:
                with foo_yaml.evaluate(environment) as result:
                    # should print '10
                    print result.get_integer()

            # should print ['blah', 'foo']
            print root_yaml.hash_keys()

