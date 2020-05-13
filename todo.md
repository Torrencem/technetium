
This is a rough todo list of features I need to add to the language:

* make a technetium_hash method in Object trait (Hash is not object safe because of the generic parameter). Then derive Hash for each of the Object structs and make that work
* dictionaries
* eval
* lambdas
* list conversion function
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
