```sh
var = touch file
```

This command will create an empty file and return true/false if the file exists.<br>
If file exits, it will not be modified.

#### Parameters

The file path.

#### Return Value

If the file exists after the command, it will return true.<br>
In case of any error, it will return false.

#### Examples

```sh
exists = touch ./dir/file.txt
```
