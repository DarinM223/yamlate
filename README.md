yamlate
=========

A data templating language extended from YAML written in Rust

[![Build Status](https://travis-ci.org/DarinM223/yamlate.svg)](https://travis-ci.org/DarinM223/yamlate)

### What it does:

Given a mapping of variables to values, it evaluates YAML properties
written in a special language dynamically. It also can update the variables in the mapping
so it can change its "state".

Language features inside YAML are prefixed with '~>' so the interpreter can know to evaluate them.

### Hypothetical use case:

Lets say that you want to gather data from around a couple thousand bug species. Each bug has different
behavior. Some change their wing color based on the season, and others change their wing color when another bug is nearby.
Once a bug changes their wing color when another bug is nearby, the other bug leaves and there is no longer another bug nearby. 

One way to model this is to hardcode the bug behavior using something similar to a strategy pattern. You could create a thousand different code 
files for each bug. 

One problem with this is that writing code for every bug ties the data to the code which makes it hard for non-developers to 
understand the core logic and ties the data to a specific language (Java, Python, etc).

Another way is to use a markup language like XML or a data serialization format like YAML so that the data is abstracted from the code.

One problem with this is that there is a lot of complex logic and data is being transformed (the other bug leaving
changes the environment) so something like YAML won't be enough.

With Yamlate, you can do something like this:

```yaml
cricket:
  wing_span: 3.5
  wing_color:
    - if:
      - '~> current_season == "spring"'
      - do:
        - 'red'
        else:
        - 'blue'
        
beetle:
  wing_span: 2.9
  wing_color:
    - if:
      - '~> another_beetle_nearby == 1'
      - do:
        - '~> another_beetle_nearby = 0'
        - 'blue'
        else:
        - 'red'

# etc... for all of the bugs
```

This allows you to have the advantages of YAML with the ability to model more complex logic.

You can also write a program in your favorite language (right now only python :P) to retrieve the data:

```python
lib = ctypes.cdll.LoadLibrary("../target/release/libyamlate.dylib")
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
```
Note: the python wrapper is pretty verbose, hopefully I can refactor it into something better

For the full example look at the python/python_example_bug.py for the python file and the examples/bug.yaml for the
YAML file

### Drawbacks:

* Slow because it is essentially an unoptimized interpreter without JIT
* Not the only language that does this (higher performing languages like Lua are widely used to
script similar cases like Redis extensions or AI for different enemies in games) 
* "code" currently is not very readable so might be difficult for non-developers to understand 

### TODO:

* Fix bugs in interpreter/FFI
* Add Ruby wrapper
* Add support for more complex language features like functions, structures
* Be able to access other fields in the YAML file as a variable