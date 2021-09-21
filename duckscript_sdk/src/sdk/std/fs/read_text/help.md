```sh
var = readfile file
```

The readfile command will read the requested file and return the value to the output variable.

### Parameters

A single parameter holding the file path.

### Return Value

The file content or none in case file does not exist.

### Examples

```sh
text = readfile ./Cargo.toml
```
