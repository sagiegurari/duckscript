```sh
result = chmod mode path
```

This command will update the mode for the given path.<br>
**This command is currently only available for unix like systems and will return false for all others such as windows.**

### Parameters

* The new mode, for example 755
* The path

### Return Value

The new mode as decimal number or false in case of any error.

### Examples

```sh
chmod 777 ./myfile.txt
```
