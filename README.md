# Technetium

![Rust](https://github.com/Torrencem/technetium/workflows/Rust/badge.svg?event=push)

Technetium is a sleek interpreted programming language that features inline shell statements, and other tools useful for short build / install scripts.

## Features

* Inline shell statements with very practical integration, and super-intuitive child process first-class objects via `sh`

* Super-simple syntax. A familiar, non-controversial syntax that plays nice with expectations

* A set of ever-convenient and always-expanding utility functions (for example, `os()` will return the current operating system as a string) and dead-easy conversion functions

* UTF-8 by default, which sensible character types that play nicely with strings

* Anything else you expect from an interpreted scripting language in present year: useful slices, first class functions that capture scope, (generators and dictionaries coming soon! Check todo.md for a full list of to come features)

## Examples

### Creating some files

```
for letter in ['A', 'B', 'C', 'D'] {
	name = "file_" + letter
	print(~"Creating file {name}")
	$ echo "{name}" > {name}.txt
}
```
