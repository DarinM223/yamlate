import ctypes
import os
from python_ffi import Yamlate 

"""
Python code that tests the C FFI integration for the sample bug YAML
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
    environment.set_integer('another_beetle_nearby', 1)
    environment.set_string('current_season', 'spring')

    with open('../examples/bug.yaml', 'r') as yaml_file:
        data = yaml_file.read()
            
        with yamlate.new_yaml_from_str(data) as root_yaml:
            with root_yaml.hash_get('cricket') as cricket_yaml:
                with cricket_yaml.hash_get('wing_color') as wing_yaml:
                    with wing_yaml.evaluate(environment) as cricket_wing_result:
                        # should print 'red'
                        print cricket_wing_result.get_string()
                                        
            with root_yaml.hash_get('beetle') as beetle_yaml:
                with beetle_yaml.hash_get('wing_color') as wing_yaml:
                    with wing_yaml.evaluate(environment) as beetle_wing_result:
                        # should print 'blue'
                        print beetle_wing_result.get_string()
                        
                        # should print '0'
                        print environment.get_integer('another_beetle_nearby')
