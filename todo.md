
This is a rough todo list of features I need to add to the language:

* Instead of no drops, clone reference, make all changes (store instruction, things that are currently RwLock's) interior mutation
* switch from Rc to Gc and from RefCell to GcCell
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
