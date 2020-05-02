# Technetium

![Rust](https://github.com/Torrencem/technetium/workflows/Rust/badge.svg?event=push)

Technetium is a sleek interpreted programming language that features inline shell statements, and other tools useful for short build / install scripts.

## Example

```
for letter in ['A', 'B', 'C', 'D'] {
	name = "file_" + letter
	print(~"Creating file {name}")
	$ echo "{name}" > {name}.txt
}
```
