```sh
string = json_encode var_name
```

This function will encode all variables, starting from the root variable as a JSON string.<br>
Since duckscript is untyped, all boolean and numeric values will be encoded as strings.<br>
If --collection is passed, the provided value is considered as string or a map/array handle which is used to fetch
the tree data and create the json string.

#### Parameters

* Option --collection flag to make the encoding use the maps/arrays and values
* The root variable name (or a handle/value in case --collection is provided)

#### Return Value

The JSON string

#### Examples

```sh
# will parse and encode to plain variables
package = json_parse "{\"name\": \"my package\", \"version\": 1, \"publish\": false, \"keywords\": [\"test1\", \"test2\"], \"directories\": {\"test\": \"spec\"}}"
jsonstring = json_encode package

# will parse and encode to maps/arrays
package = json_parse --collection "{\"name\": \"my package\", \"version\": 1, \"publish\": false, \"keywords\": [\"test1\", \"test2\"], \"directories\": {\"test\": \"spec\"}}"
jsonstring = json_encode --collection ${package}
```
