use assert_cmd::Command;
// use assert_cmd::prelude::*;
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
"#);

    cmd.assert()
        .success()
        .stdout(predicate::eq("1\n2\n1\n3\n"));

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
"#);

    cmd.assert()
        .success()
        .stdout(predicate::eq("10\n8\n6\n4\n2\nHello 123\nHello 123.456\nHello world\n"));

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
"#);

    cmd.assert()
        .success()
        .stdout(predicate::eq("5\n55\n610\n"));

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
"#);

    cmd.assert()
        .success()
        .stdout(predicate::eq("1\n999\n1\n999\n"));

    Ok(())
}

#[test]
fn method_simple() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.write_stdin(
r#"
print([1, 2, 3, "hello"].length())
print(["we", 123.01, 999].length())
"#);

    cmd.assert()
        .success()
        .stdout(predicate::eq("4\n3\n"));

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
"#);

    cmd.assert()
        .success()
        .stdout(predicate::eq("1\n2\n3\n4\n5\n1\n2\n3\n4\n"));

    Ok(())
}
