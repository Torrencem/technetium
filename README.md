# Technetium

![Rust](https://github.com/Torrencem/technetium/workflows/Rust/badge.svg?event=push)

Technetium is a sleek interpreted programming language that features inline shell statements, and other tools useful for short build / install scripts.

[Documentation can be found here](https://matthewtorrence.com/technetium/)

Since the project is still very young, all suggestions are welcome! A contributing guide with notes on internals and other considerations is to come.

## Features

* Inline shell statements with very practical integration, and super-intuitive child process first-class objects via `sh`

* Super-simple syntax. A familiar, non-controversial syntax that plays nice with expectations

* A set of ever-convenient and always-expanding utility functions (for example, `os()` will return the current operating system as a string) and dead-easy conversion functions

* UTF-8 by default, with sensible character types that play nicely with strings

## Examples

### Creating some files

```coffeescript
for letter in ['A', 'B', 'C', 'D'] {
	name = "file_" + letter
	print(~"Creating file {name}")
	$ echo "{name}" > {name}.txt
}
```

### Creating an index of files in the current directory

```coffeescript
# Run "ls" on the command line, blocking, and get the standard output
files = sh("ls").stdout()

for line in files.lines() {  # For each line in the string
	# Append the line to index.txt
	$ echo {line} >> index.txt
}

```

### Complicated Environment Capturing

```coffeescript
func create_counter() {
    value = 1
    # Return an anonymous function that increments and returns "value"
    return \() -> value++
}

c1 = create_counter()
print(c1())  # 1
c2 = create_counter()
print(c1())  # 2
print(c2())  # 1
print(c1())  # 3
```
