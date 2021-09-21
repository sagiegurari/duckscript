```sh
result = appendfile file text
```

This command enables to write the provided text into the requested file.<br>
It will return true/false value based if it was able to write the text to the file.<br>
In case the file doesn't exist, it will create it.<br>
If the file exists, it will append the text to it.

### Parameters

* The target file
* The text content to write

### Return Value

true/false based if it was able to write the text to the file.

### Examples

```sh
out = appendfile ./target/tests/writefile.txt "line 1\nline 2"
```
