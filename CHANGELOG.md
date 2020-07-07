## CHANGELOG

### v0.6.1

* New get_last_modified_time command.

### v0.6.0 (2020-07-03)

* New ftp_put command.
* New fpt_put_in_memory command.
* New ftp_get command.
* New ftp_get_in_memory command.
* New ftp_list command.
* New ftp_nlst command.
* New noop command.
* \[Breaking Change\] Conditions to support function calls #116

### v0.5.0 (2020-06-06)

* Fixed end command bug which directed execution to wrong sub command when multiple script contexts are running #110
* Runtime - Fixed spread expansion with control characters #109

### v0.4.2 (2020-06-05)

* Fixed parsing issue for commands evaluated by other commands (for example if conditions)

### v0.4.1 (2020-06-04)

* New Enable to error an exec command via new --fail-on-error flag #108

### v0.4.0 (2020-05-07)

* New set_from_array command.
* New set_to_array command.
* New set_contains command.
* New set_remove command.
* New set_size command.
* New set_is_empty command.
* New set_put command.
* New set_clear command.
* Add array_size alias to array_length command.
* New array_remove command.
* New set_new command.
* New is_set command.
* New unset command.
* New array_clear command.
* New array_contains command.
* New map_contains_value command.
* New map_contains_key command.
* New get_all_var_names command #100
* New get_by_name command.
* New set_by_name command.
* Add array_add and array_put aliases to array_push
* Runtime - Support for hashset state value.
* \[Breaking Change\] Runtime - REPL mode doesn't stop due to crashes from user commands #103

### v0.3.3 (2020-04-15)

* New which command.
* New cpu_count command.
* New printenv command #97
* New env_to_map command #96
* New map_keys command.
* New temp_dir command.
* Runtime - Use default trait.

### v0.3.2 (2020-04-04)

* New array_set command.
* New array_get command #94

### v0.3.1 (2020-03-28)

* Bump cli version.

### v0.3.0 (2020-03-13)

* New remove_command command #91
* \[Breaking Change\] uname is now a new command and does not alias os_family command.
* New os_version command.
* New os_release command.
* New os_name command.
* New is_windows command.
* New glob_chmod command.
* New glob_array command #90
* New chmod command #19
* New is_readonly command.
* New is_path_exists command.
* New is_directory command.
* New is_file command.
* Runtime - Fix value expansion for control characters

### v0.2.1 (2020-02-21)

* New temp_file command #85
* New spawn command #87
* Update build makefile.
* Runtime - Add support for **Any** state type #86
* SDK - Use fsio crate for file system apis.
* Runtime - Use fsio crate for file system apis.

### v0.2.0 (2020-02-06)

* The if/else and not commands now support complex conditions #81
* Release command now support recursive option.
* New map_clear command.
* New map_to_properties command.
* New map_load_properties command.
* New map_is_empty command.
* New map_size command.
* New map_remove command.
* New is_map command.
* New map_get command.
* New map_put command.
* New map command.
* The set command now supports 'or' condition.
* New base64 command #79
* New write_binary_file command #78
* New read_binary_file command #78
* Rename read/write text file commands.
* New base64_decode command #75
* New base64_encode command #75
* New bytes_to_string command.
* New string_to_bytes command.
* Add prefix flag to write_properties command #77
* New split command #76
* New appendfile command.
* New watchdog command.
* New pid command #73
* New whoami command.
* Alias based command implementations are not checked for variable leaks.
* New get_home_dir command.
* New array_join command.
* The read_properties command now support **--prefix** flag.
* New array_concat command.
* New trigger_error command.
* New array_push command.
* New concat command.
* Improve wget input parsing.
* Modify full named commands.
* Prevent panic for wget and http_client on invalid non http URL input.
* Runtime - Supports byte array state storage

### v0.1.8 (2020-01-24)

* Move test directory command to public std namespace to enable duckscript unit testing
* Enable to clone commands
* Split wget and http_client command #66
* New array_pop command
* Commands created from duckscript now support help text and automatic scope clearing #69
* New clear_scope command #71
* New set_error command #68
* New is_array command #70
* Test errors in duckscript based tests will break build flow.
* New --version cli option.
* New version functions and commands for duckscript and duckscript SDK.
* New replace command.
* New current_time command.
* New greater_than and less_than commands.
* New wget (http_client) command #20
* Reduce binary executable size.
* Fix CLI help documentation.

### v0.1.7 (2020-01-17)

* Fixed runner to return an error if on_error requested crash or exit and not just end the script.
* \[Breaking Change\] Unalias can remove aliases not created from the alias command.
* New properties read/write commands #61
* Default command run implementation should crash and not error #63
* \[Breaking Change\] Invoking a command that does not exist should crash and not error
* cat command to support multiple files #62
* New debug commands (dump_instructions, dump_state, dump_variables) #58 #59 #60
* Running **duck** cli without arguments, stars up the repl mode #41 #42

### v0.1.6 (2020-01-12)

* \[Breaking Change\] Changed CLI executable from duckscript to duck
* function, forin and ifelse blocks to support generic end command #53
* duckscript cli now accepts inline script #40
* Unit test framework for internal SDK commands and full test suite #51
* New exit_on_error command #49
* Ability to write standard SDK commands with duckscript #50
* New array_is_empty command.
* \[Breaking Change\] assert commands should return 'crash' and stop execution #52

### v0.1.5 (2020-01-09)

* New array_length command
* \[Breaking Change\] ForIn accepts handle value not variable name
* New length command (strlen)
* New substring command #37
* New uname/os_family command #43
* \[Breaking Change\] Commands should accept empty string ("") inputs #47
* \[Breaking Change\] Support spread binding #46
* Command result of type error, will not break the script and instead call the on_error command #45
* New get_last_error, get_last_error_line and get_last_error_source commands #45
* New last_indexof command #36
* New indexof command #35
* Added 'quit' and 'q' aliases to exit command #44
* Added state 64 bit numeric value support #39
* \[Breaking Change\] Changed standard namespace to std from sdk #34
* New read command #33
* New hostname command #18
* New trim_start command #29
* New trim_end command #30
* New trim command
* New is_empty command
* New is_defined command
* New range command #32

### v0.1.4 (2020-01-03)

* New **contains** command #28
* New **ends_with** command #27
* New **starts_with** command #26
* New **equals** command #25

### v0.1.3 (2020-01-03)

* New **ls** command #9
* New **cp** command #7
* New **man** command #16
* New **calc** command #10
* New **unset_env** command #23
* New **mv** command #8
* New **rm** command #15
* New **rmdir** command #14
* New **assert_eq** command #22
* New **assert_fail** command #3
* New **assert** command #2
* New **touch** command #4
* New **dirname** command #6
* New **canonicalize** command #21
* New **basename** command #5
* New **mkdir** command #13
* New **not** command #12

### v0.1.0 (2019-12-30)

* Initial release
