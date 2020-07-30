```sh
string = json_encode var_name
```

This function will encode all variables, starting from the root variable as a JSON string.<br>
Since duckscript is untyped, all boolean and numeric values will be encoded as strings.

#### Parameters

The root variable name

#### Return Value

The JSON string

#### Examples

```sh
package = json_parse "{\"name\": \"my package\", \"version\": 1, \"publish\": false, \"keywords\": [\"test1\", \"test2\"], \"directories\": {\"test\": \"spec\"}}"
jsonstring = json_encode package
```
