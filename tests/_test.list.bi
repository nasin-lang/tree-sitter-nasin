:i count 9
:b shell 69
./bin/nasin b tests/hello.nsn -o tests/out/hello && ./tests/out/hello
:i returncode 0
:b stdout 42
Compiled program to tests/out/hello
Hello

:b stderr 0

:b shell 102
./bin/nasin b tests/func_declaration.nsn -o tests/out/func_declaration && ./tests/out/func_declaration
:i returncode 0
:b stdout 53
Compiled program to tests/out/func_declaration
Hello

:b stderr 0

:b shell 93
./bin/nasin b tests/global_string.nsn -o tests/out/global_string && ./tests/out/global_string
:i returncode 0
:b stdout 62
Compiled program to tests/out/global_string
Hello from global

:b stderr 0

:b shell 123
./bin/nasin b tests/global_string_from_func.nsn -o tests/out/global_string_from_func && ./tests/out/global_string_from_func
:i returncode 0
:b stdout 72
Compiled program to tests/out/global_string_from_func
Hello from global

:b stderr 0

:b shell 60
./bin/nasin b tests/if.nsn -o tests/out/if && ./tests/out/if
:i returncode 0
:b stdout 54
Compiled program to tests/out/if
it's true
it's false

:b stderr 0

:b shell 90
./bin/nasin b tests/if_returning.nsn -o tests/out/if_returning && ./tests/out/if_returning
:i returncode 0
:b stdout 64
Compiled program to tests/out/if_returning
it's true
it's false

:b stderr 0

:b shell 87
./bin/nasin b tests/record_type.nsn -o tests/out/record_type && ./tests/out/record_type
:i returncode 0
:b stdout 60
Compiled program to tests/out/record_type
Hello from record

:b stderr 0

:b shell 81
./bin/nasin b tests/recursion.nsn -o tests/out/recursion && ./tests/out/recursion
:i returncode 0
:b stdout 47
Compiled program to tests/out/recursion
got 10

:b stderr 0

:b shell 72
./bin/nasin b tests/method.nsn -o tests/out/method && ./tests/out/method
:i returncode 0
:b stdout 65
Compiled program to tests/out/method
Hello from record
Hi method

:b stderr 0

