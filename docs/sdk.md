# Table of Contents
* [std::Echo (echo)](#std__Echo)
* [std::Eval (eval)](#std__Eval)
* [std::IsCommandDefined (is_command_defined)](#std__IsCommandDefined)
* [std::Noop (noop)](#std__Noop)
* [std::Not (not)](#std__Not)
* [std::ReadUserInput (read)](#std__ReadUserInput)
* [std::Release (release)](#std__Release)
* [std::ShowCommandDocumentation (man)](#std__ShowCommandDocumentation)
* [std::collections](#std__collections)
* [std::collections::Array (array)](#std__collections__Array)
* [std::collections::ArrayClear (array_clear)](#std__collections__ArrayClear)
* [std::collections::ArrayConcat (array_concat)](#std__collections__ArrayConcat)
* [std::collections::ArrayContains (array_contains)](#std__collections__ArrayContains)
* [std::collections::ArrayGet (array_get)](#std__collections__ArrayGet)
* [std::collections::ArrayIsEmpty (array_is_empty)](#std__collections__ArrayIsEmpty)
* [std::collections::ArrayJoin (array_join)](#std__collections__ArrayJoin)
* [std::collections::ArrayLength (array_length, arrlen, array_size)](#std__collections__ArrayLength)
* [std::collections::ArrayPop (array_pop)](#std__collections__ArrayPop)
* [std::collections::ArrayPush (array_push, array_add, array_put)](#std__collections__ArrayPush)
* [std::collections::ArrayRemove (array_remove)](#std__collections__ArrayRemove)
* [std::collections::ArraySet (array_set)](#std__collections__ArraySet)
* [std::collections::IsArray (is_array)](#std__collections__IsArray)
* [std::collections::IsMap (is_map)](#std__collections__IsMap)
* [std::collections::IsSet (is_set)](#std__collections__IsSet)
* [std::collections::Map (map)](#std__collections__Map)
* [std::collections::MapClear (map_clear)](#std__collections__MapClear)
* [std::collections::MapContainsKey (map_contains_key)](#std__collections__MapContainsKey)
* [std::collections::MapContainsValue (map_contains_value)](#std__collections__MapContainsValue)
* [std::collections::MapGet (map_get)](#std__collections__MapGet)
* [std::collections::MapIsEmpty (map_is_empty)](#std__collections__MapIsEmpty)
* [std::collections::MapKeys (map_keys)](#std__collections__MapKeys)
* [std::collections::MapLoadProperties (map_load_properties)](#std__collections__MapLoadProperties)
* [std::collections::MapPut (map_put, map_add)](#std__collections__MapPut)
* [std::collections::MapRemove (map_remove)](#std__collections__MapRemove)
* [std::collections::MapSize (map_size)](#std__collections__MapSize)
* [std::collections::MapToProperties (map_to_properties)](#std__collections__MapToProperties)
* [std::collections::Range (range)](#std__collections__Range)
* [std::collections::ReadProperties (read_properties)](#std__collections__ReadProperties)
* [std::collections::Set (set_new)](#std__collections__Set)
* [std::collections::SetClear (set_clear)](#std__collections__SetClear)
* [std::collections::SetContains (set_contains)](#std__collections__SetContains)
* [std::collections::SetFromArray (set_from_array)](#std__collections__SetFromArray)
* [std::collections::SetIsEmpty (set_is_empty)](#std__collections__SetIsEmpty)
* [std::collections::SetPut (set_put, set_add)](#std__collections__SetPut)
* [std::collections::SetRemove (set_remove)](#std__collections__SetRemove)
* [std::collections::SetSize (set_size)](#std__collections__SetSize)
* [std::collections::SetToArray (set_to_array)](#std__collections__SetToArray)
* [std::collections::WriteProperties (write_properties)](#std__collections__WriteProperties)
* [std::debug::DuckscriptSDKVersion (duckscript_sdk_version)](#std__debug__DuckscriptSDKVersion)
* [std::debug::DuckscriptVersion (duckscript_version)](#std__debug__DuckscriptVersion)
* [std::debug::DumpInstructions (dump_instructions)](#std__debug__DumpInstructions)
* [std::debug::DumpState (dump_state)](#std__debug__DumpState)
* [std::debug::DumpVariables (dump_variables)](#std__debug__DumpVariables)
* [std::env::EnvToMap (env_to_map)](#std__env__EnvToMap)
* [std::env::FindExecutable (which)](#std__env__FindExecutable)
* [std::env::GetCpuCount (cpu_count, get_cpu_count)](#std__env__GetCpuCount)
* [std::env::GetHomeDirectory (get_home_dir)](#std__env__GetHomeDirectory)
* [std::env::GetOSFamily (os_family)](#std__env__GetOSFamily)
* [std::env::GetOSName (os_name)](#std__env__GetOSName)
* [std::env::GetOSRelease (os_release)](#std__env__GetOSRelease)
* [std::env::GetOSVersion (os_version)](#std__env__GetOSVersion)
* [std::env::GetUserName (whoami, get_user_name)](#std__env__GetUserName)
* [std::env::GetVar (get_env)](#std__env__GetVar)
* [std::env::IsWindows (is_windows)](#std__env__IsWindows)
* [std::env::PrintCurrentDirectory (pwd, print_current_directory)](#std__env__PrintCurrentDirectory)
* [std::env::PrintEnv (print_env, printenv)](#std__env__PrintEnv)
* [std::env::SetCurrentDirectory (cd, set_current_dir, set_current_directory)](#std__env__SetCurrentDirectory)
* [std::env::SetVar (set_env)](#std__env__SetVar)
* [std::env::UName (uname)](#std__env__UName)
* [std::env::UnsetVar (unset_env)](#std__env__UnsetVar)
* [std::error::GetLastError (get_last_error)](#std__error__GetLastError)
* [std::error::GetLastErrorLine (get_last_error_line)](#std__error__GetLastErrorLine)
* [std::error::GetLastErrorSource (get_last_error_source)](#std__error__GetLastErrorSource)
* [std::error::SetError (set_error)](#std__error__SetError)
* [std::error::SetExitOnError (exit_on_error, set_exit_on_error)](#std__error__SetExitOnError)
* [std::error::TriggerError (trigger_error)](#std__error__TriggerError)
* [std::flowcontrol::ForIn (for)](#std__flowcontrol__ForIn)
* [std::flowcontrol::Function (function, fn)](#std__flowcontrol__Function)
* [std::flowcontrol::GoTo (goto)](#std__flowcontrol__GoTo)
* [std::flowcontrol::If (if)](#std__flowcontrol__If)
* [std::flowcontrol::While (while)](#std__flowcontrol__While)
* [std::fs::Append (appendfile)](#std__fs__Append)
* [std::fs::CopyPath (cp)](#std__fs__CopyPath)
* [std::fs::CreateDirectory (mkdir)](#std__fs__CreateDirectory)
* [std::fs::CreateEmptyFile (touch)](#std__fs__CreateEmptyFile)
* [std::fs::DeleteEmptyDirectory (rmdir)](#std__fs__DeleteEmptyDirectory)
* [std::fs::DeletePath (rm)](#std__fs__DeletePath)
* [std::fs::Exists (is_path_exists)](#std__fs__Exists)
* [std::fs::GetCanonicalPath (canonicalize)](#std__fs__GetCanonicalPath)
* [std::fs::GetFileName (basename)](#std__fs__GetFileName)
* [std::fs::GetLastModifiedTime (get_last_modified_time)](#std__fs__GetLastModifiedTime)
* [std::fs::GetParentDirectory (dirname)](#std__fs__GetParentDirectory)
* [std::fs::GlobArray (glob_array, globarray)](#std__fs__GlobArray)
* [std::fs::IsDirectory (is_directory, is_dir)](#std__fs__IsDirectory)
* [std::fs::IsFile (is_file)](#std__fs__IsFile)
* [std::fs::IsPathNewer (is_path_newer)](#std__fs__IsPathNewer)
* [std::fs::IsReadonly (is_readonly)](#std__fs__IsReadonly)
* [std::fs::List (ls)](#std__fs__List)
* [std::fs::MovePath (mv)](#std__fs__MovePath)
* [std::fs::Print (cat)](#std__fs__Print)
* [std::fs::ReadBytes (readbinfile, read_binary_file)](#std__fs__ReadBytes)
* [std::fs::ReadText (readfile, read_text_file)](#std__fs__ReadText)
* [std::fs::SetMode (chmod)](#std__fs__SetMode)
* [std::fs::SetModeGlob (glob_chmod)](#std__fs__SetModeGlob)
* [std::fs::TempDirectory (temp_dir)](#std__fs__TempDirectory)
* [std::fs::TempFile (temp_file)](#std__fs__TempFile)
* [std::fs::WriteBytes (writebinfile, write_binary_file)](#std__fs__WriteBytes)
* [std::fs::WriteText (writefile, write_text_file)](#std__fs__WriteText)
* [std::json](#std__json)
* [std::json::Encode (json_encode)](#std__json__Encode)
* [std::json::Parse (json_parse)](#std__json__Parse)
* [std::lib::alias::Set (alias)](#std__lib__alias__Set)
* [std::lib::alias::Unset (unalias)](#std__lib__alias__Unset)
* [std::lib::command::Remove (remove_command)](#std__lib__command__Remove)
* [std::math::Calc (calc)](#std__math__Calc)
* [std::math::GreaterThan (greater_than)](#std__math__GreaterThan)
* [std::math::LessThan (less_than)](#std__math__LessThan)
* [std::net::Hostname (hostname)](#std__net__Hostname)
* [std::net::HttpClient (http_client)](#std__net__HttpClient)
* [std::net::WGet (wget)](#std__net__WGet)
* [std::net::ftp::Get (ftp_get)](#std__net__ftp__Get)
* [std::net::ftp::GetInMemory (ftp_get_in_memory)](#std__net__ftp__GetInMemory)
* [std::net::ftp::List (ftp_list)](#std__net__ftp__List)
* [std::net::ftp::NLst (ftp_nlst)](#std__net__ftp__NLst)
* [std::net::ftp::Put (ftp_put)](#std__net__ftp__Put)
* [std::net::ftp::PutInMemory (ftp_put_in_memory)](#std__net__ftp__PutInMemory)
* [std::process::Execute (exec)](#std__process__Execute)
* [std::process::Exit (exit, quit, q)](#std__process__Exit)
* [std::process::ProcessID (pid, process_id)](#std__process__ProcessID)
* [std::process::Spawn (spawn)](#std__process__Spawn)
* [std::process::Watchdog (watchdog)](#std__process__Watchdog)
* [std::random::Range (random_range, rand_range)](#std__random__Range)
* [std::random::Text (random_text, rand_text)](#std__random__Text)
* [std::scope::Clear (clear_scope)](#std__scope__Clear)
* [std::scope::PopStack (scope_pop_stack)](#std__scope__PopStack)
* [std::scope::PushStack (scope_push_stack)](#std__scope__PushStack)
* [std::semver::IsEqual (semver_is_equal)](#std__semver__IsEqual)
* [std::semver::IsNewer (semver_is_newer)](#std__semver__IsNewer)
* [std::semver::Parse (semver_parse)](#std__semver__Parse)
* [std::string::Base64 (base64)](#std__string__Base64)
* [std::string::Base64Decode (base64_decode)](#std__string__Base64Decode)
* [std::string::Base64Encode (base64_encode)](#std__string__Base64Encode)
* [std::string::BytesToString (bytes_to_string)](#std__string__BytesToString)
* [std::string::Concat (concat)](#std__string__Concat)
* [std::string::Contains (contains)](#std__string__Contains)
* [std::string::EndsWith (ends_with)](#std__string__EndsWith)
* [std::string::Equals (equals, eq)](#std__string__Equals)
* [std::string::IndexOf (indexof)](#std__string__IndexOf)
* [std::string::IsEmpty (is_empty)](#std__string__IsEmpty)
* [std::string::LastIndexOf (last_indexof)](#std__string__LastIndexOf)
* [std::string::Length (length, strlen)](#std__string__Length)
* [std::string::Replace (replace)](#std__string__Replace)
* [std::string::Split (split)](#std__string__Split)
* [std::string::StartsWith (starts_with)](#std__string__StartsWith)
* [std::string::StringToBytes (string_to_bytes)](#std__string__StringToBytes)
* [std::string::SubString (substring)](#std__string__SubString)
* [std::string::Trim (trim)](#std__string__Trim)
* [std::string::TrimEnd (trim_end)](#std__string__TrimEnd)
* [std::string::TrimStart (trim_start)](#std__string__TrimStart)
* [std::test::Assert (assert)](#std__test__Assert)
* [std::test::AssertEquals (assert_eq)](#std__test__AssertEquals)
* [std::test::AssertError (assert_error)](#std__test__AssertError)
* [std::test::AssertFail (assert_fail)](#std__test__AssertFail)
* [std::test::AssertFalse (assert_false)](#std__test__AssertFalse)
* [std::test::TestDirectory (test_directory)](#std__test__TestDirectory)
* [std::test::TestFile (test_file)](#std__test__TestFile)
* [std::thread::Sleep (sleep)](#std__thread__Sleep)
* [std::time::CurrentTimeMillies (current_time)](#std__time__CurrentTimeMillies)
* [std::var::GetAllVarNames (get_all_var_names)](#std__var__GetAllVarNames)
* [std::var::GetByName (get_by_name)](#std__var__GetByName)
* [std::var::Set (set)](#std__var__Set)
* [std::var::SetByName (set_by_name)](#std__var__SetByName)
* [std::var::Unset (unset)](#std__var__Unset)
* [std::var::UnsetAllVars (unset_all_vars)](#std__var__UnsetAllVars)


<a name="std__Echo"></a>
## std::Echo
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

```sh
# Print multiple arguments:
echo hello world

# Print multiple spaces between words
echo "hello    world"
```


#### Aliases:
echo

<a name="std__Eval"></a>
## std::Eval
```sh
eval command arguments
```

The eval command enables to run dynamically created commands.<br>
The command and arguments passed can be variables in the form of ${name}.

#### Parameters

Any number of arguments which will construct a line to evaluate and execute.

#### Return Value

The result of the evaluated line.

#### Examples

```sh
command = set echo
eval ${command} hello world
```


#### Aliases:
eval

<a name="std__IsCommandDefined"></a>
## std::IsCommandDefined
```sh
var = is_command_defined key
```

Returns true if the provided command name exists.

#### Parameters

The command name.

#### Return Value

True if the command exists.

#### Examples

```sh
exists = is_command_defined exec
```


#### Aliases:
is_command_defined

<a name="std__Noop"></a>
## std::Noop
```sh
noop
```

Empty function that does nothing and returns none.

#### Parameters

All parameters are ignored

#### Return Value

None

#### Examples

```sh
noop
```


#### Aliases:
noop

<a name="std__Not"></a>
## std::Not
```sh
output = not [command|value|condition]
```

Enables to switch falsy to true and truthy to false.<br>
The **not** commands accept either:

* A command with optional arguments and invokes it
* A single value which doesn't match any known command
* A condition statement

If the result is one of the following:

* No output
* false (case insensitive)
* 0
* no (case insensitive)
* Empty value

It will return true, otherwise it will return false.

A condition statement is made up of values, or/and keywords and '('/')' groups.<br>
Each must be separated with a space character.

#### Parameters

A command and its arguments to invoke and evaluate its output, if a single value is provided an no such command exists, it is evaluated as a value.

#### Return Value

The switched value of the input.

#### Examples

```sh
fn test_not_true
    value = not true

    assert_false ${value}
end

fn test_not_false
    value = not false

    assert ${value}
end

fn test_not_command_true
    value = not set true

    assert_false ${value}
end

fn test_not_command_false
    value = not set false

    assert ${value}
end

fn test_not_condition_true
    value = not true and false or true and false or ( true and true or false )

    assert_false ${value}
end

fn test_not_condition_false
    value = not true and false or true and false or ( true and true or false ) and false

    assert ${value}
end
```


#### Aliases:
not

<a name="std__ReadUserInput"></a>
## std::ReadUserInput
```sh
var = read
```

Reads the user input into the output variable.<br>
If the user didn't insert any input, none will be returned.

#### Parameters

None

#### Return Value

The user input or none if no input was entered.

#### Examples

```sh
echo Enter Full Name:
name = read

if is_empty ${name}
    echo You didn't enter any value
else
    echo Your name is: ${name}
end
```


#### Aliases:
read

<a name="std__Release"></a>
## std::Release
```sh
release [-r|--recursive] handle
```

Releases an internal handle stored in the runtime memory.<br>
Certain commands (such as **array**) will create a handle and the variable will only hold a reference to that handle.<br>
In order to release those handles once they are no longer needed, the release command should be used.<br>
By providing the recursive flag, it will also go over the data values (array items, map values, ...) and release each one of them as well
if they are handles to other arrays/maps/...

#### Parameters

* Optional recursive flag (default false)
* The handle name.

#### Return Value

* true - If a handle was found and removed
* false - If no handle was found

#### Examples

```sh
release ${array_handle}
```


#### Aliases:
release

<a name="std__ShowCommandDocumentation"></a>
## std::ShowCommandDocumentation
```sh
var = man command
```

Prints and returns the help documentation of the provided command.

#### Parameters

The command name.

#### Return Value

The help documentation or if not found, none.

#### Examples

```sh
man set
```


#### Aliases:
man

<a name="std__collections"></a>
## std::collections
The collections module contains commands which enable to interact with different data models such as arrays, sets and maps.

* Arrays are simple ordered list of items
* Sets are unordered unique collection of items
* Maps are key/value (dictionary) structure where the keys are unique

Access to these data structures are done via handles.<br>
Handles are provided by the data structure creation command (such as: array, range, map, set) and are used in all
other commands to read/modify those data structures.<br>
Once done with a specific data structure, you must release it via release command to prevent any memory leaks.



<a name="std__collections__Array"></a>
## std::collections::Array
```sh
handle = array value1 value2 value3 ...
```

Creates an array from the input arguments and returns a handle to that array.<br>
This handle can be passed to other commands which support arrays using handles.<br>
Once the array is no longer used, it should be released using the **release** command.

#### Parameters

Any number of arguments which will construct the array.

#### Return Value

A handle to the array.

#### Examples

```sh
handle = array ${var} "hello world" 5 ${another_var}

# once done we should release the handle
release ${handle}
```


#### Aliases:
array

<a name="std__collections__ArrayClear"></a>
## std::collections::ArrayClear
```sh
result = array_clear handle
```

Clears the provided array.

#### Parameters

The array handle.

#### Return Value

True if successful.

#### Examples

```sh
handle = array

result = array_push ${handle} 1

result = array_is_empty ${handle}
assert_false ${result}

result array_clear ${handle}
assert ${result}

result = array_is_empty ${handle}
assert ${result}

release ${handle}
```


#### Aliases:
array_clear

<a name="std__collections__ArrayConcat"></a>
## std::collections::ArrayConcat

```sh
handle = array_concat [handle]*
```

Concats all provided arrays and returns a handle to a new array with all items.

#### Parameters

Any number of array handles.

#### Return Value

A handle to the new array.

#### Examples

```sh
input1 = range 1 4
input2 = range 4 6
input3 = range 6 8

# new array will contain values from 1-7
arr = array_concat ${input1} ${input2} ${input3}
```


#### Source:

```sh

for scope::array_concat::arg in ${scope::array_concat::arguments}
    if not is_array ${scope::array_concat::arg}
        trigger_error "Invalid input, non array handle or array not found."
    end
end

scope::array_concat::array = array

for scope::array_concat::arg in ${scope::array_concat::arguments}
    for scope::array_concat::item in ${scope::array_concat::arg}
        array_push ${scope::array_concat::array} ${scope::array_concat::item}
    end
end

set ${scope::array_concat::array}

```


#### Aliases:
array_concat

<a name="std__collections__ArrayContains"></a>
## std::collections::ArrayContains

```sh
var = array_contains handle value
```

Returns the first index of the array with the same value as provided.<br>
If not found, false will be returned.

#### Parameters

* The array handle.
* The value

#### Return Value

The value index in the array or false if not found.

#### Examples

```sh
handle = array value1 value2 value3
index = array_contains ${handle} value2
```


#### Source:

```sh

scope::array_contains::index = set false
scope::array_contains::value = set ${scope::array_contains::argument::2}

scope::array_contains::counter = set 0
for scope::array_contains::next_value in ${scope::array_contains::argument::1}
    scope::array_contains::found = equals ${scope::array_contains::next_value} ${scope::array_contains::value}

    if ${scope::array_contains::found}
        scope::array_contains::index = set ${scope::array_contains::counter}
        scope::array_contains::argument::1 = set
    end

    scope::array_contains::counter = calc ${scope:array_contains::counter} + 1
end

set ${scope::array_contains::index}

```


#### Aliases:
array_contains

<a name="std__collections__ArrayGet"></a>
## std::collections::ArrayGet
```sh
var = array_get handle index
```

Returns the element from the array at a given index or none if the index is bigger than the array length.

#### Parameters

* The array handle.
* The element index.

#### Return Value

The element at the given index from the array or none.

#### Examples

```sh
handle = array 1 2 3
element = array_get ${handle} 2
assert_eq ${element} 3
```


#### Aliases:
array_get

<a name="std__collections__ArrayIsEmpty"></a>
## std::collections::ArrayIsEmpty

```sh
var = array_is_empty handle
```

Returns true if the provided array handle is an empty array.

#### Parameters

The array handle.

#### Return Value

True if the provided handle belongs to an empty array.

#### Examples

```sh
values = array
out = array_is_empty ${values}
```


#### Source:

```sh

scope::array_is_empty::length = array_length ${scope::array_is_empty::argument::1}
equals 0 ${scope::array_is_empty::length}

```


#### Aliases:
array_is_empty

<a name="std__collections__ArrayJoin"></a>
## std::collections::ArrayJoin

```sh
var = array_join handle separator
```

Joins all values in the provided array with the provided separator in between each value.

#### Parameters

* An array handle
* The separator to put between each item pair

#### Return Value

The joined string value

#### Examples

```sh
function test_to_string
    arr = array hello world
    string = array_join ${arr} ", "

    release ${arr}

    assert_eq ${string} "hello, world"
end

function test_numbers
    arr = range 1 5
    string = array_join ${arr} ", "

    release ${arr}

    assert_eq ${string} "1, 2, 3, 4"
end

function test_empty_separator
    arr = range 1 5
    string = array_join ${arr} ""

    release ${arr}

    assert_eq ${string} "1234"
end
```


#### Source:

```sh

if not is_array ${scope::array_join::argument::1}
    trigger_error "Invalid input, non array handle or array not found."
end

if not array_is_empty ${scope::array_join::argument::1}
    for scope::array_join::item in ${scope::array_join::argument::1}
        scope::array_join::string = set "${scope::array_join::string}${scope::array_join::item}${scope::array_join::argument::2}"
    end

    if not is_empty ${scope::array_join::argument::2}
        scope::array_join::separatorlen = strlen ${scope::array_join::argument::2}
        scope::array_join::stringlen = strlen ${scope::array_join::string}
        scope::array_join::offset = calc ${scope::array_join::stringlen} - ${scope::array_join::separatorlen}
        scope::array_join::string = substring ${scope::array_join::string} 0 ${scope::array_join::offset}
    end
end

set ${scope::array_join::string}

```


#### Aliases:
array_join

<a name="std__collections__ArrayLength"></a>
## std::collections::ArrayLength
```sh
var = array_length handle
```

Returns the array length based on the provided array handle.

#### Parameters

The array handle.

#### Return Value

The array length.

#### Examples

```sh
handle = array a b c "d e"
len = array_length ${handle}
released = release ${handle}
echo Array length: ${len} released: ${released}

handle = range 0 10
len = array_length ${handle}
released = release ${handle}
echo Array length: ${len} released: ${released}
```


#### Aliases:
array_length, arrlen, array_size

<a name="std__collections__ArrayPop"></a>
## std::collections::ArrayPop
```sh
var = array_pop handle
```

Returns the last element of the array or none if the array is empty.

#### Parameters

The array handle.

#### Return Value

The last element of the array or none if the array is empty.

#### Examples

```sh
handle = array 1 2 3
last_element = array_pop ${handle}
assert_eq ${last_element} 3
```


#### Aliases:
array_pop

<a name="std__collections__ArrayPush"></a>
## std::collections::ArrayPush
```sh
var = array_push handle value
```

Pushes an additional value to an existing array.

#### Parameters

The array handle.

#### Return Value

True if a new value was pushed.

#### Examples

```sh
handle = array 1 2 3
array_push ${handle} 4
last_element = array_pop ${handle}
assert_eq ${last_element} 4
```


#### Aliases:
array_push, array_add, array_put

<a name="std__collections__ArrayRemove"></a>
## std::collections::ArrayRemove
```sh
result = array_remove handle index
```

Removes the item from the array at the given index.<br>
If the array is not found or the index is greater than the array size, this command will return false.<br>
Otherwise it will return true.

#### Parameters

* The array handle.
* The element index.

#### Return Value

True if successful.

#### Examples

```sh
arr = array old

element = array_get ${arr} 0
assert_eq ${element} old

result = array_remove ${arr} 0
assert ${result}

empty = array_is_empty ${arr}
assert ${empty}
```


#### Aliases:
array_remove

<a name="std__collections__ArraySet"></a>
## std::collections::ArraySet
```sh
result = array_set handle index value
```

Updates the array at a given index with the provided value.<br>
If the array is not found or the index is greater than the array size, this command will return false.<br>
Otherwise it will return true.

#### Parameters

* The array handle.
* The element index.
* The element value.

#### Return Value

True if successful.

#### Examples

```sh
arr = array old

element = array_get ${arr} 0
assert_eq ${element} old

result = array_set ${arr} 0 new
assert ${result}

element = array_get ${arr} 0
assert_eq ${element} new
```


#### Aliases:
array_set

<a name="std__collections__IsArray"></a>
## std::collections::IsArray
```sh
var = is_array handle
```

Returns true if the provided value is an array handle.

#### Parameters

The array handle.

#### Return Value

True if the provided value is an array handle.

#### Examples

```sh
arr = array 1 2 3

value = is_array ${arr}
assert ${value}

released = release ${arr}
assert ${released}
```


#### Aliases:
is_array

<a name="std__collections__IsMap"></a>
## std::collections::IsMap
```sh
var = is_map handle
```

Returns true if the provided value is a map handle.

#### Parameters

The map handle.

#### Return Value

True if the provided value is a map handle.

#### Examples

```sh
map_handle = map

value = is_map ${map_handle}
assert ${value}

released = release ${map_handle}
assert ${released}
```


#### Aliases:
is_map

<a name="std__collections__IsSet"></a>
## std::collections::IsSet
```sh
var = is_set handle
```

Returns true if the provided value is a set handle.

#### Parameters

The set handle.

#### Return Value

True if the provided value is a set handle.

#### Examples

```sh
handle = set_new 1 2 3

value = is_set ${handle}
assert ${value}

released = release ${handle}
assert ${released}
```


#### Aliases:
is_set

<a name="std__collections__Map"></a>
## std::collections::Map
```sh
handle = map
```

Creates an empty map and returns a handle to that array.<br>
This handle can be passed to other commands which support maps using handles.<br>
Once the map is no longer used, it should be released using the **release** command.

#### Parameters

None

#### Return Value

A handle to the map.

#### Examples

```sh
handle = map

# once done we should release the handle
release ${handle}
```


#### Aliases:
map

<a name="std__collections__MapClear"></a>
## std::collections::MapClear
```sh
result = map_clear handle
```

Clears the provided map.

#### Parameters

The map handle.

#### Return Value

True if successful.

#### Examples

```sh
handle = map

result = map_put ${handle} a 1

result = map_is_empty ${handle}
assert_false ${result}

result map_clear ${handle}
assert ${result}

result = map_is_empty ${handle}
assert ${result}

release ${handle}
```


#### Aliases:
map_clear

<a name="std__collections__MapContainsKey"></a>
## std::collections::MapContainsKey

```sh
var = map_contains_key handle key
```

Returns true if the provided key was found in the map.

#### Parameters

* The map handle.
* The key

#### Return Value

True if the key was found in the map.

#### Examples

```sh
handle = map
map_put ${handle} key value
found = map_contains_key ${handle} key
```


#### Source:

```sh

scope::map_contains_key::value = map_get ${scope::map_contains_key::argument::1} ${scope::map_contains_key::argument::2}
is_defined scope::map_contains_key::value

```


#### Aliases:
map_contains_key

<a name="std__collections__MapContainsValue"></a>
## std::collections::MapContainsValue

```sh
var = map_contains_value handle value
```

Returns true if the provided value was found in the map.

#### Parameters

* The map handle.
* The value

#### Return Value

True if the value was found in the map.

#### Examples

```sh
handle = map
map_put ${handle} key value
found = map_contains_value ${handle} value
```


#### Source:

```sh

scope::map_contains_value::found = set false
scope::map_contains_value::not_empty = not map_is_empty ${scope::map_contains_value::argument::1}

if ${scope::map_contains_value::not_empty}
    scope::map_contains_value::value = set ${scope::map_contains_value::argument::2}
    scope::map_contains_value::key_array_handle = map_keys ${scope::map_contains_value::argument::1}

    for scope::map_contains_value::item in ${scope::map_contains_value::key_array_handle}
        scope::map_contains_value::next_value = map_get ${scope::map_contains_value::argument::1} ${scope::map_contains_value::item}
        scope::map_contains_value::found = equals ${scope::map_contains_value::next_value} ${scope::map_contains_value::value}

        if ${scope::map_contains_value::found}
            release ${scope::map_contains_value::key_array_handle}
        end
    end
end

release ${scope::map_contains_value::key_array_handle}
set ${scope::map_contains_value::found}

```


#### Aliases:
map_contains_value

<a name="std__collections__MapGet"></a>
## std::collections::MapGet
```sh
value = map_get handle key
```

Returns a the value corresponding to the key from the map.

#### Parameters

* The map handle.
* The key.

#### Return Value

The value corresponding to the key from the map.

#### Examples

```sh
handle = map

result = map_put ${handle} key value
assert_eq ${result} true

value = map_get ${handle} key
assert_eq ${value} value

release ${handle}
```


#### Aliases:
map_get

<a name="std__collections__MapIsEmpty"></a>
## std::collections::MapIsEmpty

```sh
var = map_is_empty handle
```

Returns true if the provided map handle is an empty map.

#### Parameters

The map handle.

#### Return Value

True if the provided handle belongs to an empty map.

#### Examples

```sh
handle = map
map_put ${handle} key value
empty = map_is_empty ${handle}
```


#### Source:

```sh

scope::map_is_empty::length = map_size ${scope::map_is_empty::argument::1}
equals 0 ${scope::map_is_empty::length}

```


#### Aliases:
map_is_empty

<a name="std__collections__MapKeys"></a>
## std::collections::MapKeys
```sh
keys = map_keys handle
```

Returns a handle to an array holding all keys in the provided map handle.

#### Parameters

* The map handle.

#### Return Value

A handle to an array holding all map keys.

#### Examples

```sh
handle = map

result = map_put ${handle} key1 value1
assert_eq ${result} true
result = map_put ${handle} key2 value2
assert_eq ${result} true

keys = map_keys ${handle}
for key in ${keys}
    value = map_get ${handle} ${key}
    echo Key: ${key} Value: ${value}
end

release ${handle}
release ${keys}
```


#### Aliases:
map_keys

<a name="std__collections__MapLoadProperties"></a>
## std::collections::MapLoadProperties
```sh
var = map_load_properties [--prefix prefix] handle text
```

Parsers and loads all properties to the provided map.

#### Parameters

* Optional --prefix and the prefix value
* The map handle.
* The properties text.

#### Return Value

True if successful.

#### Examples

```sh
handle = map

result = map_put ${handle} key value
assert_eq ${result} true

value = map_get ${handle} key
assert_eq ${value} value

release ${handle}
```


#### Aliases:
map_load_properties

<a name="std__collections__MapPut"></a>
## std::collections::MapPut
```sh
var = map_put handle key value
```

Inserts a key-value pair into the map.

#### Parameters

* The map handle.
* The key.
* The new value.

#### Return Value

True if a new value was inserted.

#### Examples

```sh
handle = map

result = map_put ${handle} key value
assert_eq ${result} true

value = map_get ${handle} key
assert_eq ${value} value

release ${handle}
```


#### Aliases:
map_put, map_add

<a name="std__collections__MapRemove"></a>
## std::collections::MapRemove
```sh
value = map_remove handle key
```

Removes a the value corresponding to the key from the map and returns it.

#### Parameters

* The map handle.
* The key.

#### Return Value

The value corresponding to the key from the map.

#### Examples

```sh
handle = map

result = map_put ${handle} key value
assert_eq ${result} true

value = map_remove ${handle} key
assert_eq ${value} value

release ${handle}
```


#### Aliases:
map_remove

<a name="std__collections__MapSize"></a>
## std::collections::MapSize
```sh
var = map_size handle
```

Returns the map size based on the provided map handle.

#### Parameters

The map handle.

#### Return Value

The map size.

#### Examples

```sh
handle = map

result = map_put ${handle} a 1
result = map_put ${handle} b 2
result = map_put ${handle} c 3

result = map_size ${handle}
assert_eq ${result} 3

release ${handle}
```


#### Aliases:
map_size

<a name="std__collections__MapToProperties"></a>
## std::collections::MapToProperties
```sh
text = map_to_properties [--prefix prefix] handle
```

Converts the provided map to properties text.

#### Parameters

* Optional --prefix and the prefix value
* The map handle.

#### Return Value

The properties text.

#### Examples

```sh
handle = map
map_put ${handle} a 1
map_put ${handle} b 2
map_put ${handle} a.b.c 123

text = map_to_properties ${handle}
```


#### Aliases:
map_to_properties

<a name="std__collections__Range"></a>
## std::collections::Range
```sh
handle = range start end
```

Creates an array from the input start and end range values and returns a handle to that array.<br>
This handle can be passed to other commands which support arrays using handles.<br>
Once the array is no longer used, it should be released using the **release** command.

#### Parameters

* The start numeric value
* The end numeric value which cannot be smaller than the start value.

#### Return Value

A handle to the array.

#### Examples

```sh
handle = range 1 10

# once done we should release the handle
release ${handle}
```


#### Aliases:
range

<a name="std__collections__ReadProperties"></a>
## std::collections::ReadProperties
```sh
count = read_properties [--prefix key] text
```

Parses the properties (based on java properties format) text and sets them as variables.<br>
This command will also return the count of properties read.<br>
If prefix is provided, all properties read, will be stored as variables with the **prefix.** as their prefix.

#### Parameters

* Optional --prefix and the prefix value
* The text to parse.

#### Return Value

The properties count.

#### Examples

```sh
count = read_properties "a=1\nb=2\na.b.c=3"
assert_eq ${count} 3

assert_eq ${a} 1
assert_eq ${b} 2
assert_eq ${a.b.c} 3

count = read_properties --prefix config a=1\nb=2\na.b.c=3
assert_eq ${count} 3

assert_eq ${config.a} 1
assert_eq ${config.b} 2
assert_eq ${config.a.b.c} 3
```


#### Aliases:
read_properties

<a name="std__collections__Set"></a>
## std::collections::Set
```sh
handle = set_new value1 value2 value3 ...
```

Creates a new set from the input arguments and returns a handle to that set.<br>
This handle can be passed to other commands which support sets using handles.<br>
Once the set is no longer used, it should be released using the **release** command.

#### Parameters

Any number of arguments which will construct the set.

#### Return Value

A handle to the set.

#### Examples

```sh
handle = set_new ${var} "hello world" 5 ${another_var}

# once done we should release the handle
release ${handle}
```


#### Aliases:
set_new

<a name="std__collections__SetClear"></a>
## std::collections::SetClear
```sh
result = set_clear handle
```

Clears the provided set.

#### Parameters

The set handle.

#### Return Value

True if successful.

#### Examples

```sh
handle = set

result = set_put ${handle} 1

result = set_is_empty ${handle}
assert_false ${result}

result set_clear ${handle}
assert ${result}

result = set_is_empty ${handle}
assert ${result}

release ${handle}
```


#### Aliases:
set_clear

<a name="std__collections__SetContains"></a>
## std::collections::SetContains
```sh
var = set_contains handle value
```

Returns true if the set contains the provided value.

#### Parameters

* The set handle.
* The value

#### Return Value

True if the value was found in the set.

#### Examples

```sh
handle = set_new value1 value2 value3
found = set_contains ${handle} value2
```


#### Aliases:
set_contains

<a name="std__collections__SetFromArray"></a>
## std::collections::SetFromArray

```sh
set_handle = set_from_array array_handle
```

Returns a set handle created from the provided array values.

#### Parameters

The array handle.

#### Return Value

The new set handle.

#### Examples

```sh
array_handle = array value1 value2 value3
set_handle = set_from_array ${handle}
```


#### Source:

```sh

if not is_array ${scope::set_from_array::argument::1}
    trigger_error "Invalid input, non array handle or array not found."
end

scope::set_from_array::set = set_new
for scope::set_from_array::next_value in ${scope::set_from_array::argument::1}
    set_put ${scope::set_from_array::set} ${scope::set_from_array::next_value}
end

set ${scope::set_from_array::set}

```


#### Aliases:
set_from_array

<a name="std__collections__SetIsEmpty"></a>
## std::collections::SetIsEmpty

```sh
var = set_is_empty handle
```

Returns true if the provided set handle is an empty set.

#### Parameters

The set handle.

#### Return Value

True if the provided handle belongs to an empty set.

#### Examples

```sh
handle = set
set_put ${handle} value
empty = set_is_empty ${handle}
```


#### Source:

```sh

scope::set_is_empty::length = set_size ${scope::set_is_empty::argument::1}
equals 0 ${scope::set_is_empty::length}

```


#### Aliases:
set_is_empty

<a name="std__collections__SetPut"></a>
## std::collections::SetPut
```sh
var = set_put handle value
```

Pushes an additional value to an existing set.

#### Parameters

The set handle.

#### Return Value

True if a new value was pushed.

#### Examples

```sh
handle = set_new 1 2 3
set_put ${handle} 4
size = set_size ${handle}
assert_eq ${size} 4
```


#### Aliases:
set_put, set_add

<a name="std__collections__SetRemove"></a>
## std::collections::SetRemove
```sh
removed = set_remove handle value
```

Removes a the value from the set and returns true/false if it was removed.

#### Parameters

* The set handle.
* The value to remove.

#### Return Value

True if the value was found and removed from the set.

#### Examples

```sh
handle = set_new

result = set_put ${handle} value
assert_eq ${result} true

removed = set_remove ${handle} value
assert ${removed}

release ${handle}
```


#### Aliases:
set_remove

<a name="std__collections__SetSize"></a>
## std::collections::SetSize
```sh
var = set_size handle
```

Returns the set size based on the provided set handle.

#### Parameters

The set handle.

#### Return Value

The set size.

#### Examples

```sh
handle = set

result = set_put ${handle} 1
result = set_put ${handle} 2
result = set_put ${handle} 3

result = set_size ${handle}
assert_eq ${result} 3

release ${handle}
```


#### Aliases:
set_size

<a name="std__collections__SetToArray"></a>
## std::collections::SetToArray
```sh
array_handle = set_to_array set_handle
```

Converts the provided set to an array and returns the new array handle.

#### Parameters

The set handle.

#### Return Value

The array handle or false in case of error.

#### Examples

```sh
set_handle = set_new value1 value2 value3
array_handle = set_to_array ${set_handle}
```


#### Aliases:
set_to_array

<a name="std__collections__WriteProperties"></a>
## std::collections::WriteProperties
```sh
text = write_properties [--prefix prefix] [names]
```

Creates a properties string from the provided list of variable names (not values).

#### Parameters

* Optional prefix which will be added to all written properties.
* A list of variable names.

#### Return Value

The properties text value.

#### Examples

```sh
a = set 1
b = set 2
a.b.c = set 3

# text will be equal to:
# a=1
# b=2
# a.b.c=3
text = write_properties a b a.b.c

# text will be equal to:
# P.a=1
# P.b=2
# P.a.b.c=3
text = write_properties --prefix P a b a.b.c
```


#### Aliases:
write_properties

<a name="std__debug__DuckscriptSDKVersion"></a>
## std::debug::DuckscriptSDKVersion
```sh
var = duckscript_sdk_version
```

Returns the duckscript SDK version.

#### Parameters

None

#### Return Value

The duckscript SDK version.

#### Examples

```sh
version = duckscript_sdk_version 
```


#### Aliases:
duckscript_sdk_version

<a name="std__debug__DuckscriptVersion"></a>
## std::debug::DuckscriptVersion
```sh
var = duckscript_version
```

Returns the duckscript runtime version.

#### Parameters

None

#### Return Value

The duckscript runtime version.

#### Examples

```sh
version = duckscript_version 
```


#### Aliases:
duckscript_version

<a name="std__debug__DumpInstructions"></a>
## std::debug::DumpInstructions
```sh
value = dump_instructions
```

Returns all script instructions structure (not script text) in textual form.

#### Parameters

None

#### Return Value

The script instructions.

#### Examples

```sh
value = dump_instructions
found = contains ${value} dump_instructions
assert found
```


#### Aliases:
dump_instructions

<a name="std__debug__DumpState"></a>
## std::debug::DumpState
```sh
value = dump_state
```

Returns all script state in textual form.

#### Parameters

None

#### Return Value

The script state.

#### Examples

```sh
numbers = range -5 15

text = dump_instructions
found = contains ${text} -5
assert found
```


#### Aliases:
dump_state

<a name="std__debug__DumpVariables"></a>
## std::debug::DumpVariables
```sh
value = dump_variables
```

Returns all script variables in textual form.

#### Parameters

None

#### Return Value

The script variables.

#### Examples

```sh
one = set 1
two = set 2
values = array 1 2 yes true
numbers = range -5 15

text = dump_variables
found = contains ${text} two
assert found
found = contains ${text} 2
assert found
found = contains ${text} handle
assert found
```


#### Aliases:
dump_variables

<a name="std__env__EnvToMap"></a>
## std::env::EnvToMap
```sh
handle = env_to_map
```

Converts all environment variables to a map and returns the map handle.

#### Parameters

None

#### Return Value

The map handle.

#### Examples

```sh
set_env env_to_map_test test_value

handle = env_to_map

value = map_get ${handle} env_to_map_test
assert_eq ${value} test_value

release ${handle}
```


#### Aliases:
env_to_map

<a name="std__env__FindExecutable"></a>
## std::env::FindExecutable
```sh
var = which executable
```

Returns the path to the executable if it exists.<br>
If not found it will return an empty string.

#### Parameters

The executable to find.

#### Return Value

The executable path or empty string if not found.

#### Examples

```sh
path = which echo
```


#### Aliases:
which

<a name="std__env__GetCpuCount"></a>
## std::env::GetCpuCount
```sh
var = cpu_count
```

Returns the number of CPUs.

#### Parameters

None

#### Return Value

The CPU count.

#### Examples

```sh
count = cpu_count
```


#### Aliases:
cpu_count, get_cpu_count

<a name="std__env__GetHomeDirectory"></a>
## std::env::GetHomeDirectory
```sh
var = get_home_dir
```

Returns the user home directory path.<br>
In case of any error, false will be returned.

#### Parameters

None

#### Return Value

The user home directory path or false in case of any error.

#### Examples

```sh
directory = get_home_dir
```


#### Aliases:
get_home_dir

<a name="std__env__GetOSFamily"></a>
## std::env::GetOSFamily
```sh
var = os_family
```

Returns the OS family (windows, linux, mac).

#### Parameters

None

#### Return Value

The OS family (windows, linux, mac).

#### Examples

```sh
name = os_family
```


#### Aliases:
os_family

<a name="std__env__GetOSName"></a>
## std::env::GetOSName
```sh
var = os_name
```

Returns the OS name.

#### Parameters

None

#### Return Value

The OS name.

#### Examples

```sh
name = os_name
```


#### Aliases:
os_name

<a name="std__env__GetOSRelease"></a>
## std::env::GetOSRelease
```sh
var = os_release
```

Returns the OS release.<br>
**This command is not supported on windows.**

#### Parameters

None

#### Return Value

The OS release.

#### Examples

```sh
release = os_release
```


#### Aliases:
os_release

<a name="std__env__GetOSVersion"></a>
## std::env::GetOSVersion
```sh
var = os_version
```

Returns the OS version.<br>
**This command is not supported on windows.**

#### Parameters

None

#### Return Value

The OS version.

#### Examples

```sh
version = os_version
```


#### Aliases:
os_version

<a name="std__env__GetUserName"></a>
## std::env::GetUserName
```sh
var = whoami
```

Returns the current user name.

#### Parameters

None

#### Return Value

The current user name.

#### Examples

```sh
name = whoami
```


#### Aliases:
whoami, get_user_name

<a name="std__env__GetVar"></a>
## std::env::GetVar
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

<a name="std__env__IsWindows"></a>
## std::env::IsWindows

```sh
var = is_windows
```

Returns true if the current OS family is windows.

#### Parameters

None

#### Return Value

True if the current OS family is windows.

#### Examples

```sh
windows = is_windows
```


#### Source:

```sh

scope::is_windows::os = os_family
equals ${scope::is_windows::os} windows

```


#### Aliases:
is_windows

<a name="std__env__PrintCurrentDirectory"></a>
## std::env::PrintCurrentDirectory
```sh
var = pwd
```

Prints and also returns the current directory.

#### Parameters

None

#### Return Value

The current directory path.

#### Examples

```sh
# Print the current directory:
pwd

# Print and also store the current directory:
directory = pwd
```


#### Aliases:
pwd, print_current_directory

<a name="std__env__PrintEnv"></a>
## std::env::PrintEnv

```sh
var = printenv
```

Prints and returns all environment variables.

#### Parameters

None

#### Return Value

All environment variables printout text.

#### Examples

```sh
set_env TEST_PRINT_ENV TRUE

text = printenv

valid = contains ${text} TEST_PRINT_ENV=TRUE
assert ${valid}
```


#### Source:

```sh

scope::print_env::map = env_to_map
scope::print_env::text = map_to_properties ${scope::print_env::map}
release ${scope::print_env::map}

echo ${scope::print_env::text}
set ${scope::print_env::text}

```


#### Aliases:
print_env, printenv

<a name="std__env__SetCurrentDirectory"></a>
## std::env::SetCurrentDirectory
```sh
cd path
```

Sets the current directory based on the input path.<br>
If no path is provided, it will default to the user home directory.<br>
If the path does not exist, it will return none.

#### Parameters

The new current directory.

#### Return Value

The new current directory or none in case of any error such as target directory not found.

#### Examples

```sh
# Move to user home directory and store the path in the home variable
home = cd

# Move to the requested directory
cd ./scripts
```


#### Aliases:
cd, set_current_dir, set_current_directory

<a name="std__env__SetVar"></a>
## std::env::SetVar
```sh
var = set_env key value
```

Sets the environment variable defined by the provided key to the provided value.

#### Parameters

Two arguments are required:

* key - The name of the environment variable to set
* value - The new environment variable value

#### Return Value

true if successful

#### Examples

```sh
set_env HOME /usr/me
```


#### Aliases:
set_env

<a name="std__env__UName"></a>
## std::env::UName

```sh
var = uname [-a]
```

Acts similar to uname on unix like systems.

#### Parameters

* Optional -a for extended information (not supported on windows).

#### Return Value

The OS name and optionally extra information.

#### Examples

```sh
value = uname -a
```


#### Source:

```sh

scope::uname::extended_info = equals -a ${scope::uname::argument::1}
scope::uname::info = os_name

scope::uname::not_windows = not is_windows

if ${scope::uname::extended_info} and ${scope::uname::not_windows}
    scope::uname::release = os_release
    scope::uname::version = os_version
    scope::uname::info = set "${scope::uname::info} ${scope::uname::release} ${scope::uname::version}"
end

set ${scope::uname::info}

```


#### Aliases:
uname

<a name="std__env__UnsetVar"></a>
## std::env::UnsetVar
```sh
unset_env key
```

Removes the environment variable defined by the provided key.

#### Parameters

The name of the environment variable to remove

#### Return Value

None

#### Examples

```sh
unset_env HOME
```


#### Aliases:
unset_env

<a name="std__error__GetLastError"></a>
## std::error::GetLastError
```sh
var = get_last_error
```

In case of any runtime error, this function will return the error message.

#### Parameters

None

#### Return Value

The last error message or none

#### Examples

```sh
# This will trigger an error
assert_fail

error = get_last_error
echo Error Message: ${error}
```


#### Aliases:
get_last_error

<a name="std__error__GetLastErrorLine"></a>
## std::error::GetLastErrorLine
```sh
var = get_last_error_line
```

In case of any runtime error, this function will return the error line (if available).

#### Parameters

None

#### Return Value

The last error line or none

#### Examples

```sh
# This will trigger an error
assert_fail

line = get_last_error_line
echo Error Line: ${line}
```


#### Aliases:
get_last_error_line

<a name="std__error__GetLastErrorSource"></a>
## std::error::GetLastErrorSource
```sh
var = get_last_error_source
```

In case of any runtime error, this function will return the error source (such as file name) if available.

#### Parameters

None

#### Return Value

The last error source or none

#### Examples

```sh
# This will trigger an error
assert_fail

source = get_last_error_source
echo Error Source File: ${source}
```


#### Aliases:
get_last_error_source

<a name="std__error__SetError"></a>
## std::error::SetError
```sh
set_error message
```

Sets the last error which is accessible via get_last_error.<br>
This command will not trigger the on_error command flow.

#### Parameters

The error message.

#### Return Value

None

#### Examples

```sh
set_error "my error message"

error = get_last_error

assert_eq ${error} "my error message"
```


#### Aliases:
set_error

<a name="std__error__SetExitOnError"></a>
## std::error::SetExitOnError
```sh
var = exit_on_error value
```

Enables to cause the script execution to stop in case of any error.<br>
By default all errors simply trigger the on_error command which the default SDK stores and provides access to.<br>
However, with this command you can change the on_error command to instead stop the script execution.

#### Parameters

If no argument is provided, it will return the current state.<br>
If an argument is provided, it will modify the state and return it as true/false.

#### Return Value

The current/updated state as true/false value

#### Examples

```sh
# Get current state
will_exit = exit_on_error
echo Current state: ${will_exit}

# Update the current state
will_exit = exit_on_error true
echo Current state: ${will_exit}
```


#### Aliases:
exit_on_error, set_exit_on_error

<a name="std__error__TriggerError"></a>
## std::error::TriggerError
```sh
trigger_error [message]
```

Triggers an error that will trigger the on_error flow.

#### Parameters

Optional error message.

#### Return Value

None

#### Examples

```sh
trigger_error "my error message"
error = get_last_error
assert_eq ${error} "my error message"
```


#### Aliases:
trigger_error

<a name="std__flowcontrol__ForIn"></a>
## std::flowcontrol::ForIn
```sh
args = array a b c
for arg in ${args}
    # commands
end
release args
```

The for/in command enables to iterate over an array (see [array command](#std__collections__Array)).<br>
The first argument will contain the current iteration value from the array.<br>
Once all values have been read, it will exit the loop.

#### Parameters

* for
  * The variable name which will hold the current iteration value
  * The string "in"
  * The handle to the array of values to iterate
* end - no parameters

#### Return Value

None

#### Examples

```sh
# Simple example iteration over the list of letters:
args = array a b c

for arg in ${args}
    echo current arg is: ${arg}
end

release args

# Example nested loops:
args = array 1 2 3
for i in ${args}
    for j in ${args}
        echo i: ${i} j: ${j}
    end
end
```


#### Aliases:
for

<a name="std__flowcontrol__Function"></a>
## std::flowcontrol::Function
```sh
fn my_function
    # function content
    return output
end

fn <scope> another_function
    # function content
end
```

This command provides the function language feature as a set of commands:

* function/fn - Defines a function start block
* end - Defines the end of the function block
* return - Allows to exit a function at any point and return an output
* *&lt;scope&gt;* - Optional annotation which enables to use a new scope during the function invocation.
* *function name* - Dynamically created commands based on the function name which are used to invoke the function code.

When a function command is detected, it will search for the end command that comes after.<br>
That entire block is considered the function code block (functions cannot be nested in outer functions)<br>

In order to invoke the function, simply call the function name with any amount of parameters.<br>
Those parameters will be set as ${1}, ${2}, ... and so on.<br>
Since variables are global, it will overwrite any older values stored in those variables.<br>

To exit a function and return a value, simply use the **return** command with the value you want to return.<br>
The variable that was used when the function was originally called, will now store that value.<br>
The return command can be used to exit early without any value.<br>
In case the code reached the **end** call, the function will exit but will not return a value.<br>

The *&lt;scope&gt;* annotation enables to start a new scope when running the function.<br>
All variables defined will not be available except the variables provided to the function as arguments.<br>
All variables created during the function invocation will be deleted once the function ends, except the return value.<br>
This enables a clean function invocation without impacting the global variables.

#### Parameters

* function - The function name used later on to invoke the function
* end - no parameters
* return - optional single paramter to return as an output of the function call
* *&lt;scope&gt;* - Optional annotation which enables to use a new scope during the function invocation.
* *function name* - Any number of arguments which will automatically be set as global variables: ${1}, ${2}, ... as so on.

#### Return Value

The function invocation returns the output provided by the return command.

#### Examples

```sh
# Simple example of a function definition which echo 'hello world' and exits.

# function start
fn hello_world
    echo hello world
end

# function invocation
hello_world

# Example of calling a function and returning a value
fn get_hello_world
    return "hello world"
end

# function invocation
text = get_hello_world

# this will print "hello world"
echo ${text}

# Example of passing arguments
# Also the function is with scope annotation so it has no access
# to any variable except those provided during the function invocation.
fn <scope> print_input
    # ${1} is set with the value 'hello'
    # ${2} is set with the value 'world'
    echo ${1} ${2}
end

print_input hello world

# Functions can call other functions
fn get_one
    return 1
end

fn get_number
    number = get_one
    return ${number}
end

output = get_number

# this will print 1
echo ${output}
```


#### Aliases:
function, fn

<a name="std__flowcontrol__GoTo"></a>
## std::flowcontrol::GoTo
```sh
goto :label
```

The goto command enables you to jump to any position in the script, if that position has a label value.

#### Parameters

A single valid label value.

#### Return Value

None

#### Examples

```sh
goto :good

echo bad

:good echo good
```


#### Aliases:
goto

<a name="std__flowcontrol__If"></a>
## std::flowcontrol::If
```sh
if [command|value|condition]
    # commands
elseif [command|value|condition]
    # commands
else
    # commands
end
```

This command provides the if/elseif/else condition language feature as a set of commands:

* if - Defines an if condition
* elseif - Defines optional secondary condition blocks
* else - Optinoal fallback block
* end - Defines the end of the entire if/else block

if and elseif commands accept either:

* A command with optional arguments and invokes it
* A single value which doesn't match any known command
* A condition statement

If the result is one of the following:

* No output
* false (case insensitive)
* 0
* no (case insensitive)
* Empty value

It is considered falsy.<br>
In case of falsy value, it will skip to the next elseif/else block.<br>
If a truthy (non falsy) output is found, it will invoke the commands of that code block and ignore all other elseif/else blocks.<br>

if blocks can be nested in other if blocks (see examples).

A condition statement is made up of values, or/and keywords and '('/')' groups.<br>
Each must be separated with a space character.

#### Parameters

* if/elseif - A command and its arguments to invoke and evaluate its output, if a single value is provided an no such command exists, it is evaluated as a value.
* else/end - no parameters

#### Return Value

None

#### Examples

```sh
# Simple example of an if statement that evaluates the argument value as true and echos "in if"
if true
    echo in if
end

# Example of using **not** command to reverse the output value
if not false
    echo in if
end

# Example of an if statement that evaluates the command as true and echos "in if"
if set true
    echo in if
end

# Example of if condition returning a falsy result and navigation goes to the else block which echos "in else"
if set false
    echo should not be here
else
    echo in else
end

# Example of if condition returning a falsy result and navigation goes to the elseif block has a truthy condition
if set false
    echo should not be here
elseif set true
    echo in else if
else
    echo should not be here
end

# Nested if example:
if set false
    echo should not be here
elseif set true
    echo in else if but not done yet

    if set true
        echo nested if
    end
else
    echo should not be here
end

valid = set false
if true and false or true and false or ( true and true or false )
    valid = set true
end
assert ${valid}

if true and false or true and false or ( true and true or false ) and false
    assert_fail
end
```


#### Aliases:
if

<a name="std__flowcontrol__While"></a>
## std::flowcontrol::While
```sh
while [command|value|condition]
    # commands
end
```

This command provides the while loop language feature as a set of commands:

* while - Defines a while condition and start of loop
* end - Defines the end of the while block

The while command accept either:

* A command with optional arguments and invokes it
* A single value which doesn't match any known command
* A condition statement

If the result is one of the following:

* No output
* false (case insensitive)
* 0
* no (case insensitive)
* Empty value

It is considered falsy.<br>
In case of falsy value, it will skip to the next line after the while block.<br>
If a truthy (non falsy) output is found, it will invoke the commands of that code block and go back to the start of the while condition.<br>

while blocks can be nested in other while blocks (see examples).

A condition statement is made up of values, or/and keywords and '('/')' groups.<br>
Each must be separated with a space character.

#### Parameters

* while - A command and its arguments to invoke and evaluate its output, if a single value is provided an no such command exists, it is evaluated as a value.
* end - no parameters

#### Return Value

None

#### Examples

```sh
top_count = set 0
inner_count = set 0
counter = set 0
while not equals ${top_count} 10
    top_count = calc ${top_count} + 1
    inner_count = set 0

    while not equals ${inner_count} 10
        inner_count = calc ${inner_count} + 1
        counter = calc ${counter} + 1
    end
end

assert_eq ${counter} 100
```


#### Aliases:
while

<a name="std__fs__Append"></a>
## std::fs::Append
```sh
result = appendfile file text
```

This command enables to write the provided text into the requested file.<br>
It will return true/false value based if it was able to write the text to the file.<br>
In case the file doesn't exist, it will create it.<br>
If the file exists, it will append the text to it.

#### Parameters

* The target file
* The text content to write

#### Return Value

true/false based if it was able to write the text to the file.

#### Examples

```sh
out = appendfile ./target/tests/writefile.txt "line 1\nline 2"
```


#### Aliases:
appendfile

<a name="std__fs__CopyPath"></a>
## std::fs::CopyPath
```sh
var = cp source target
```

This command copies the requested file or directory to the target location.<br>
If the source directory is not empty, its entire contents will be copied as well.

#### Parameters

* The source path to copy
* The target path

#### Return Value

**true** if the path was copied.

#### Examples

```sh
# copy a single file
copied = cp ./file1.txt ./file2.txt

# copy a directory
copied = cp ./source ./target
```


#### Aliases:
cp

<a name="std__fs__CreateDirectory"></a>
## std::fs::CreateDirectory
```sh
var = mkdir directory
```

This command will create the requested directory (and needed parent directories) and return true/false if it was successful.

#### Parameters

The directory name to create.

#### Return Value

The operation success value - true if directory exists, else false.

#### Examples

```sh
exists = mkdir ./dir/subdir
```


#### Aliases:
mkdir

<a name="std__fs__CreateEmptyFile"></a>
## std::fs::CreateEmptyFile
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


#### Aliases:
touch

<a name="std__fs__DeleteEmptyDirectory"></a>
## std::fs::DeleteEmptyDirectory
```sh
var = rmdir path
```

This command delete the requested empty directory and returns true if successful.<br>
If the path leads to a file or a directory which is not empty, this command will fail.

#### Parameters

A single parameter holding the directory path.

#### Return Value

**true** if the directory was deleted.

#### Examples

```sh
deleted = rmdir ./mydir
```


#### Aliases:
rmdir

<a name="std__fs__DeletePath"></a>
## std::fs::DeletePath
```sh
var = rm [-r] path
```

This command delete the requested file, empty directory or recursively deletes a directory
and all its content (files and sub directories) if the **-r** flag is provided.

#### Parameters

* Optional flags (currently only -r is supported which indicates recursive deletion)
* The path to delete

#### Return Value

**true** if the path was deleted.

#### Examples

```sh
# delete a file or empty directory
deleted = rm ./target

# deletes a directory and all its content
deleted = rm -r ./target
```


#### Aliases:
rm

<a name="std__fs__Exists"></a>
## std::fs::Exists
```sh
var = is_path_exists path
```

This command will return true/false based if the provided path points to an existing file system entry.

#### Parameters

The path to check.

#### Return Value

True if the path points to an existing file system entry.

#### Examples

```sh
existing = is_path_exists ./dir
existing = is_path_exists ./dir/somefile.txt
```


#### Aliases:
is_path_exists

<a name="std__fs__GetCanonicalPath"></a>
## std::fs::GetCanonicalPath
```sh
var = canonicalize path
```

This command will return the c path for the provided input.<br>
In case unable, it will return the original input.

#### Parameters

The file/directory path to canonicalize.

#### Return Value

The canonicalized path, or if unsuccessful, the original path.

#### Examples

```sh
path = canonicalize ./target
```


#### Aliases:
canonicalize

<a name="std__fs__GetFileName"></a>
## std::fs::GetFileName
```sh
var = basename path
```

This command will return the last path element of the provided path.<br>
If unable, it will return none.

#### Parameters

The path to extract the last element from.

#### Return Value

The last path element or none if unsuccessful.

#### Examples

```sh
file = basename ./dir/file.txt
```


#### Aliases:
basename

<a name="std__fs__GetLastModifiedTime"></a>
## std::fs::GetLastModifiedTime
```sh
var = get_last_modified_time path
```

This command will return the last modified time in millies from unix epoch.

#### Parameters

The path to check.

#### Return Value

The last modified time in millies from unix epoch or false in case path does not exist.

#### Examples

```sh
time = get_last_modified_time ./dir/somefile.txt
```


#### Aliases:
get_last_modified_time

<a name="std__fs__GetParentDirectory"></a>
## std::fs::GetParentDirectory
```sh
var = dirname path
```

This command will return the parent path of the provided path.<br>
If the parent path is empty, it will return none.

#### Parameters

The path to extract the parent path from.

#### Return Value

The parent path or none.

#### Examples

```sh
directory = dirname ./dir/file.txt
```


#### Aliases:
dirname

<a name="std__fs__GlobArray"></a>
## std::fs::GlobArray
```sh
handle = glob_array pattern
```

Returns an array handle containing all path entries found from the provided glob pattern.<br>
The pattern can be a relative path from current directory or an absolute path.

#### Parameters

The glob pattern.

#### Return Value

The array handle.

#### Examples

```sh
handle = glob_array ./somedir/**/*.txt

for path in ${handle}
    echo ${path}
end
```


#### Aliases:
glob_array, globarray

<a name="std__fs__IsDirectory"></a>
## std::fs::IsDirectory
```sh
var = is_dir path
```

This command will return true/false based if the provided path points to an existing directory.

#### Parameters

The path to check.

#### Return Value

True if the path points to an existing directory.

#### Examples

```sh
existing_dir = is_dir ./dir
```


#### Aliases:
is_directory, is_dir

<a name="std__fs__IsFile"></a>
## std::fs::IsFile
```sh
var = is_file path
```

This command will return true/false based if the provided path points to an existing file.

#### Parameters

The path to check.

#### Return Value

True if the path points to an existing file.

#### Examples

```sh
existing_file = is_file ./dir/somefile.txt
```


#### Aliases:
is_file

<a name="std__fs__IsPathNewer"></a>
## std::fs::IsPathNewer
```sh
var = is_path_newer newer older
```

This command will return true if the 'newer' path last modified time is after the 'older' path last modified time.

#### Parameters

* newer - The file/directory path to check.
* older - The file/directory path to check.

#### Return Value

True if the 'newer' path last modified time is after the 'older' path last modified time.
Otherwise or in case of an error, false will be returned.

#### Examples

```sh
newer = is_path_newer ./new_file.txt ./old_file.txt
```


#### Aliases:
is_path_newer

<a name="std__fs__IsReadonly"></a>
## std::fs::IsReadonly
```sh
var = is_readonly path
```

This command will return true/false based if the provided path exists and is set to readonly.

#### Parameters

The path to check.

#### Return Value

True if the provided path exists and is set to readonly.

#### Examples

```sh
readonly = is_readonly ./dir/somefile.txt
```


#### Aliases:
is_readonly

<a name="std__fs__List"></a>
## std::fs::List
```sh
var = ls [flags] [path]
```

Lists the file/directory contents.<br>
If no path is provided, the current working directory will be used.<br>
The supported flags are:

* -l - Shows extended information

#### Parameters

* Optional flags (currently only -l is supported)
* Optional path (if not provided, current working directory is used)

#### Return Value

**true** is operation was successful.

#### Examples

```sh
# prints current directory content
ls

# prints current directory content
ls .

# prints examples directory content
ls ./examples

# prints examples directory content with extended info
ls -l ./examples

# prints current directory content with extended info
ls -l

# prints file name
ls ./examples/ls.ds

# prints file name with extended info
ls -l ./examples/ls.ds
```


#### Aliases:
ls

<a name="std__fs__MovePath"></a>
## std::fs::MovePath
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


#### Aliases:
mv

<a name="std__fs__Print"></a>
## std::fs::Print
```sh
var = cat [file]+
```

The cat command will print out the requested file/s.<br>
In addition it will also return the value to the output variable.

#### Parameters

Multiple file paths.

#### Return Value

The file content or none if the file does not exist.

#### Examples

```sh
cat ./docs/sdk.md
```


#### Aliases:
cat

<a name="std__fs__ReadBytes"></a>
## std::fs::ReadBytes
```sh
handle = read_binary_file file
```

Reads a raw file and returns a handle to the binary data.

#### Parameters

A single parameter holding the file path.

#### Return Value

The binary data handle.

#### Examples

```sh
handle = read_binary_file ./Cargo.toml
text = bytes_to_string ${handle}
```


#### Aliases:
readbinfile, read_binary_file

<a name="std__fs__ReadText"></a>
## std::fs::ReadText
```sh
var = readfile file
```

The readfile command will read the requested file and return the value to the output variable.

#### Parameters

A single parameter holding the file path.

#### Return Value

The file content or none in case file does not exist.

#### Examples

```sh
text = readfile ./Cargo.toml
```


#### Aliases:
readfile, read_text_file

<a name="std__fs__SetMode"></a>
## std::fs::SetMode
```sh
result = chmod mode path
```

This command will update the mode for the given path.<br>
**This command is currently only available for unix like systems and will return false for all others such as windows.**

#### Parameters

* The new mode, for example 755
* The path

#### Return Value

The new mode as decimal number or false in case of any error.

#### Examples

```sh
chmod 777 ./myfile.txt
```


#### Aliases:
chmod

<a name="std__fs__SetModeGlob"></a>
## std::fs::SetModeGlob

```sh
result = glob_chmod mode glob
```

This command will update the mode for the given glob pattern.<br>
**This command is currently only available for unix like systems and will return false for all others such as windows.**

#### Parameters

* The new mode, for example 755
* The path glob

#### Return Value

The amount of path entries affected by the operation or false in case of any error.

#### Examples

```sh
file1 = set ./target/_duckscript_test/glob_chmod/modify1.txt
touch ${file1}
file2 = set ./target/_duckscript_test/glob_chmod/modify2.txt
touch ${file2}

count = glob_chmod 777 ./target/_duckscript_test/glob_chmod/**/*.txt
assert_eq ${count} 2

readonly = is_readonly ${file1}
assert_false ${readonly}
readonly = is_readonly ${file2}
assert_false ${readonly}

count = glob_chmod 444 ./target/_duckscript_test/glob_chmod/**/*.txt
assert_eq ${count} 2

readonly = is_readonly ${file1}
assert ${readonly}
readonly = is_readonly ${file2}
assert ${readonly}
```


#### Source:

```sh

scope::glob_chmod::handle = glob_array ${scope::glob_chmod::argument::2}
scope::glob_chmod::output = array_length ${scope::glob_chmod::handle}

for scope::glob_chmod::entry in ${scope::glob_chmod::handle}
    scope::glob_chmod::result = chmod ${scope::glob_chmod::argument::1} ${scope::glob_chmod::entry}

    if equals ${scope::glob_chmod::result} false
        release ${scope::glob_chmod::handle}
        scope::glob_chmod::output = set false
    end
end

release ${scope::glob_chmod::handle}

set ${scope::glob_chmod::output}

```


#### Aliases:
glob_chmod

<a name="std__fs__TempDirectory"></a>
## std::fs::TempDirectory
```sh
path = temp_dir
```

This command will return the system temporary directory path.

#### Parameters

None

#### Return Value

The directory path.

#### Examples

```sh
path = temp_dir

echo ${path}
```


#### Aliases:
temp_dir

<a name="std__fs__TempFile"></a>
## std::fs::TempFile
```sh
path = temp_file [extension]
```

This command will create a new empty temporary file and return its path.

#### Parameters

Optional file extension.

#### Return Value

The file path.

#### Examples

```sh
path = temp_file toml

echo ${path}
```


#### Aliases:
temp_file

<a name="std__fs__WriteBytes"></a>
## std::fs::WriteBytes
```sh
result = write_binary_file file handle
```

This command enables to write binary data of the provided binary handle into the requested file.<br>
It will return true/false value based if it was able to write the binary data to the file.

#### Parameters

* The target file
* The binary data handle

#### Return Value

true/false based if it was able to write the binary data to the file.

#### Examples

```sh
handle = string_to_bytes "some text"
result = write_binary_file ./target/tests/data.bin ${handle}
```


#### Aliases:
writebinfile, write_binary_file

<a name="std__fs__WriteText"></a>
## std::fs::WriteText
```sh
result = writefile file text
```

This command enables to write the provided text into the requested file.<br>
It will return true/false value based if it was able to write the text to the file.

#### Parameters

* The target file
* The text content to write

#### Return Value

true/false based if it was able to write the text to the file.

#### Examples

```sh
result = writefile ./target/tests/writefile.txt "line 1\nline 2"
```


#### Aliases:
writefile, write_text_file

<a name="std__json"></a>
## std::json
The json module provides json parsing and encoding capabilities.<br>
When parsing a JSON string, the structure will be represented by simple variables.<br>
The root object (or simple value) will be set in the json_parse output variable and any sub structure will be
defined as variables with prefix of the root variable name.<br>
Object nodes, will have the value of: **[OBJECT]**.<br>
Array nodes will have a length variable defined, for example: **arr.length**<br>

Because duckscript variables have no type, the json_encode will define every boolean/numeric value as JSON string.<br>

Below is a simple example showing how to parse and encode values of all types.

```sh
fn test_simple_types
    str = json_parse \"myvalue\"
    assert_eq ${str} myvalue
    jsonstring = json_encode str
    assert_eq ${jsonstring} \"myvalue\"

    number = json_parse 500
    assert_eq ${number} 500
    jsonstring = json_encode number
    # numeric value is encoded as string
    assert_eq ${jsonstring} \"500\"

    bool = json_parse true
    assert_eq ${bool} true
    jsonstring = json_encode bool
    # boolean value is encoded to string
    assert_eq ${jsonstring} \"true\"

    arr = json_parse "[1, 2, 3]"
    # arr.length is not part of the JSON structure but added as a variable to enable
    # to loop over the array using the range command
    assert_eq ${arr.length} 3
    # direct array location access example
    assert_eq ${arr[0]} 1
    assert_eq ${arr[1]} 2
    assert_eq ${arr[2]} 3
    # array loop example
    arr_range = range 0 ${arr.length}
    for index in ${arr_range}
        expected_value = calc ${index} + 1
        value = get_by_name arr[${index}]
        assert_eq ${value} ${expected_value}
    end

    object = json_parse "{\"str\": \"my string value\", \"number\": 500, \"bool\": true, \"array\": [1, 2, 3]}"
    assert_eq ${object} [OBJECT]
    assert_eq ${object.str} "my string value"
    assert_eq ${object.number} 500
    assert_eq ${object.bool} true
    assert_eq ${object.array.length} 3
    assert_eq ${object.array[0]} 1
    assert_eq ${object.array[1]} 2
    assert_eq ${object.array[2]} 3
    jsonstring = json_encode object
    found = contains ${jsonstring} "\"str\":\"my string value\""
    assert ${found}
    found = contains ${jsonstring} "\"number\":\"500\""
    assert ${found}
    found = contains ${jsonstring} "\"bool\":\"true\""
    assert ${found}
    found = contains ${jsonstring} "\"array\":[\"1\",\"2\",\"3\"]"
    assert ${found}

    # we can cleanup all variables created from the json parse starting from the root object
    unset_all_vars --prefix object
    defined = is_defined object
    assert_false ${defined}
    defined = is_defined object.str
    assert_false ${defined}
    defined = is_defined object.array.length
    assert_false ${defined}
end
```



<a name="std__json__Encode"></a>
## std::json::Encode
```sh
string = json_encode var_name
```

This function will encode all variables, starting from the root variable as a JSON string.<br>
Since duckscript is untyped, all boolean and numeric values will be encoded as strings.

#### Parameters

The root variable name

#### Return Value

The JSON string

#### Examples

```sh
package = json_parse "{\"name\": \"my package\", \"version\": 1, \"publish\": false, \"keywords\": [\"test1\", \"test2\"], \"directories\": {\"test\": \"spec\"}}"
jsonstring = json_encode package
```


#### Aliases:
json_encode

<a name="std__json__Parse"></a>
## std::json::Parse
```sh
var = json_parse string
```

This function will parse the provided JSON string and will create variables based on the parsed data.<br>
The variables will reflect the json structure.<br>
Object keys will have name using the json path standard, for example root.child<br>
And arrays will have the array access annotation and length variable, for example:

```sh
root.child[5]
root.child.length
```

#### Parameters

The JSON string to parse.

#### Return Value

The root value.

#### Examples

```sh
package = json_parse "{\"name\": \"my package\", \"version\": 1, \"publish\": false, \"keywords\": [\"test1\", \"test2\"], \"directories\": {\"test\": \"spec\"}}"

assert_eq ${package} "[OBJECT]"
assert_eq ${package.name} "my package"
assert_eq ${package.version} 1
assert_eq ${package.publish} false
assert_eq ${package.keywords.length} 2
assert_eq ${package.keywords[0]} test1
assert_eq ${package.keywords[1]} test2
assert_eq ${package.directories.test} spec
```


#### Aliases:
json_parse

<a name="std__lib__alias__Set"></a>
## std::lib::alias::Set
```sh
var = alias command arguments
```

This command enables to define new commands with default arguments.<br>
The new alias can be invoked with additional arguments that will be appended to the default set.

#### Parameters

Any number of arguments which will be added to the already defined arguments set during the aliasing.

#### Return Value

**true** if the alias was created, else **false**.

#### Examples

```sh
# This example creates a new **my_echo** alias that will print the prefix before the requested arguments.
created = alias my_echo echo [ECHO]

# This will print "[ECHO] hello world "
created = my_echo hello world
```


#### Aliases:
alias

<a name="std__lib__alias__Unset"></a>
## std::lib::alias::Unset
```sh
unalias name
```

Removes previously defined alias and return true/false based if an alias was actually removed.

#### Parameters

The alias name to remove.

#### Return Value

A true/false value in case an alias with the provided name existed.

#### Examples

```sh
alias my_echo echo [ECHO]

# This will print "[ECHO] hello world "
my_echo hello world

unalias my_echo

# This will error
echo The script will now error as my_echo is no longer defined
my_echo hello world
```


#### Aliases:
unalias

<a name="std__lib__command__Remove"></a>
## std::lib::command::Remove
```sh
remove_command name
```

Removes a command and all its aliases.

#### Parameters

The command or alias name to remove.

#### Return Value

A true/false value in case a command was removed.

#### Examples

```sh
remove_command set
```


#### Aliases:
remove_command

<a name="std__math__Calc"></a>
## std::math::Calc
```sh
var = calc [operation]
```

The calc command accepts multiple arguments which make up a mathematical operation which will be
calculated and its result will be returned.

#### Parameters

Any number of arguments which will construct a line to calculate.

#### Return Value

The result of the mathematical calculation.

#### Examples

```sh
# result is 36
result = calc 1 + 5 * 7
```


#### Aliases:
calc

<a name="std__math__GreaterThan"></a>
## std::math::GreaterThan
```sh
var = greater_than left right
```

This command returns true/false based on left > right calculation.

#### Parameters

Two numeric values to compare.

#### Return Value

True if first argument is bigger than second argument.

#### Examples

```sh
result = greater_than 2 1.5
```


#### Aliases:
greater_than

<a name="std__math__LessThan"></a>
## std::math::LessThan
```sh
var = less_than left right
```

This command returns true/false based on left < right calculation.

#### Parameters

Two numeric values to compare.

#### Return Value

True if first argument is smaller than second argument.

#### Examples

```sh
result = less_than 1 1.5
```


#### Aliases:
less_than

<a name="std__net__Hostname"></a>
## std::net::Hostname
```sh
var = hostname
```

Returns the hostname.

#### Parameters

None

#### Return Value

The hostname

#### Examples

```sh
name = hostname
```


#### Aliases:
hostname

<a name="std__net__HttpClient"></a>
## std::net::HttpClient
```sh
var = http_client [--method method] [--payload payload] [--output-file file] URL
```

Invokes a HTTP request.<br>
The request method by default is GET but can be modified by the ```--method``` parameter.<br>
The ```--output-file``` parameter will redirect a valid response output to the provided file, otherwise all response text will be set to the
output variable.<br>
When redirecting to file, the output would be the response size.<br>
The ```--payload``` parameter enables to pass a payload to POST http requests.<br>
In case of errors or error HTTP response codes, false will be returned.

#### Parameters

* Optional HTTP Method, for example ```--method GET``` or ```--method POST``` (currently only GET and POST are supported).
* Optional post payload via ```--payload``` parameter.
* Optional redirection of output to file via ```--output-file``` parameter.
* The target URL

#### Return Value

The response text or in case of output redirection to file, the response size.<br>
In case of errors, it will return false.

#### Examples

```sh
function test_get
    response = http_client https://www.rust-lang.org/

    found = contains ${response} Rust

    assert ${found}
end

function test_get_to_file
    file = set ./target/_duckscript_test/http_client/page.html
    rm ${file}

    response_size = http_client --output-file ${file} https://www.rust-lang.org/

    response = readfile ${file}
    found = contains ${response} Rust

    assert ${found}
    assert ${response_size}
end

function test_post
    payload = set {\"login\":\"login\",\"password\":\"password\"}
    response = http_client --method POST --payload ${payload} https://reqbin.com/echo/post/json

    found = contains ${response} success

    assert ${found}
end
```


#### Aliases:
http_client

<a name="std__net__WGet"></a>
## std::net::WGet

```sh
var = wget [--method=HTTP-method] [--post-data=payload] [-O file] URL
```

Invokes a HTTP request.<br>
The request method by default is GET but can be modified by the ```--method``` parameter.<br>
The ```-O``` parameter will redirect a valid response output to the provided file, otherwise all response text will be set to the
output variable.<br>
When redirecting to file, the output would be the response size.<br>
The ```--post-data``` parameter enables to pass a payload to POST http requests.<br>
In case of errors or error HTTP response codes, false will be returned.

#### Parameters

* Optional HTTP Method, for example --method=HTTP-GET or --method=HTTP-POST (currently only GET and POST are supported).
* Optional post payload via ```--post-data``` parameter.
* Optional redirection of output to file via ```-O``` parameter.
* The target URL

#### Return Value

The response text or in case of output redirection to file, the response size.<br>
In case of errors, it will return false.

#### Examples

```sh
function test_get
    response = wget https://www.rust-lang.org/

    found = contains ${response} Rust

    assert ${found}
end

function test_get_to_file
    file = set ./target/_duckscript_test/wget/page.html
    rm ${file}

    response_size = wget -O ${file} https://www.rust-lang.org/

    response = readfile ${file}
    found = contains ${response} Rust

    assert ${found}
    assert ${response_size}
end

function test_post
    payload = set {\"login\":\"login\",\"password\":\"password\"}
    response = wget --method=HTTP-POST --post-data=${payload} https://reqbin.com/echo/post/json

    found = contains ${response} success

    assert ${found}
end
```


#### Source:

```sh

scope::wget::url = array_pop ${scope::wget::arguments}

scope::wget::method = set GET

scope::wget::lookingfor = set flag
for scope::wget::arg in ${scope::wget::arguments}
    if equals ${scope::wget::lookingfor} flag
        if starts_with ${scope::wget::arg} --method=HTTP-
            scope::wget::len = strlen --method=HTTP-
            scope::wget::method = substring ${scope::wget::arg} ${scope::wget::len}
        elif starts_with ${scope::wget::arg} --post-data=
            scope::wget::len = strlen --post-data=
            scope::wget::payload = substring ${scope::wget::arg} ${scope::wget::len}
        elif equals ${scope::wget::arg} -O
            scope::wget::lookingfor = set file
        end
    elif equals ${scope::wget::lookingfor} file
        scope::wget::file = set ${scope::wget::arg}
        scope::wget::lookingfor = set flag
    end
end

http_client --method "${scope::wget::method}" --output-file "${scope::wget::file}" --payload "${scope::wget::payload}" ${scope::wget::url}

```


#### Aliases:
wget

<a name="std__net__ftp__Get"></a>
## std::net::ftp::Get
```sh
result = ftp_get --host <hostname> [--port 21] [--username <user name>] [--password <password>] [--path <path>] [--type <A/I>] --remote-file <file name> --local-file <file name>
```

Invokes the FTP GET command from the given connection and file details.

#### Parameters

* --host - The host name or IP to connect to
* --port - Optional port number to use (by default 21)
* --username - Optional user name used to login (if not user or password provided, no login operation will be invoked)
* --password - Optional password used to login (if not user or password provided, no login operation will be invoked)
* --path - Optional path on the remote server to invoke operation on
* --type - Optional setting of the transfer type as A (ascii) I (image, binary)
* --remote-file - The remote file to download
* --local-file - The target local file name

#### Return Value

true if operation was completed.

#### Examples

```sh
ftp_get --host myhost --username someuser --password 12345 --remote-file README.md --local-file README.md
```


#### Aliases:
ftp_get

<a name="std__net__ftp__GetInMemory"></a>
## std::net::ftp::GetInMemory
```sh
handle = ftp_get_in_memory --host <hostname> [--port 21] [--username <user name>] [--password <password>] [--path <path>] [--type <A/I>] --remote-file <file name>
```

Invokes the FTP GET command from the given connection and file details.

#### Parameters

* --host - The host name or IP to connect to
* --port - Optional port number to use (by default 21)
* --username - Optional user name used to login (if not user or password provided, no login operation will be invoked)
* --password - Optional password used to login (if not user or password provided, no login operation will be invoked)
* --path - Optional path on the remote server to invoke operation on
* --type - Optional setting of the transfer type as A (ascii) I (image, binary)
* --remote-file - The remote file to download

#### Return Value

The binary data handle.

#### Examples

```sh
handle = ftp_get_in_memory --host myhost --username someuser --password 12345 --remote-file README.md
text = bytes_to_string ${handle}
```


#### Aliases:
ftp_get_in_memory

<a name="std__net__ftp__List"></a>
## std::net::ftp::List
```sh
handle = ftp_list --host <hostname> [--port 21] [--username <user name>] [--password <password>] [--path <path>]
```

Invokes the FTP LIST command from the given connection details and path.<br>
Returns a handle to an array of all response entries.

#### Parameters

* --host - The host name or IP to connect to
* --port - Optional port number to use (by default 21)
* --username - Optional user name used to login (if not user or password provided, no login operation will be invoked)
* --password - Optional password used to login (if not user or password provided, no login operation will be invoked)
* --path - Optional path on the remote server to invoke operation on

#### Return Value

A handle to an array holding all entries.

#### Examples

```sh
handle = ftp_list --host myhost --username someuser --password 12345

for entry in ${handle}
    echo ${entry}
end
```


#### Aliases:
ftp_list

<a name="std__net__ftp__NLst"></a>
## std::net::ftp::NLst
```sh
handle = ftp_nlst --host <hostname> [--port 21] [--username <user name>] [--password <password>] [--path <path>]
```

Invokes the FTP NLST command from the given connection details and path.<br>
Returns a handle to an array of all response entries.

#### Parameters

* --host - The host name or IP to connect to
* --port - Optional port number to use (by default 21)
* --username - Optional user name used to login (if not user or password provided, no login operation will be invoked)
* --password - Optional password used to login (if not user or password provided, no login operation will be invoked)
* --path - Optional path on the remote server to invoke operation on

#### Return Value

A handle to an array holding all entries.

#### Examples

```sh
handle = ftp_nlst --host myhost --username someuser --password 12345

for entry in ${handle}
    echo ${entry}
end
```


#### Aliases:
ftp_nlst

<a name="std__net__ftp__Put"></a>
## std::net::ftp::Put
```sh
result = ftp_put --host <hostname> [--port 21] [--username <user name>] [--password <password>] [--path <path>] [--type <A/I>] --remote-file <file name> --local-file <file name>
```

Invokes the FTP PUT command from the given connection and file details.

#### Parameters

* --host - The host name or IP to connect to
* --port - Optional port number to use (by default 21)
* --username - Optional user name used to login (if not user or password provided, no login operation will be invoked)
* --password - Optional password used to login (if not user or password provided, no login operation will be invoked)
* --path - Optional path on the remote server to invoke operation on
* --type - Optional setting of the transfer type as A (ascii) I (image, binary)
* --remote-file - The remote file to upload
* --local-file - The source local file to upload

#### Return Value

true if operation was completed.

#### Examples

```sh
ftp_put --host myhost --username someuser --password 12345 --remote-file README.md --local-file README.md
```


#### Aliases:
ftp_put

<a name="std__net__ftp__PutInMemory"></a>
## std::net::ftp::PutInMemory
```sh
result = ftp_put_in_memory --host <hostname> [--port 21] [--username <user name>] [--password <password>] [--path <path>] [--type <A/I>] --remote-file <file name> --content <content>
```

Invokes the FTP PUT command from the given connection and file details.

#### Parameters

* --host - The host name or IP to connect to
* --port - Optional port number to use (by default 21)
* --username - Optional user name used to login (if not user or password provided, no login operation will be invoked)
* --password - Optional password used to login (if not user or password provided, no login operation will be invoked)
* --path - Optional path on the remote server to invoke operation on
* --type - Optional setting of the transfer type as A (ascii) I (image, binary)
* --remote-file - The remote file to upload
* --content - The textual content to upload

#### Return Value

true if operation was completed.

#### Examples

```sh
ftp_put_in_memory --host myhost --username someuser --password 12345 --remote-file README.md --content "This is the README content"
```


#### Aliases:
ftp_put_in_memory

<a name="std__process__Execute"></a>
## std::process::Execute
```sh
exec [--fail-on-error] command [args]*

output = exec command [args]*
stdout = set ${output.stdout}
stderr = set ${output.stderr}
exit_code = set ${output.code}

exit_code = exec --get-exit-code command [args]*
```

Executes the provided native command and arguments.<br>
If no output variable is set, the command output will be flushed to the main out/err stream of the parent process.<br>
In addition, in order to fail the command in case of the child process failed, add the --fail-on-error flag.<br>
If an output variable is set, it will be used as a base variable name from which the command stout, stderr and exit code information can be pulled from.<br>
The actual output variable name will not be modified, instead new variables will be created using that variable name as a baseline:

* *output*.stdout - Will hold the stdout text content.
* *output*.stderr - Will hold the stderr text content.
* *output*.code - Will hold the process exit code.

If an output variable is set and the --get-exit-code flag is provided, the output will only contain the exit code.

#### Parameters

* --fail-on-error - If no output variable is provided, it will cause an error in case the executed process exits with an error exit code.
* --get-exit-code - If an output variable is provided, it will contain the exit code.
* The command to execute and its arguments.

#### Return Value

Optionally a base name to access the process stout, stderr and exit code information.

#### Examples

```sh
# Example of running a command and flushing its output to the parent process.
exec echo hello world

# Example of running a command and storing its output.
output = exec echo hello world

stdout = set ${output.stdout}
stderr = set ${output.stderr}
exit_code = set ${output.code}

echo stdout: ${stdout}
echo stderr: ${stderr}
echo exit code: ${exit_code}
```


#### Aliases:
exec

<a name="std__process__Exit"></a>
## std::process::Exit
```sh
code = exit [code]
```

Exits the script with the given code stored in the output variable.

#### Parameters

A positive number as exit code or none for 0.

#### Return Value

The exit code.

#### Examples

```sh
# exit with code '0'
code = exit

# exit with code '1'
code = exit 1
```


#### Aliases:
exit, quit, q

<a name="std__process__ProcessID"></a>
## std::process::ProcessID
```sh
var = pid
```

Returns the current process ID.

#### Parameters

None

#### Return Value

The current process ID.

#### Examples

```sh
id = pid
```


#### Aliases:
pid, process_id

<a name="std__process__Spawn"></a>
## std::process::Spawn
```sh
pid = spawn command [args]*
```

Executes the provided native command and arguments.<br>
It will not wait for the process to finish and will return the process pid.

#### Parameters

The command to execute and its arguments.

#### Return Value

The process pid.

#### Examples

```sh
pid = spawn echo test

echo PID: ${pid}
```


#### Aliases:
spawn

<a name="std__process__Watchdog"></a>
## std::process::Watchdog
```sh
count = watchdog [--max-retries value] [--interval value] -- command [arguments]*
```

Executes the provided native command and arguments.<br>
In case the command exited it will be executed again up to the max retries provided.<br>
The watchdog will wait the specified interval in milliseconds between invocations.<br>
In case of an invalid command, the watchdog will not reattempt the invocation and will exit without retries.

#### Parameters

* --max-retries - Positive value of max retries (excluding the first invocation). value <= 0 for unlimited retries. Default is unlimited.
* --interval - The amount in milliseconds between retries. 0 for no waiting between invocations. Default is no wait.
* The command to execute (preceded by a **--** separator).
* The command arguments.

#### Return Value

The amount of invocations or false in case of any error.

#### Examples

```sh
count = watchdog --max-retries 0 -- echo test
assert_eq ${count} 1

count = watchdog --max-retries 3 --interval 10 -- echo test
assert_eq ${count} 4
```


#### Aliases:
watchdog

<a name="std__random__Range"></a>
## std::random::Range
```sh
output = random_range min max
```

Generate a random value in the range of min and max values provided, i.e. inclusive of min and exclusive of max.

#### Parameters

* min - The min range value (inclusive)
* max - The max range value (exclusive)

#### Return Value

The generated numeric value.

#### Examples

```sh
value = random_range -10 10
echo ${value}
```


#### Aliases:
random_range, rand_range

<a name="std__random__Text"></a>
## std::random::Text
```sh
output = random_text [length]
```

Generates random alphanumeric text with the requested length (length is 1 if not provided).

#### Parameters

Optional text length. Length is defaulted to 1 if not provided.

#### Return Value

The generated alphanumeric value.

#### Examples

```sh
value = random_text 50
echo ${value}
```


#### Aliases:
random_text, rand_text

<a name="std__scope__Clear"></a>
## std::scope::Clear
```sh
clear_scope name
```

Clears all variables which are prefixed with the provided name + ::.<br>
For example, if the value provided is **my_scope** all variables that start with **my_scope::** will be removed.

#### Parameters

The scope name.

#### Return Value

None.

#### Examples

```sh
testscope = set true
testscope::1 = set 1
testscope::subscope::1 = set 1

assert_eq ${testscope} true
defined = is_defined testscope::1
assert ${defined}
assert_eq ${testscope::1} 1
defined = is_defined testscope::subscope::1
assert ${defined}
assert_eq ${testscope::subscope::1} 1

clear_scope testscope

assert_eq ${testscope} true

defined = is_defined testscope::1
assert_false ${defined}
defined = is_defined testscope::subscope::1
assert_false ${defined}
```


#### Aliases:
clear_scope

<a name="std__scope__PopStack"></a>
## std::scope::PopStack
```sh
scope_pop_stack [--copy name1 name2 ...]
```

Removes all known variables except for the variables provided by the optional --copy argument and than restores the
previously pushed stack.<br>
Functions with the **<scope>** annotation will automatically invoke this command when they end or return a value.

#### Parameters

Optional variable names to keep.

#### Return Value

None.

#### Examples

```sh
var1 = set 1
var2 = set 2

scope_push_stack --copy var2

defined = is_defined var1
echo ${defined}
defined = is_defined var2
echo ${defined}

var3 = set 3
var4 = set 4

scope_pop_stack --copy var4

defined = is_defined var1
echo ${defined}
defined = is_defined var2
echo ${defined}
defined = is_defined var3
echo ${defined}
defined = is_defined var4
echo ${defined}
```


#### Aliases:
scope_pop_stack

<a name="std__scope__PushStack"></a>
## std::scope::PushStack
```sh
scope_push_stack [--copy name1 name2 ...]
```

Removes all known variables except for the variables provided by the optional --copy argument.<br>
Functions with the **<scope>** annotation will automatically invoke this command and keep only the relevant
function arguments in the new scope.

#### Parameters

Optional variable names to keep.

#### Return Value

None.

#### Examples

```sh
var1 = set 1
var2 = set 2

scope_push_stack --copy var2

defined = is_defined var1
echo ${defined}
defined = is_defined var2
echo ${defined}
```


#### Aliases:
scope_push_stack

<a name="std__semver__IsEqual"></a>
## std::semver::IsEqual
```sh
output = semver_is_equal value1 value2
```

Returns true if both semver values are valid and equal.

#### Parameters

Two semver values to compare.

#### Return Value

True if both semver values are valid and equal, else false.

#### Examples

```sh
equal = semver_is_equal 1.2.3 1.2.3
assert ${equal}

equal = semver_is_equal 1.2.3 2.2.3
assert_false ${equal}
```


#### Aliases:
semver_is_equal

<a name="std__semver__IsNewer"></a>
## std::semver::IsNewer
```sh
output = semver_is_newer newer older
```

Returns true if both semver values are valid and first value is newer.

#### Parameters

* The expected newer value
* The expected older value

#### Return Value

True if both semver values are valid and first value is newer, else false.

#### Examples

```sh
newer = semver_is_newer 3.2.3 2.2.3
assert ${newer}

newer = semver_is_newer 1.2.3 2.2.3
assert_false ${newer}

newer = semver_is_newer 1.2.3 1.2.3
assert_false ${newer}
```


#### Aliases:
semver_is_newer

<a name="std__semver__Parse"></a>
## std::semver::Parse
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


#### Aliases:
semver_parse

<a name="std__string__Base64"></a>
## std::string::Base64

```sh
var = base64 [-e] [-encode] [-d] [-decode] value
```

Invokes the base64 encode/decode command with the provided value.<br>
This command allows for a more similar cli command which wraps the base64_encode and base64_decode commands.

#### Parameters

* Optional -e or -encode flags to set the mode to encode (default)
* Optional -d or -decode flags to set the mode to decode
* The value, in case of encoding this is the binary handle, in case of decoding this is the base64 textual value.

#### Return Value

* In case of encoding, the base64 textual value will be returned.
* In case of decoding, a handle to the binary data will be returned.

#### Examples

```sh
handle = string_to_bytes "hello world"
text = base64 ${handle}
release ${handle}
assert_eq ${text} aGVsbG8gd29ybGQ=

handle = base64 -decode ${text}
text = bytes_to_string ${handle}
release ${handle}
assert_eq ${text} "hello world"
```


#### Source:

```sh

scope::base64::input_data = array_pop ${scope::base64::arguments}
scope::base64::encode = set true

for scope::base64::arg in ${scope::base64::arguments}
    if equals ${scope::base64::arg} -e
         scope::base64::encode = set true
    elif equals ${scope::base64::arg} -encode
         scope::base64::encode = set true
    elif equals ${scope::base64::arg} -d
         scope::base64::encode = set false
    elif equals ${scope::base64::arg} -decode
         scope::base64::encode = set false
    end
end

if ${scope::base64::encode}
    scope::base64::output = base64_encode ${scope::base64::input_data}
else
    scope::base64::output = base64_decode ${scope::base64::input_data}
end

scope::base64::output = set ${scope::base64::output}

```


#### Aliases:
base64

<a name="std__string__Base64Decode"></a>
## std::string::Base64Decode
```sh
text = base64_encode handle
```

Encodes using base64 the provided binary data and returns the encoded text value.<br>
The binary data is provided as a handle.

#### Parameters

The handle to the binary data to encode.

#### Return Value

The encoded textual value.

#### Examples

```sh
handle = string_to_bytes "hello world"
text = base64_encode ${handle}

release ${handle}

assert_eq ${text} "hello world"
```


#### Aliases:
base64_decode

<a name="std__string__Base64Encode"></a>
## std::string::Base64Encode
```sh
text = base64_encode handle
```

Encodes using base64 the provided binary data and returns the encoded text value.<br>
The binary data is provided as a handle.

#### Parameters

The handle to the binary data to encode.

#### Return Value

The encoded textual value.

#### Examples

```sh
handle = string_to_bytes "hello world"
text = base64_encode ${handle}

release ${handle}

assert_eq ${text} "hello world"
```


#### Aliases:
base64_encode

<a name="std__string__BytesToString"></a>
## std::string::BytesToString
```sh
text = bytes_to_string handle
```

Converts the provided UTF-8 binary array to string and returns it.

#### Parameters

A handle to a binary array holding UTF-8 text.

#### Return Value

The textual data.

#### Examples

```sh
handle = string_to_bytes "hello world"
text = bytes_to_string ${handle}

release ${handle}

assert_eq ${text} "hello world"
```


#### Aliases:
bytes_to_string

<a name="std__string__Concat"></a>
## std::string::Concat

```sh
var = concat [value]*
```

Concats the provided input into a single string and returns it.

#### Parameters

Any number of values to concat.

#### Return Value

The result of the concatenation of all input values.

#### Examples

```sh
output = concat 1 2 3 4
assert_eq ${output} 1234

output = concat 1 "2 3" 4
assert_eq ${output} "12 34"
```


#### Source:

```sh

scope::concat::output = set ""
for scope::concat::arg in ${scope::concat::arguments}
    scope::concat::output = set "${scope::concat::output}${scope::concat::arg}"
end

set ${scope::concat::output}

```


#### Aliases:
concat

<a name="std__string__Contains"></a>
## std::string::Contains
```sh
var = contains all partial
```

Returns true if the first argument contains the value of the second argument.

#### Parameters

* The full text to search in
* The text to search for

#### Return Value

**true** if contains.

#### Examples

```sh
# valid conditions
result = contains abcd bc

value = set "some text"
result = contains ${value} "me tex"

# will return false
result = contains abcd b1c
```


#### Aliases:
contains

<a name="std__string__EndsWith"></a>
## std::string::EndsWith
```sh
var = ends_with all partial
```

Returns true if the first argument ends with the value of the second argument.

#### Parameters

* The full text to search in
* The suffix text to search for

#### Return Value

**true** if ends with.

#### Examples

```sh
# valid conditions
result = ends_with abcd abc

value = set "some text"
result = ends_with ${value} "me text"

# will return false
result = ends_with abcd abc
```


#### Aliases:
ends_with

<a name="std__string__Equals"></a>
## std::string::Equals
```sh
var = eq value1 value2
```

Returns true if both provided values are equal.

#### Parameters

Two values to evaluate if they are equal

#### Return Value

**true** if equal.

#### Examples

```sh
# valid conditions
is_same = eq yes yes
is_same = eq false false

value = set "some text"
is_same = eq ${value} "some text"

# will return false
is_same = eq 1 2
```


#### Aliases:
equals, eq

<a name="std__string__IndexOf"></a>
## std::string::IndexOf
```sh
var = indexof full_text text_to_find
```

This command will attempt to find the text from the second argument inside the text in the first argument.<br>
If found, an index value will be returned, otherwise none is returned.

#### Parameters

* The text to search in
* The text to find

#### Return Value

The index of the text found or none if not found.

#### Examples

```sh
index = indexof "    some  text   " some 
```


#### Aliases:
indexof

<a name="std__string__IsEmpty"></a>
## std::string::IsEmpty
```sh
var = is_empty value
```

Returns true if the provided value is none or an empty string.

#### Parameters

The value to validate.

#### Return Value

True if the provided value is none or an empty string.

#### Examples

```sh
value = set "hello world"
empty = is_empty ${value}
```


#### Aliases:
is_empty

<a name="std__string__LastIndexOf"></a>
## std::string::LastIndexOf
```sh
var = last_indexof full_text text_to_find
```

This command will attempt to find the text from the second argument inside the text in the first argument.<br>
If found, an index value will be returned, otherwise none is returned.<br>
Unlike the **indexof** command, this command will search for text starting at the end, going backwards.

#### Parameters

* The text to search in
* The text to find

#### Return Value

The index of the text found or none if not found.

#### Examples

```sh
index = last_indexof "    some  text   " some
```


#### Aliases:
last_indexof

<a name="std__string__Length"></a>
## std::string::Length
```sh
var = length text
```

Returns the text length.

#### Parameters

The text to extract the length from.

#### Return Value

The text length value.

#### Examples

```sh
len = length "Hello World"
```


#### Aliases:
length, strlen

<a name="std__string__Replace"></a>
## std::string::Replace
```sh
var = replace text from to
```

Returns new value of text after replacing all from values to the provided to values.

#### Parameters

* The full text
* The from text
* The to text

#### Return Value

The updated text.

#### Examples

```sh
text = set "my large text value with lots of text"
updated = replace ${text} text stuff

assert_eq ${updated} "my large stuff value with lots of stuff"
```


#### Aliases:
replace

<a name="std__string__Split"></a>
## std::string::Split
```sh
handle = split text pattern
```

Splits the provided text based on the provided pattern and return a handle the
created array with all the splitted values.

#### Parameters

* The text to split
* The pattern to split by

#### Return Value

A handle to the values array.

#### Examples

```sh
handle = split a23b23c23d23e 23

len = array_length ${handle}

value = array_pop ${handle}
assert_eq ${value} e
value = array_pop ${handle}
assert_eq ${value} d
value = array_pop ${handle}
assert_eq ${value} c
value = array_pop ${handle}
assert_eq ${value} b
value = array_pop ${handle}
assert_eq ${value} a

release ${handle}

assert_eq ${len} 5
```


#### Aliases:
split

<a name="std__string__StartsWith"></a>
## std::string::StartsWith
```sh
var = starts_with all partial
```

Returns true if the first argument starts with the value of the second argument.

#### Parameters

* The full text to search in
* The prefix text to search for

#### Return Value

**true** if starts with.

#### Examples

```sh
# valid conditions
result = starts_with abcd abc

value = set "some text"
result = starts_with ${value} "some te"

# will return false
result = starts_with abcd bcd
```


#### Aliases:
starts_with

<a name="std__string__StringToBytes"></a>
## std::string::StringToBytes
```sh
handle = string_to_bytes text
```

Converts the provided string into binary format and returns a handle to the binary data.

#### Parameters

The text to convert.

#### Return Value

A handle to the binary data.

#### Examples

```sh
handle = string_to_bytes "hello world"
text = bytes_to_string ${handle}

release ${handle}

assert_eq ${text} "hello world"
```


#### Aliases:
string_to_bytes

<a name="std__string__SubString"></a>
## std::string::SubString
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


#### Aliases:
substring

<a name="std__string__Trim"></a>
## std::string::Trim
```sh
var = trim value
```

Returns the provided value with leading and trailing whitespace removed.

#### Parameters

The value to trim.

#### Return Value

The trimmed value. If no input provided, this command will return none.

#### Examples

```sh
# trimmed will now hold "some  text"
trimmed = trim "  some  text   "
```


#### Aliases:
trim

<a name="std__string__TrimEnd"></a>
## std::string::TrimEnd
```sh
var = trim_end value
```

Returns the provided value with trailing whitespace removed.

#### Parameters

The value to trim.

#### Return Value

The trimmed value. If no input provided, this command will return none.

#### Examples

```sh
# trimmed will now hold "  some  text"
trimmed = trim_end "  some  text   "
```


#### Aliases:
trim_end

<a name="std__string__TrimStart"></a>
## std::string::TrimStart
```sh
var = trim_start value
```

Returns the provided value with leading whitespace removed.

#### Parameters

The value to trim.

#### Return Value

The trimmed value. If no input provided, this command will return none.

#### Examples

```sh
# trimmed will now hold "some  text   "
trimmed = trim_start "  some  text   "
```


#### Aliases:
trim_start

<a name="std__test__Assert"></a>
## std::test::Assert
```sh
assert value [error message]
```

Used to validate the input is truthy.<br>
If the value is one of the following:

* No output
* false (case insensitive)
* 0
* no (case insensitive)
* Empty value

It is considered falsy and will exist with an error.

#### Parameters

* The value to evaluate
* Optional error message

#### Return Value

**true** if truthy.

#### Examples

```sh
# valid conditions
assert ok
assert true
assert yes

value = set "some text"
assert ${value}

# error conditions (each one will break the execution)
assert
assert false
assert 0
assert false "This is my error message"
```


#### Aliases:
assert

<a name="std__test__AssertEquals"></a>
## std::test::AssertEquals
```sh
assert_eq value1 value2 [error message]
```

Used to validate the input is the same.<br>
If they are not, the command will exist with an error.

#### Parameters

* Two values to evaluate if they are equal
* Optional error message

#### Return Value

**true** if equal.

#### Examples

```sh
# valid conditions
assert_eq yes yes
assert_eq false false

value = set "some text"
assert_eq ${value} "some text"

# error conditions (each one will break the execution)
assert_eq 1 2
assert_eq 1 2 "This is my error message"
```


#### Aliases:
assert_eq

<a name="std__test__AssertError"></a>
## std::test::AssertError
```sh
assert_error [error message]
```

This command will cause a runtime error which will not stop the script execution.<br>
If error message is provided, it will be used as part of the error output.

#### Parameters

Optional error message.

#### Return Value

None

#### Examples

```sh
assert_error

assert_error "This is my error message"
```


#### Aliases:
assert_error

<a name="std__test__AssertFail"></a>
## std::test::AssertFail
```sh
assert_fail [error message]
```

This command will exist with an error.<br>
If error message is provided, it will be used as part of the error output.

#### Parameters

Optional error message.

#### Return Value

None

#### Examples

```sh
assert_fail

assert_fail "This is my error message"
```


#### Aliases:
assert_fail

<a name="std__test__AssertFalse"></a>
## std::test::AssertFalse
```sh
assert_false value [error message]
```

Used to validate the input is falsy.<br>
If the value is one of the following:

* No output
* false (case insensitive)
* 0
* no (case insensitive)
* Empty value

It is considered falsy.

#### Parameters

* The value to evaluate
* Optional error message

#### Return Value

**true** if falsy.

#### Examples

```sh
# valid conditions
assert_false
assert_false false
assert_false 0
assert_false false "This is my error message"

# error conditions (each one will break the execution)
assert_false ok
assert_false true
assert_false yes

value = set "some text"
assert_false ${value}
```


#### Aliases:
assert_false

<a name="std__test__TestDirectory"></a>
## std::test::TestDirectory
```sh
test_directory directory [pattern]
```

This command can be used to run unit tests written in duckscript.<br>
It will run all duckscript files in the directory tree ending with **test.ds** and for each file, it will run
all functions that start with **test_**.<br>
Each such function is considered as a test and can run any type of code and check itself using assert commands.

#### Parameters

* The root directory of all test files (all files ending with **test.ds** in the directory tree will be checked)
* Optional pattern for the file name or test function to limit invocation of only those tests.

#### Return Value

**true** if successful.

#### Examples

This is an example of a test function:

```sh
function test_set_get_unset
    unset_env TEST_SET_GET_UNSET
    value = get_env TEST_SET_GET_UNSET
    assert_false ${value}

    value = set_env TEST_SET_GET_UNSET "test value"
    assert ${value}
    value = get_env TEST_SET_GET_UNSET
    assert_eq ${value} "test value"
end
```


#### Aliases:
test_directory

<a name="std__test__TestFile"></a>
## std::test::TestFile
```sh
test_file file [test name]
```

This command can be used to run unit tests written in duckscript.<br>
It will run all test functions that start with **test_** in the given file.<br>
Each such function is considered as a test and can run any type of code and check itself using assert commands.

#### Parameters

* The file name containing the test functions.
* Optional pattern for the test function to limit invocation of only those tests.

#### Return Value

**true** if successful.

#### Examples

This is an example of a test function:

```sh
function test_set_get_unset
    unset_env TEST_SET_GET_UNSET
    value = get_env TEST_SET_GET_UNSET
    assert_false ${value}

    value = set_env TEST_SET_GET_UNSET "test value"
    assert ${value}
    value = get_env TEST_SET_GET_UNSET
    assert_eq ${value} "test value"
end
```


#### Aliases:
test_file

<a name="std__thread__Sleep"></a>
## std::thread::Sleep
```sh
sleep millies
```

Will cause the script execution to half for the given amount of milliseconds.<br>
The command will also return the amount of milliseconds waited.

#### Parameters

A positive numeric value.

#### Return Value

The amount of milliseconds waited.

#### Examples

```sh
# will sleep for 10 milliseconds
time = sleep 10
echo Waited for ${time} milliseconds.
```


#### Aliases:
sleep

<a name="std__time__CurrentTimeMillies"></a>
## std::time::CurrentTimeMillies
```sh
var = current_time
```

Returns the current time in milliseconds (from January 1, 1970 UTC).

#### Parameters

None

#### Return Value

The current time in milliseconds.

#### Examples

```sh
result = current_time
echo ${result}
```


#### Aliases:
current_time

<a name="std__var__GetAllVarNames"></a>
## std::var::GetAllVarNames
```sh
handle = get_all_var_names
```

Creates an array holding all currently known variable names and returns the array handle.

#### Parameters

None

#### Return Value

A handle to the array.

#### Examples

```sh
handle = get_all_var_names

# once done we should release the handle
release ${handle}
```


#### Aliases:
get_all_var_names

<a name="std__var__GetByName"></a>
## std::var::GetByName
```sh
var = get_by_name name
```

This command returns the variable value based on the given variable name.<br>
It is similar to
```sh
var = set ${name}
```
However, it allows for a dynamic variable name.

#### Parameters

The variable name.

#### Return Value

The variable value or none if no such variable exists.

#### Examples

```sh
var = set test
value = get_by_name var
defined = is_defined value

assert ${defined}
assert_eq ${value} test
```


#### Aliases:
get_by_name

<a name="std__var__Set"></a>
## std::var::Set
```sh
var = set arg [or arg]*
```

The set command will simply return the provided argument and set it to the output variable.<br>
In case the argument is falsy it will attempt to provide another value if an 'or' keyword is set.

A value is considered falsy if it is one of the following:

* false (case insensitive)
* 0
* no (case insensitive)
* Empty value

#### Parameters

The argument to set or an 'or' conditional arguments.

#### Return Value

The first truthy value

#### Examples

```sh
# Return simple 'hello' text value
var = set hello

# Return expanded value: 'home: ....'
var = set "home: ${HOME}"

value = set test or false
assert_eq ${value} test

value = set 0 or no or false or NO or FALSE
assert_eq ${value} FALSE
```


#### Aliases:
set

<a name="std__var__SetByName"></a>
## std::var::SetByName
```sh
var = set_by_name name [value]
```

This command sets the variable value based on the variable name.<br>
It is similar to
```sh
name = set ${value}
```
However, it allows for a dynamic variable name.

#### Parameters

* The variable name.
* The new variable value, if not provided, the variable will be unset.

#### Return Value

The new variable value.

#### Examples

```sh
var = set test
value = get_by_name var
defined = is_defined value

assert ${defined}
assert_eq ${value} test
```


#### Aliases:
set_by_name

<a name="std__var__Unset"></a>
## std::var::Unset

```sh
unset [names]*
```

Undefines all the variable names provided.

#### Parameters

A list of variable names to undefine.

#### Return Value

None

#### Examples

```sh
var = set 1
defined = is_defined var
assert ${defined}
unset var
defined = is_defined var
assert_false ${defined}
```


#### Source:

```sh

for scope::unset::name in ${scope::unset::arguments}
    set_by_name ${scope::unset::name}
end

```


#### Aliases:
unset

<a name="std__var__UnsetAllVars"></a>
## std::var::UnsetAllVars
```sh
handle = unset_all_vars [--prefix value]
```

Removes all known variables.<br>
If the prefix is provided, only variables starting with the prefix value will be removed.

#### Parameters

* Optional variable name prefix

#### Return Value

None

#### Examples

```sh
fn test_remove_all
    a = set 1
    b = set 2

    defined = is_defined a
    assert ${defined}
    defined = is_defined b
    assert ${defined}

    unset_all_vars

    defined = is_defined a
    assert_false ${defined}
    defined = is_defined b
    assert_false ${defined}
end

fn test_remove_by_prefix
    root1 = set true
    root1.child = set true
    root12 = set true

    root2 = set true

    defined = is_defined root1
    assert ${defined}
    defined = is_defined root1.child
    assert ${defined}
    defined = is_defined root12
    assert ${defined}
    defined = is_defined root2
    assert ${defined}

    unset_all_vars --prefix root1

    defined = is_defined root1
    assert_false ${defined}
    defined = is_defined root1.child
    assert_false ${defined}
    defined = is_defined root12
    assert_false ${defined}
    defined = is_defined root2
    assert ${defined}
end
```


#### Aliases:
unset_all_vars

### License
Developed by Sagie Gur-Ari and licensed under the
[Apache 2](https://github.com/sagiegurari/duckscript/blob/master/LICENSE) open source license.
