## CHANGELOG

### v0.1.6

* assert commands should return 'crash' and stop execution #52

### v0.1.5 (2020-01-09)

* New array_length command
* ForIn accepts handle value not variable name
* New length command (strlen)
* New substring command #37
* New uname/os_family command #43
* Commands should accept empty string ("") inputs #47
* Support spread binding #46
* Command result of type error, will not break the script and instead call the on_error command #45
* New get_last_error, get_last_error_line and get_last_error_source commands #45
* New last_indexof command #36
* New indexof command #35
* Added 'quit' and 'q' aliases to exit command #44
* Added state 64 bit numeric value support #39
* Changed standard namespace to std from sdk #34
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
