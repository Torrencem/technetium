use assert_cmd::Command;
// use assert_cmd::prelude::*;
use predicates::prelude::*;

type TestError = Box<dyn std::error::Error>;

#[test]
fn attribute_error() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.write_stdin(
r#"
x = 10

print(x.incorrect)
"#);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("print(x.incorrect)")) // Gives the correct line
        .stderr(predicate::str::contains("Runtime Error: AttributeError")) // Gives the correct type of error
        .stderr(predicate::str::contains("int")); // Mentions the type

    Ok(())
}

#[test]
fn method_error() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.write_stdin(
r#"
x = 10

print(x.incorrect())
"#);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("print(x.incorrect())")) // Gives the correct line
        .stderr(predicate::str::contains("Runtime Error: AttributeError")) // Gives the correct type of error
        .stderr(predicate::str::contains("method")) 
        .stderr(predicate::str::contains("int")); // Mentions the type

    Ok(())
}

#[test]
fn index_oob() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.write_stdin(
r#"
l = [1, 2, 3]

print(l[3])
"#);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("print(l[3])")) // Gives the correct line
        .stderr(predicate::str::contains("Index out of bounds")); // Mentions the variable
    
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.write_stdin(
r#"
l = [1, 2, 3]

l[3] += 1
"#);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("l[3] += 1")) // Gives the correct line
        .stderr(predicate::str::contains("Index out of bounds")); // Mentions the variable

    Ok(())
}

#[test]
fn unknown_variable() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.write_stdin(
r#"
x = 10

print(y)
"#);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("print(y)")) // Gives the correct line
        .stderr(predicate::str::contains("Undefined variable: y")); // Mentions the variable

    Ok(())
}

#[test]
fn unknown_variable2() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.write_stdin(
r#"
x = 10

print(fib(x))
"#);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("print(fib(x))"))
        .stderr(predicate::str::contains("Undefined variable"))
        .stderr(predicate::str::contains("Undefined function: fib"));

    Ok(())
}

#[test]
fn recursive_format_error() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.write_stdin(
r#"

my_num = 100

if my_num > 50 {
	$ cat /dev/urandom | head -c {my_num * 3 * my_num.no_attr() }
} else {
	print(~"Number too small! The number is {my_num}")
}

print(5)

"#);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Attribute"))
        .stderr(predicate::str::contains("method"))
        .stderr(predicate::str::contains("int"))
        .stderr(predicate::str::contains("my_num.no_attr()"));

    Ok(())
}
