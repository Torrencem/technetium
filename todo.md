
This is a rough todo list of features I need to add to the language:

* Get rid of temp panics in lexer, add lexing error type and propogate correctly
* case of
* many tests for format string instruction
* `sh` expressions (shell objects with methods and attributes)
* standard library: sh, cd, get command line args, platform
* mutable strings (should dup from constants, i.e. worry about "changing" GCD's)
* generators
* multi-file programs (import from another file)
* standard library math (sin, cos, etc.)
* work on documentation
* benchmarks
* slices
* (+) for lists
* defer block
