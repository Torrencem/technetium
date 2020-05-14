
This is a rough todo list of features I need to add to the language:

* turn debug descriptor generation in compile into a function which returns a new descriptor, and replace all of the times it's used with a call to the function
* dictionaries
* add documentation for special_funcs (hash, lock, print, exit, etc.)
* figure out a way to make a macro for implementing Object for ObjectCell stuff, so the .lock_immutable() boilerplate isn't necessary
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
