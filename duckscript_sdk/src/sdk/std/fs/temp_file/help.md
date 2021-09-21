```sh
path = temp_file [extension]
```

This command will create a new empty temporary file and return its path.

### Parameters

Optional file extension.

### Return Value

The file path.

### Examples

```sh
path = temp_file toml

echo ${path}
```
