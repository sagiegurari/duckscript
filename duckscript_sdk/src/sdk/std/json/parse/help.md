```sh
var = json_parse string
```

This function will parse the provided JSON string and will create variables based on the parsed data.<br>
The variables will reflect the json structure.<br>
Object keys will have name using the json path standard, for example root.child<br>
And arrays will have the array access annotation and length variable, for example:

```sh
root.child[5]
root.child.length
```

In case the --collection flag is provided, it will instead create maps/array as needed and return the root handle (or primitive value) of
the json data.

#### Parameters

* Optional --collection flag to parse and return value/map/array
* The JSON string to parse.

#### Return Value

The root value/handle.

#### Examples

```sh
# parse to simple variables
package = json_parse "{\"name\": \"my package\", \"version\": 1, \"publish\": false, \"keywords\": [\"test1\", \"test2\"], \"directories\": {\"test\": \"spec\"}}"

assert_eq ${package} "[OBJECT]"
assert_eq ${package.name} "my package"
assert_eq ${package.version} 1
assert_eq ${package.publish} false
assert_eq ${package.keywords.length} 2
assert_eq ${package.keywords[0]} test1
assert_eq ${package.keywords[1]} test2
assert_eq ${package.directories.test} spec

# parse to maps/arrays
package = json_parse --collection "{\"name\": \"my package\", \"version\": 1, \"publish\": false, \"keywords\": [\"test1\", \"test2\"], \"directories\": {\"test\": \"spec\"}}"
name = map_get ${package} name
assert_eq ${name} "my package"
version = map_get ${package} version
assert_eq ${version} 1
public = map_get ${package} public
assert_false ${public}
keywords_handle = map_get ${package} keywords
length = array_length ${keywords_handle}
assert_eq ${length} 2
value = array_pop ${keywords_handle}
assert_eq ${value} test2
value = array_pop ${keywords_handle}
assert_eq ${value} test1
directories = map_get ${package} directories
directory = map_get ${directories} test
assert_eq ${directory} spec
```
