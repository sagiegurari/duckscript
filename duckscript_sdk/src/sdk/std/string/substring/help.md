```sh
var = substring text
var = substring text start end
var = substring text start
var = substring text -end
```

The substring command will create a new string value from the text provided in the range requested.

#### Parameters

* The text to substring from
* Additional parameters
    * None - start index is 0 and end index is the text length
    * Two arguments - First is the start index and second is the end index
    * One argument
        * If >= 0 it defines the start index and end index is the text length
        * If < 0 it defines the end index going backwards from the end of the text. Start index is 0.

#### Return Value

The substring value or false in case of error.

#### Examples

```sh
# string is 'Hello World'
string = substring "Hello World"
echo ${string}

# string is 'll'
string = substring "Hello World" 2 4
echo ${string}

# string is 'llo World'
string = substring "Hello World" 2
echo ${string}

# string is 'Hello W'
string = substring "Hello World" -4
echo ${string}
```
