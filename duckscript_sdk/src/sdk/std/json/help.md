The json module provides json parsing and encoding capabilities.<br>
Parsing and encoding JSONs can be do to/from simple variables or to collections (maps/arrays).<br>
By default, when parsing a JSON string, the structure will be represented by simple variables.<br>
The root object (or simple value) will be set in the json_parse output variable and any sub structure will be
defined as variables with prefix of the root variable name.<br>
Object nodes, will have the value of: **[OBJECT]**.<br>
Array nodes will have a length variable defined, for example: **arr.length**<br>
If the --collections flag is provided, parsing will return the JSON value or a handle to a collection which will hold the values (or sub collections).<br>
These collections are better way to handling unknown json structures but must be released with the **release --recursive** command.

Because duckscript variables have no type, the json_encode will define every boolean/numeric value as JSON string.<br>

Below is a simple example showing how to parse and encode values of all types when using the default behaviour of storing to variables.

```sh
fn test_simple_types
    str = json_parse \"myvalue\"
    assert_eq ${str} myvalue
    jsonstring = json_encode str
    assert_eq ${jsonstring} \"myvalue\"

    number = json_parse 500
    assert_eq ${number} 500
    jsonstring = json_encode number
    # numeric value is encoded as string
    assert_eq ${jsonstring} \"500\"

    bool = json_parse true
    assert_eq ${bool} true
    jsonstring = json_encode bool
    # boolean value is encoded to string
    assert_eq ${jsonstring} \"true\"

    arr = json_parse "[1, 2, 3]"
    # arr.length is not part of the JSON structure but added as a variable to enable
    # to loop over the array using the range command
    assert_eq ${arr.length} 3
    # direct array location access example
    assert_eq ${arr[0]} 1
    assert_eq ${arr[1]} 2
    assert_eq ${arr[2]} 3
    # array loop example
    arr_range = range 0 ${arr.length}
    for index in ${arr_range}
        expected_value = calc ${index} + 1
        value = get_by_name arr[${index}]
        assert_eq ${value} ${expected_value}
    end

    object = json_parse "{\"str\": \"my string value\", \"number\": 500, \"bool\": true, \"array\": [1, 2, 3]}"
    assert_eq ${object} [OBJECT]
    assert_eq ${object.str} "my string value"
    assert_eq ${object.number} 500
    assert_eq ${object.bool} true
    assert_eq ${object.array.length} 3
    assert_eq ${object.array[0]} 1
    assert_eq ${object.array[1]} 2
    assert_eq ${object.array[2]} 3
    jsonstring = json_encode object
    found = contains ${jsonstring} "\"str\":\"my string value\""
    assert ${found}
    found = contains ${jsonstring} "\"number\":\"500\""
    assert ${found}
    found = contains ${jsonstring} "\"bool\":\"true\""
    assert ${found}
    found = contains ${jsonstring} "\"array\":[\"1\",\"2\",\"3\"]"
    assert ${found}

    # we can cleanup all variables created from the json parse starting from the root object
    unset_all_vars --prefix object
    defined = is_defined object
    assert_false ${defined}
    defined = is_defined object.str
    assert_false ${defined}
    defined = is_defined object.array.length
    assert_false ${defined}
end
```

