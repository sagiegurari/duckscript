# Table of Contents
* [sdk::Echo](#sdk__Echo)
* [sdk::Function](#sdk__Function)
* [sdk::Set](#sdk__Set)
* [sdk::env::Get](#sdk__env__Get)
* [sdk::env::Set](#sdk__env__Set)
* [sdk::fs::Print](#sdk__fs__Print)
* [sdk::fs::Read](#sdk__fs__Read)


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

The amount of arguments printed.

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

<a name="sdk__Function"></a>
## sdk::Function
```sh
function my_function
# function content
end_function
```


#### Aliases:
function, fn

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

<a name="sdk__env__Get"></a>
## sdk::env::Get
```sh
var = get_env key
```

Returns the environment variable value for the provided key.

#### Parameters

First argument is the environment variable key.


#### Return Value

The environment variable value.

#### Examples

```sh
home = get_env HOME
```


#### Aliases:
get_env

<a name="sdk__env__Set"></a>
## sdk::env::Set
```sh
set_env key value
```

Sets the environment variable defined by the provided key to the provided value.

#### Parameters

Two arguments are required:

* key - The name of the environment variable to set
* value - The new environment variable value


#### Return Value

None

#### Examples

```sh
set_env HOME /usr/me
```


#### Aliases:
set_env

<a name="sdk__fs__Print"></a>
## sdk::fs::Print
```sh
var = cat file
```

The cat command will print out the requested file.<br>
In addition it will also return the value to the output variable.

#### Parameters

A single parameter holding the file path.


#### Return Value

The file content.

#### Examples

```sh
cat ./docs/sdk.md
```


#### Aliases:
cat

<a name="sdk__fs__Read"></a>
## sdk::fs::Read
```sh
var = readfile file
```

The readfile command will read the requested file and return the value to the output variable.

#### Parameters

A single parameter holding the file path.


#### Return Value

The file content.

#### Examples

```sh
text = readfile ./Cargo.toml
```


#### Aliases:
readfile

### License
Developed by Sagie Gur-Ari and licensed under the
[Apache 2](https://github.com/sagiegurari/duckscript/blob/master/LICENSE) open source license.
