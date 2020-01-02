```sh
var = mv source target
```

This command moves the requested source path to the target path.

* If the source and target paths define a file, it will move the file as defined.
* If target path is a directory path, it will move the source file/directory into that target directory path.

All missing parent directories in the target path will be created as needed.

#### Parameters

* The source path to copy
* The target path

#### Return Value

**true** if the move was successful.

#### Examples

```sh
# move a single file
moved = mv ./file1.txt ./file2.txt

# move a single file into the target directory
moved = mv ./file1.txt ./somedir

# move entire directory into another directory
moved = mv ./source ./target/subdir
```
