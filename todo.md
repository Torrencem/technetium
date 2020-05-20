
This is a rough todo list of features I need to add to the language:

* void keyword
* lambdas
* generators
* comprehensions
* defer block
* add some functions relating to https://github.com/xdg-rs/dirs
* interactive mode (a la python)
* tertiery expressions (if blah then blah else blah)
* pass Files from main around in compile context, to prepare for multi-file programs
* multi-file programs (import from another file) using "module" objects
* add multiple error reporting for format strings
* 2 places where memory is essentially leaked: Reference cycles (fix with change to Gc and GcCell), and "parent references"
* windows?
* unicode string literal \u{blahblah}
* make it easy to round floats when printing (I might want to write my own dtoa that is better)
* benchmarks
