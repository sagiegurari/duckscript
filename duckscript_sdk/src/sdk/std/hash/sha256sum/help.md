```sh
var = sha256sum file
```

Runs SHA-256 hash on the provided file returns the hashed value in hex.

### Parameters

The file to hash

### Return Value

The hash value in hex or false in case of error.
The result will be in lowercase, same as with the core utils with the same name.

### Examples

```sh
hashed = sha256sum ./myfile.txt
```
