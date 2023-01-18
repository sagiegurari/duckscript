```
zip [--base basedir] [--compression comp] [--append] <zipfile> <files>
```

Packs the provided files into a ZIP archive.

File paths in the archive will be relative to current working directory.

### Parameters

 - zipfile - The path to the ZIP archive to be created.
 - files - One or more file paths to pack. No globbing is performed. However, array
   handles containing the files to pack can be used.
 - Optional base directory via `--base <basedir>` --- this directory will be used
   as a base for the file paths inside the archive.
 - Optional append flag via `--append` --- if set, the files will be added to the
   existing archive. If the archive does not exist, it will be created.
 - Optional compression mode via `--compression comp` where `comp` is one of
   `deflate`, `bzip2`, `none`. If not specified, `deflate` is used by default.

### Return value

true if successful

### Examples

```sh
# create some files for the example
mkdir ./test
touch ./test/foo/bar.txt
touch ./test/baz/qux.png
touch ./test/folder/another/file.doc

# pack the files
zipped = zip ./archive.zip ./test/foo/bar.txt ./test/folder/another/file.doc
```
