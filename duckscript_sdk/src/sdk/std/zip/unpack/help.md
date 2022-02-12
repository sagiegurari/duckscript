```
unzip <zipfile> [target]
```

Unpacks the ZIP file into the `target` directory, if provided, or
into current working directory otherwise.

### Parameters

- zipfile - The path to the ZIP archive to be created.
- target - The directory to unpack files into. If not provided,
  current working directory is used.

### Return value

true if successful (i.e. the zip file exists and there are no existing file conflicts)

### Examples

```sh
# ./stuff directory will be created automatically
unzipped = unzip ./archive.zip ./stuff
```
