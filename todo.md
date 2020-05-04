
This is a rough todo list of features I need to add to the language:

* more examples in readme
* negative indexing a la python (check [::-1])
* generators
* dictionaries
* use bigint
* .lines() clones; it could not, if Object methods took Rc instead of &self. Maybe some solution in terms of rental? or owning_ref?
* pass variables as reference? (for swap)
* comprehensions
* lambdas
* python syntax highlighting on github doesn't like $'s (so close!)
* tertiery expressions (if blah then blah else blah)
* add diagnostic message with some kind of full trace
* work on documentation for all standard library
* run clippy
* multi-file programs (import from another file)
* make windows use cmd.exe instead of sh
* standard library: get command line args
* read string literal \u{blahblah}
* make it easy to round floats when printing (I might want to write my own dtoa that is better)
* defer block
* benchmarks
