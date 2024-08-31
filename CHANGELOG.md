## CHANGELOG

### v0.10.0

* Enhancement: Runtime - \[Breaking Change\] New Env struct enabling commands to redirect out/err to provided streams #440

### v0.9.3 (2024-01-19)

* Fix: if/else condition with a command that accepts empty values #390
* Enhancement: dump commands will print to output if no output variable is defined

### v0.9.2 (2023-11-18)

* Enhancement: \[Breaking Change\] add support for renaming directories via mv command #374

### v0.9.1 (2023-09-17)

* Fix: panic on empty environment variable name provided for set_env command

### v0.9.0 (2023-09-08)

* Fix: Runtime - \[Breaking Change\] Empty spread should not count as an empty string argument #354

### v0.8.20 (2023-06-15)

* Maintenance: Upgrade dependencies

### v0.8.19 (2023-05-25)

* Maintenance: Upgrade dependencies

### v0.8.18 (2023-04-22)

* Fix: fix mv command for files when output does not exist #319 (thanks @burrbull)
* Maintenance: Replace math evaluation crate to evalexpr
* Fix: fix array_contains command

### v0.8.17 (2023-01-21)

* Enhancement: New zip/unzip commands #294 (thanks @Red-Teapot)
* Maintenance: Upgrade dependencies

### v0.8.16 (2022-10-17)

* Fix: watchdog command support for unlimited retries

### v0.8.15 (2022-10-05)

* Enhancement: Runtime - Add parser::parse_text_with_source_file #273 (thanks @coolreader18)

### v0.8.14 (2022-08-06)

* Maintenance: Runtime - Update dependencies
* Maintenance: Migrate from unmaintained ftp crate to suppaftp fork

### v0.8.13 (2022-07-21)

* Enhancement: Runtime - Enable to clone duckscript context #253 (thanks waterlens)
* Enhancement: Support both native TLS via openssl and pure rust TLS #258 (thanks @jirutka)

### v0.8.12 (2022-05-25)

* Enhancement: Add support for stdin input passing to child process in exec, watchdog and spawn commands #247
* Enhancement: Replace native TLS support via openssl with pure rust TLS
* Update dependencies

### v0.8.11 (2022-04-20)

* Fix: Runtime - fix control characters '\' parsing and expansion #237
* Enhancement: New get_file_size/filesize command #222
* Enhancement: Add include hidden files option for gitignore_path_array command #236 (thanks @Blightbuster)

### v0.8.10 (2021-12-10)

* New SDK and cli tls feature to enable usage without tls support (by default enabled)
* Update dependencies

### v0.8.9 (2021-11-01)

* Fix publish flow
* Update docs

### v0.8.8 (2021-11-01)

* New print and println commands which support styled output (color and style) #203

### v0.8.7 (2021-09-21)

* New digest command #196
* New sha256sum command #196
* New sha512sum command #196
* SDK docs now escape github emojis

### v0.8.6 (2021-09-20)

* Fix glob_cp command to support absolute paths #197

### v0.8.5 (2021-09-13)

* New join_path command.
* New glob_cp command #192
* set_env command now accepts a map handle and sets all env vars from the map

### v0.8.4 (2021-21-07)

* New lowercase command #183 (thanks @asvln)
* New uppercase command #183 (thanks @asvln)
* New camelcase command #183 (thanks @asvln)
* New snakecase command #183 (thanks @asvln)
* New kebabcase command #183 (thanks @asvln)
* Docs - update --collection documentation for json_parse and json_encode commands #175

### v0.8.3 (2021-07-10)

* Fix release with recursive flag to support set data structures as well.
* New --collection flag to json_parse command which returns maps/arrays instead of variables #175
* New --collection flag to json_encode command which encodes maps/arrays instead of variables #175

### v0.8.2 (2021-06-04)

* Upgrade dependencies #179

### v0.8.1 (2021-04-09)

* New gitignore_path_array command

### v0.8.0 (2021-03-08)

* Runtime - \[Breaking Change\] Make script errors chainable.

### v0.7.4 (2021-03-05)

* Fix flow control commands scope handling of call info stack

### v0.7.3 (2021-03-01)

* New hex_decode and hex_encode command #166 (thanks @umaYnit)

### v0.7.2 (2021-01-23)

* rm command support for multiple paths.
* Fix: Command `rm` remove file with flag `-r` #158 (thanks @umaYnit)
* Upgraded rand 0.8

### v0.7.1 (2020-12-13)

* Checked in Cargo.lock to help linux distributions package the duckscript CLI #146
* Alpine linux installation instructions #148 (thanks @jirutka)

### v0.7.0 (2020-11-26)

* Runtime - \[Breaking Change\] Break with an error if exit code with non zero value #144
* Adding --silent flag for spawn command #142

### v0.6.9 (2020-10-16)

* New while loop command #138
* New linter CLI command #139

### v0.6.8 (2020-10-01)

* Runtime - Fix variable expansion and support single $ and % characters #132

### v0.6.7 (2020-08-27)

* New --get-exit-code flag for exec command #127
* New random_range and random_text commands #128
* New semver_parse, semver_is_equal and semver_is_newer commands #129
* New is_command_defined command #130

### v0.6.6 (2020-08-14)

* Bug fix in exec command with fail on error flag.

### v0.6.5 (2020-07-31)

* Bug fix in substring when using negative end index only.

### v0.6.4 (2020-07-31)

* New json_encode command #124
* New json_parse command #124
* New unset_all_vars command.
* Module level documentation #125
* test_file command is now documented and public.

### v0.6.3 (2020-07-24)

* Reduce rustc minimal version.

### v0.6.2 (2020-07-24)

* New function &lt;scope&gt; annotation #121
* New scope_pop_stack command #121
* New scope_push_stack command #121

### v0.6.1 (2020-07-08)

* New is_path_newer command.
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
