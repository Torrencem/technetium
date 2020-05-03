
This is a rough todo list of features I need to add to the language:

* .lines() clones; it could not, if Object methods took Arc instead of &self
* .lines() also uses chars .nth when it probably shouldn't
* more examples in readme
* replace weak\_debug with a vec of debug spans
* negative indexing a la python (check [::-1])
* generators
* dictionaries
* use bigint
* pass variables as reference? (for swap)
* comprehensions
* lambdas
* python syntax highlighting on github doesn't like $'s (so close!)
* tertiery expressions (if blah then blah else blah)
* work on documentation for all standard library
* run clippy
* multi-file programs (import from another file)
* make windows use cmd.exe instead of sh
* standard library: get command line args
* read string literal \u{blahblah}
* make it easy to round floats when printing (I might want to write my own dtoa that is better)
* defer block
* benchmarks
