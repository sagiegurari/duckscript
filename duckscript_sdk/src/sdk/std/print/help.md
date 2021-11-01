```sh
print [--style|-s bold|underline|italic|dimmed|blink|strikethrough]* [--color|-c black|red|green|yellow|blue|magenta|cyan|white|bright_<color>|rgb_<red>_<green>_<blue>] [--background-color|-bgc black|red|green|yellow|blue|magenta|cyan|white|bright_<color>|rgb_<red>_<green>_<blue>] [arg]*
```

The print command will printout all provided arguments with optional color values.<br>
No end of line will be added.<br>
The style will continue to additional printouts until an echo/println is used to close the line.<br>
**Not all colors and all styles are supported on every terminal.**

### Parameters

* Optional styles - To support multiple styles, add the option as much as needed.
* Optional color - The text color. For RGB, use the rgb_ prefix with the values separated by a _ character.
* Optional background color (also supports rgb_ prefix)
* Any number of arguments may be provided and will be printed.

### Return Value

The amount of arguments printed.

### Examples

```sh
# Print multiple arguments:
print hello world

# Print multiple spaces between words
print "hello    world"

# Print with style/color values
print --style underline --color red My Bold Red Text
echo
print -s underline -s bold -c bright_green -bgc red Hellow World
echo
```
