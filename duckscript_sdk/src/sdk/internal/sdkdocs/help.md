```sh
doc_file = internal::sdkdocs [prefix] output_file
```

Generates markdown documentation of all known commands and writes them into the provided file.

#### Parameters

* Optional name prefix
* The target file name which will hold the generated documentation.

#### Return Value

The target file name.

#### Examples

```sh
doc_file = internal::sdkdocs ./docs/sdk.md
```
