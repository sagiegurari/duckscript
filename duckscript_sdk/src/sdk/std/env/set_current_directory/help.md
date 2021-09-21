```sh
cd path
```

Sets the current directory based on the input path.<br>
If no path is provided, it will default to the user home directory.<br>
If the path does not exist, it will return none.

### Parameters

The new current directory.

### Return Value

The new current directory or none in case of any error such as target directory not found.

### Examples

```sh
# Move to user home directory and store the path in the home variable
home = cd

# Move to the requested directory
cd ./scripts
```
