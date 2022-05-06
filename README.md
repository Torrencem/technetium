# Technetium

![Rust](https://github.com/Torrencem/technetium/workflows/Rust/badge.svg?event=push)

_technetium_ (or _tech_) is a scripting language with easy access to the shell.

[Documentation can be found here](https://matthewtorrence.com/technetium/)

## Installation

To install, setup cargo and rust, and then use the command ``cargo install --path .`` in the main directory to install the ``tech`` binary. You can also copy the ``scripts/tcmake`` script into your path to be able to use the ``tcmake`` command (relies on ``tech``).


## Examples

### Creating some files

```coffeescript
for letter in ['A', 'B', 'C', 'D'] {
	name = ("file_" + letter).escape()
	println(~"Creating file {name}")
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
println(c1())  # 1
c2 = create_counter()
println(c1())  # 2
println(c2())  # 1
println(c1())  # 3
```

### Countdown timer

```coffeescript
# Read the first command line argument as an integer
count = int(args()[0])

while count {
    # Print a message and return to the beginning
    # of the line with a carriage return
    printr(~"{count--}s remaining   ")

    # Wait for 1 second
    $ sleep 1
}

println()
```

### Sync Files Between Two Directories

```coffeescript
for file in stale(["dir_A/**/*"]) {
        # Find the path to the file within dir_A
        relative_path = strip_path_prefix(canonicalize(file), canonicalize("dir_A"))
        # Copy this file into dir_B
        $ cp {file} dir_B/{relative_path}
}
```