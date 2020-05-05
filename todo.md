
This is a rough todo list of features I need to add to the language:

* s[:-1]
* dictionaries
* use bigint
* comprehensions
* lambdas
* tertiery expressions (if blah then blah else blah)
* add diagnostic message with some kind of full trace
* list methods
* .lines() clones; it could not, if Object methods took Rc<Self> instead of &self. (use nightly?)
* run clippy
* generators
* pass variables as reference? (for swap)
* multi-file programs (import from another file)
* make windows use cmd.exe instead of sh
* standard library: get command line args
* read string literal \u{blahblah}
* make it easy to round floats when printing (I might want to write my own dtoa that is better)
* defer block
* benchmarks
