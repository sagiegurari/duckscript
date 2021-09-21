```sh
var = last_indexof full_text text_to_find
```

This command will attempt to find the text from the second argument inside the text in the first argument.<br>
If found, an index value will be returned, otherwise none is returned.<br>
Unlike the **indexof** command, this command will search for text starting at the end, going backwards.

### Parameters

* The text to search in
* The text to find

### Return Value

The index of the text found or none if not found.

### Examples

```sh
index = last_indexof "    some  text   " some
```
