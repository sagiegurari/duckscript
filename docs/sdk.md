# Table of Contents
* [internal::sdkdocs](#internal__sdkdocs)
* [sdk::Echo](#sdk__Echo)
* [sdk::Set](#sdk__Set)


<a name="internal__sdkdocs"></a>
## internal::sdkdocs
```sh
doc_file = internal::sdkdocs output_file
```

Generates markdown documentation of all known commands and writes them into the provided file.

#### Parameters

The target file name which will hold the generated documentation.


#### Return Value

The target file name.

#### Examples

```sh
doc_file = internal::sdkdocs ./docs/sdk.md
```


<a name="sdk__Echo"></a>
## sdk::Echo
```sh
echo [arg]*
```

The echo command will printout all provided arguments.<br>
After all input is done, an end of line will be printed as well.

#### Parameters

Any number of arguments may be provided and will be printed.


#### Return Value

None

#### Examples

Print multiple arguments:

```sh
echo hello world
```

Print multiple spaces between words

```sh
echo "hello    world"
```


#### Aliases:
echo

<a name="sdk__Set"></a>
## sdk::Set
```sh
var = set arg
```

The set command will simply return the provided argument and set it to the output variable.

#### Parameters

Only the first argument will be returned.


#### Return Value

The first command argument.

#### Examples

Return a simple text value:

```sh
var = set hello
```

Return an expanded value:

```sh
var = set "home: ${HOME}"
```


#### Aliases:
set
