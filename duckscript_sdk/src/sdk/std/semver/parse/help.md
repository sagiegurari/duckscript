```sh
base = semver_parse value
```

Parses the provided value and sets the major, minor and patch variables.<br>
The variable names are based on the output variable name, for example if the output variable name is out:

* out.major - Holds the output major version
* out.minor - Holds the output minor version
* out.patch - Holds the output patch version

#### Parameters

The semver value.

#### Return Value

The major, minor and patch values.

#### Examples

```sh
version = semver_parse 1.2.3

echo ${version.major}
echo ${version.minor}
echo ${version.patch}
```
