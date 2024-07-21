:i count 8
:b shell 70
./bin/torvo b tests/hello.torv -o tests/out/hello && ./tests/out/hello
:i returncode 0
:b stdout 6
Hello

:b stderr 0

:b shell 103
./bin/torvo b tests/func_declaration.torv -o tests/out/func_declaration && ./tests/out/func_declaration
:i returncode 0
:b stdout 6
Hello

:b stderr 0

:b shell 94
./bin/torvo b tests/global_string.torv -o tests/out/global_string && ./tests/out/global_string
:i returncode 0
:b stdout 18
Hello from global

:b stderr 0

:b shell 124
./bin/torvo b tests/global_string_from_func.torv -o tests/out/global_string_from_func && ./tests/out/global_string_from_func
:i returncode 0
:b stdout 18
Hello from global

:b stderr 0

:b shell 61
./bin/torvo b tests/if.torv -o tests/out/if && ./tests/out/if
:i returncode 0
:b stdout 22
it's true!
it's false

:b stderr 0

:b shell 91
./bin/torvo b tests/if_returning.torv -o tests/out/if_returning && ./tests/out/if_returning
:i returncode 0
:b stdout 22
it's true!
it's false

:b stderr 0

:b shell 88
./bin/torvo b tests/record_type.torv -o tests/out/record_type && ./tests/out/record_type
:i returncode 101
:b stdout 0

:b stderr 178
thread 'main' panicked at src/typecheck/module_checker.rs:104:9:
assertion failed: stack.len() == 1
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

:b shell 82
./bin/torvo b tests/recursion.torv -o tests/out/recursion && ./tests/out/recursion
:i returncode 0
:b stdout 7
got 10

:b stderr 0

