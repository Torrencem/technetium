use assert_cmd::Command;
// use assert_cmd::prelude::*;
use predicates::prelude::*;

type TestError = Box<dyn std::error::Error>;

#[test]
fn attribute_error() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin("tech")?;
    cmd.write_stdin(
        r#"
x = 10

println(x.incorrect)
"#,
    );

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("println(x.incorrect)")) // Gives the correct line
        .stderr(predicate::str::contains("Runtime Error: AttributeError")) // Gives the correct type of error
        .stderr(predicate::str::contains("int")); // Mentions the type

    Ok(())
}

#[test]
fn method_error() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin("tech")?;
    cmd.write_stdin(
        r#"
x = 10

println(x.incorrect())
"#,
    );

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("println(x.incorrect())")) // Gives the correct line
        .stderr(predicate::str::contains("Runtime Error: AttributeError")) // Gives the correct type of error
        .stderr(predicate::str::contains("method"))
        .stderr(predicate::str::contains("int")); // Mentions the type

    Ok(())
}

#[test]
fn index_oob() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin("tech")?;
    cmd.write_stdin(
        r#"
l = [1, 2, 3]

println(l[3])
"#,
    );

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("println(l[3])")) // Gives the correct line
        .stderr(predicate::str::contains("Index out of bounds")); // Mentions the variable

    let mut cmd = Command::cargo_bin("tech")?;
    cmd.write_stdin(
        r#"
l = [1, 2, 3]

l[3] += 1
"#,
    );

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("l[3] += 1")) // Gives the correct line
        .stderr(predicate::str::contains("Index out of bounds")); // Mentions the variable

    Ok(())
}

#[test]
fn unknown_variable() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin("tech")?;
    cmd.write_stdin(
        r#"
x = 10

println(y)
"#,
    );

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("println(y)")) // Gives the correct line
        .stderr(predicate::str::contains("Undefined variable: y")); // Mentions the variable

    Ok(())
}

#[test]
fn unknown_variable2() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin("tech")?;
    cmd.write_stdin(
        r#"
x = 10

println(fib(x))
"#,
    );

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("println(fib(x))"))
        .stderr(predicate::str::contains("Undefined variable"))
        .stderr(predicate::str::contains("Undefined function: fib"));

    Ok(())
}

#[test]
fn recursive_format_error() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin("tech")?;
    cmd.write_stdin(
        r#"

my_num = 100

if my_num > 50 {
	$ cat /dev/urandom | head -c {my_num * 3 * my_num.no_attr() }
} else {
	println(~"Number too small! The number is {my_num}")
}

println(5)

"#,
    );

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Attribute"))
        .stderr(predicate::str::contains("method"))
        .stderr(predicate::str::contains("int"))
        .stderr(predicate::str::contains("my_num.no_attr()"));

    Ok(())
}

#[test]
fn recursive_lex_error() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin("tech")?;
    cmd.write_stdin(
        r#"

# Pad it out a bit for offsetting to matter
my_name = "Matt"

println(~"My name is not {my_name * 1.2.3}")
"#,
    );

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Lex Error"))
        .stderr(predicate::str::contains("decimal"))
        .stderr(predicate::str::contains("my_name * 1.2.3"));

    Ok(())
}

#[test]
fn recursive_parse_error() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin("tech")?;
    cmd.write_stdin(
        r#"
# more padding, to make sure offsets work correctly

println(~"{val * * 2}")
"#,
    );

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Parse error"))
        .stderr(predicate::str::contains("Mult"))
        .stderr(predicate::str::contains("val * * 2"));

    Ok(())
}

#[test]
fn dict_key_error() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin("tech")?;
    cmd.write_stdin(
        r#"
d = {1: 2}

println(d["hello"])
"#,
    );

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("key"))
        .stderr(predicate::str::contains("println(d[\"hello\"])"));

    Ok(())
}

#[test]
fn multiple_error_reporting() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin("tech")?;
    cmd.write_stdin(
        r#"
a = 2 *

b = 3 * 4 *
"#,
    );

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("2 *"))
        .stderr(predicate::str::contains("3 * 4 *"));

    Ok(())
}


#[test]
fn special_char_lex() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin("tech")?;
    cmd.write_stdin(
        r#"
l = [1]

print(@l@@@@@)
        "#);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("special char"));

    Ok(())
}
