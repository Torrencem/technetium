use assert_cmd::Command;
use predicates::prelude::*;

type TestError = Box<dyn std::error::Error>;

#[test]
fn capture_variables() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.write_stdin(
        r#"
func create_counter() {
    value = 0
    func count() {
        value += 1
        return value
    }
    return count
}

c1 = create_counter()
print(c1())
c2 = create_counter()
print(c1())
print(c2())
print(c1())
"#,
    );

    cmd.assert().success().stdout(predicate::eq("1\n2\n1\n3\n"));

    Ok(())
}

#[test]
fn loops() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.write_stdin(
        r#"
x = 10
while x > 1 {
    print(x)
    x -= 2
}

for val in [123, 123.456, "world"] {
    print("Hello " + val)
}
"#,
    );

    cmd.assert().success().stdout(predicate::eq(
        "10\n8\n6\n4\n2\nHello 123\nHello 123.456\nHello world\n",
    ));

    Ok(())
}

#[test]
fn recursive_fib() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.write_stdin(
        r#"
func fib(n) {
    if n <= 2 {
        return 1
    } else {
        return fib(n - 1) + fib(n - 2)
    }
}

print(fib(5))
print(fib(10))
print(fib(15))
"#,
    );

    cmd.assert().success().stdout(predicate::eq("5\n55\n610\n"));

    Ok(())
}

#[test]
fn index_list_tuple() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.write_stdin(
        r#"
print([1, 2, 3, "hello"][0])
print(["we", 123.01, 999][2])
print((1, 2, 3, "hello")[0])
print(("we", 123.01, 999)[2])
"#,
    );

    cmd.assert()
        .success()
        .stdout(predicate::eq("1\n999\n1\n999\n"));

    Ok(())
}

#[test]
fn index_set() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.write_stdin(
        r#"
l = [1, 2, 3]

l[0] = "Pie"

print(l[0])
"#,
    );

    cmd.assert().success().stdout(predicate::eq("Pie\n"));

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.write_stdin(
        r#"
l = [1, "w", 3]

l[2] += 100

print(l[2])
"#,
    );

    cmd.assert().success().stdout(predicate::eq("103\n"));

    Ok(())
}

#[test]
fn method_simple() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.write_stdin(
        r#"
print([1, 2, 3, "hello"].length())
print(["we", 123.01, 999].length())
"#,
    );

    cmd.assert().success().stdout(predicate::eq("4\n3\n"));

    Ok(())
}

#[test]
fn comments() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.write_stdin(
        r#"
print("Hello world!")  # To the world
print("Hi again!")          # To whom?
"#,
    );

    cmd.assert()
        .success()
        .stdout(predicate::eq("Hello world!\nHi again!\n"));

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.write_stdin(
        r#"
# blank here
print("Hello world!")  # To the world
# Fully blank line

print("Hi again!")          # To whom?
"#,
    );

    cmd.assert()
        .success()
        .stdout(predicate::eq("Hello world!\nHi again!\n"));

    Ok(())
}

#[test]
fn test_builtins() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.write_stdin(
        r#"
x = 5
y = 6
z = 7
if x < y && x <= y {
    print(1)
}
if y > x && y >= x {
    print(2)
}
if x <= x && x >= x && x == x && x != y {
    print(3)
}
if x < y || y < x {
    print(4)
}
if (x * y) / y == x {
    print(5)
}
x = 5.0
y = 6.0
z = 7.0
if x < y && x <= y {
    print(1)
}
if y > x && y >= x {
    print(2)
}
if x <= x && x >= x && x == x && x != y {
    print(3)
}
if x < y || y < x {
    print(4)
}
"#,
    );

    cmd.assert()
        .success()
        .stdout(predicate::eq("1\n2\n3\n4\n5\n1\n2\n3\n4\n"));

    Ok(())
}

#[test]
fn test_range() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.write_stdin(
        r#"
for i in range(5) {
	print(i)
}

for i in range(-10, -5) {
	print(i)
}

for i in range(10, 100, 10) {
	print(i)
}
"#,
    );

    cmd.assert().success().stdout(predicate::eq(
        r#"0
1
2
3
4
-10
-9
-8
-7
-6
10
20
30
40
50
60
70
80
90
"#,
    ));

    Ok(())
}

#[test]
fn test_sh_and_substitution() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.write_stdin(
        r#"
my_var = "hello"
my_var2 = "hello2"

$ echo {my_var} and {my_var2}
"#,
    );

    cmd.assert()
        .success()
        .stdout(predicate::eq("hello and hello2\n"));

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.write_stdin(
        r#"
my_num = 10

if my_num > 50 {
	$ cat /dev/urandom | head -c {my_num * 3}
} else {
	print(~"Number too small! The number is {my_num}")
}
"#,
    );

    cmd.assert()
        .success()
        .stdout(predicate::eq("Number too small! The number is 10\n"));

    Ok(())
}

#[test]
fn test_substitution() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.write_stdin(
        r#"
x = 10
s = ~"I can say x isn't {x + 2}"
print(s)
print(~"S was: {s}")
"#,
    );

    cmd.assert().success().stdout(predicate::eq(
        "I can say x isn't 12\nS was: I can say x isn't 12\n",
    ));

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.write_stdin(
        r#"
my_num = 10

if my_num > 50 {
	$ cat /dev/urandom | head -c {my_num * 3}
} else {
	print(~"Number too small! The number is {my_num}")
}
"#,
    );

    cmd.assert()
        .success()
        .stdout(predicate::eq("Number too small! The number is 10\n"));

    Ok(())
}

#[test]
fn test_case_of() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.write_stdin(
        r#"
my_num = 1000

case my_num + 20 of {
	10 => print("no"),
	100 => {
		print("no!")
	},
	1000 => {
		print("no")
	},
	1020 => {
		print("yes!")
	},
}

"#,
    );

    cmd.assert().success().stdout(predicate::eq("yes!\n"));

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.write_stdin(
        r#"
my_val = "Hello there!"

case my_val of {
    "not really" => print("no"),
	"Hello thar!" => {
		print("no!")
	},
    "Hello there!" => print("yes!"),
}
"#,
    );

    cmd.assert().success().stdout(predicate::eq("yes!\n"));

    Ok(())
}

#[test]
fn test_sh_objects() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.write_stdin(
        r#"

my_num = 123

program = sh(~"echo {my_num}")

program.join()

print(~"program output was: {program.stdout()}")
"#,
    );

    cmd.assert()
        .success()
        .stdout(predicate::eq("program output was: 123\n\n")); // Second newline comes from output of echo
    Ok(())
}

#[test]
fn test_no_string_deadlock() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.write_stdin(
        r#"
a = "hello"

b = a

print(a == b)
print(a != b)
"#,
    );

    cmd.assert()
        .success()
        .stdout(predicate::eq("true\nfalse\n"));
    Ok(())
}

#[test]
fn test_bool_literals() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.write_stdin(
        r#"
print(false && true)

print(true && true)

print(false || true)
"#,
    );

    cmd.assert()
        .success()
        .stdout(predicate::eq("false\ntrue\ntrue\n"));
    Ok(())
}

#[test]
fn test_char_indexing() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.write_stdin(
        r#"
my_name = "matt"

my_name[0] = 'M'

print(my_name)
print(my_name[2])
"#,
    );

    cmd.assert().success().stdout(predicate::eq("Matt\nt\n"));
    Ok(())
}

#[test]
fn test_short_circuiting() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.write_stdin(
        r#"
print(true || exit(1))
"#,
    );

    cmd.assert().success().stdout(predicate::eq("true\n"));

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.write_stdin(
        r#"
print(false || exit(0))
"#,
    );

    cmd.assert().success().stdout(predicate::eq(""));

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.write_stdin(
        r#"
print(true && exit(0))
"#,
    );

    cmd.assert().success().stdout(predicate::eq(""));
    
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.write_stdin(
        r#"
print(false && exit(1))
"#,
    );

    cmd.assert().success().stdout(predicate::eq("false\n"));

    Ok(())
}

#[test]
fn test_clone() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.write_stdin(
        r#"
a = "Hello"
b = clone(a)

b[0] = 'h'

print(a)
"#,
    );

    cmd.assert().success().stdout(predicate::eq("Hello\n"));

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.write_stdin(
        r#"
a = "Hello"
b = a

b[0] = 'h'

print(a)
"#,
    );

    cmd.assert().success().stdout(predicate::eq("hello\n"));

    Ok(())
}

#[test]
fn test_conversions() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.write_stdin(
        r#"
f = 3.14
i = 3
s = "3.14"
si = "3"
print(int(f) == i)
print(abs(float(s) - f) < 0.001)
print(int(si) == i)
print(bool(f) && bool(i) && bool(s) && bool(si))
"#,
    );

    cmd.assert().success().stdout(predicate::eq("true\ntrue\ntrue\ntrue\n"));

    Ok(())
}

#[test]
fn test_char_conversions() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.write_stdin(
        r#"
c = '💯'

print(int(c))
print(char(int(c)))
"#,
    );

    cmd.assert().success().stdout(predicate::eq("128175\n💯\n"));

    Ok(())
}

#[test]
fn test_escape() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.write_stdin(
        r#"
s = "random\t\"string\"\n\n"

print(s.escape())
"#,
    );

    cmd.assert().success().stdout(predicate::eq("random\\t\\\"string\\\"\\n\\n\n"));

    Ok(())
}

#[test]
fn test_list_arith() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.write_stdin(
        r#"
l = [1] * 3 + [2] * 3

print(l)
"#,
    );

    cmd.assert().success().stdout(predicate::eq("[1, 1, 1, 2, 2, 2]\n"));

    Ok(())
}

#[test]
fn test_slices() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.write_stdin(
        r#"
list = [1, 2, 5, 10]

print(list[::2])
print(list[1::2])
print(list[0:3])
print(list[2:-1:-1])
"#,
    );

    cmd.assert().success().stdout(predicate::eq("[1, 5]\n[2, 10]\n[1, 2, 5]\n[5, 2, 1]\n"));

    Ok(())
}
