use assert_cmd::Command;
use predicates::prelude::*;

type TestError = Box<dyn std::error::Error>;

#[test]
fn capture_variables() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.write_stdin(
        r#"
func create_counter() {
    value = 1
    func count() {
        return value++
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
fn test_post_pre_ops() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.write_stdin(
        r#"
l = [0, 0, 0, 0]

print(l[1]++)  # 0
print(l[1])  # 1

print(++l[0])  # 1
print(l[0])  # 1
print(--l[0])  # 0
print(l[0])  # 0
"#,
    );

    cmd.assert()
        .success()
        .stdout(predicate::eq("0\n1\n1\n1\n0\n0\n"));
    Ok(())
}

#[test]
fn test_negative_indexing() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.write_stdin(
        r#"
l = [0, 1, 2, 3, 4, 5]

print(l[::-1])
print(l[-2])
print(l[-6])
l[-1] = 100
print(l[5])

s = "Hello!"
s[-1] = '?'
print(s)
"#,
    );

    cmd.assert()
        .success()
        .stdout(predicate::eq("[5, 4, 3, 2, 1, 0]\n4\n0\n100\nHello?\n"));
    Ok(())
}

