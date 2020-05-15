
This is a rough todo list of features I need to add to the language:

* dictionaries
* add contains method to all collections
* add documentation for special_funcs (hash, lock, print, exit, etc.)
* allow newlines after commas in list and tuple and set literals
* eval
* lambdas
* list and set conversion functions that just iterate and add
* generators
* comprehensions
* defer block
* interactive mode (a la python)
* tertiery expressions (if blah then blah else blah)
* pass Files from main around in compile context, to prepare for multi-file programs
* multi-file programs (import from another file) using "module" objects
* 2 places where memory is essentially leaked: Reference cycles (fix with change to Gc and GcCell), and "parent references"
* windows?
* standard library: get command line args
* unicode string literal \u{blahblah}
* make it easy to round floats when printing (I might want to write my own dtoa that is better)
* benchmarks
