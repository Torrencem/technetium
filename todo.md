
This is a rough todo list of features I want to add to the language:

* multiplication for lists and strings!
* The following code stackoverflows:

```
l = [1]

l.push(l)

println(l)
```

Equivelent code in Python is smart enough to notice that `l[i] is l`, and so at that spot, it prints `[...]`. It's even smart enough to notice when `l[i][j] is l`!
It looks like the code in CPython that's responsible for this is here: https://github.com/python/cpython/blob/e5fe509054183bed9aef42c92da8407d339e8af8/Objects/listobject.c#L373

* Following example should show in error message when it was locked in the source code:
```
a = {2}
b = {3}
a.add(b)
b.add(5)
println(a)
```

* @ in the middle of programs doesn't cause errors
* Print / Println is dumb
* Allow empty dict literal
* Let set() be empty set (no args)
* Union / intersection / subtraction of sets (with + ^ and - respectively)
* allow integer literals outside the range of i64
* Document "using tcmake as a build system"
* Update function doc style according to https://pythonhosted.org/an_example_pypi_project/sphinx.html#function-definitions
* Add a bunch of examples to documentation, especially for map filter
* add some functions relating to https://github.com/xdg-rs/dirs
* add reduce
* way to get reference to self in a lambda expression (a la this in javascript)
* defer
* interactive mode (a la python)
* tertiery expressions (if blah then blah else blah)
* generators
* comprehensions
* pass Files from main around in compile context, to prepare for multi-file programs
* multi-file programs (import from another file) using "module" objects
* add multiple error reporting for format strings
* 2 places where memory is essentially leaked: Reference cycles (fix with change to Gc and GcCell), and "parent references"
* windows?
* unicode string literal \u{blahblah}
* make it easy to round floats when printing
* benchmarks
