```sh
var = digest --algo (sha256|sha512) (--file path|content)
```

Runs the requested hash on the provided file or string content and returns the hashed value in hex.

#### Parameters

* --algo and algorithm to use (currently sha256 and sha512 are supported)
* Optional --file and file path
* Optional the string content to hash (if file is not provided)

#### Return Value

The hash value in hex or false in case of error.

#### Examples

```sh
hashed = digest --algo sha256 "hello world\n"
assert_eq ${hashed} A948904F2F0F479B8F8197694B30184B0D2ED1C1CD2A1EC0FB85D299A192A447

hashed = digest --algo sha512 --file ./myfile.txt
```
