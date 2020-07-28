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

#### Parameters

The JSON string to parse.

#### Return Value

The root value.

#### Examples

```sh
package = json_parse "{\"name\": \"my package\", \"version\": 1, \"publish\": false, \"keywords\": [\"test1\", \"test2\"], \"directories\": {\"test\": \"spec\"}}"

defined = is_defined package
assert_false ${defined}

assert_eq ${package.name} "my package"
assert_eq ${package.version} 1
assert_eq ${package.publish} false
assert_eq ${package.keywords.length} 2
assert_eq ${package.keywords[0]} test1
assert_eq ${package.keywords[1]} test2
assert_eq ${package.directories.test} spec
```
