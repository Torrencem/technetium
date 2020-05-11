
This is a rough todo list of features I need to add to the language:

* Fix a fundamental problem: I want the Object trait methods to have Rc<...>, but then: 1. how do I do dispatch? It seems impossible, and 2. if Self is anywhere in the types, the trait Object will not be Object safe
* Make all object methods take a ObjectRef to self (no nightly, use "this")
* Instead of no drops, clone reference, make all changes (store instruction) interior mutation
* dictionaries
* comprehensions
* lambdas
* tertiery expressions (if blah then blah else blah)
* add full stack trace to diagnostic message in src/error.rs
* eval
* .lines() clones; it could not
* generators
* multi-file programs (import from another file) using "module" objects
* make windows use cmd.exe instead of sh
* standard library: get command line args
* unicode string literal \u{blahblah}
* make it easy to round floats when printing (I might want to write my own dtoa that is better)
* defer block
* benchmarks
