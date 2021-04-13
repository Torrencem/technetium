use assert_cmd::Command;
use predicates::prelude::*;

type TestError = Box<dyn std::error::Error>;

#[test]
fn test_builtins() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin("tech")?;
    cmd.write_stdin(
        r#"
x = 5
y = 6
z = 7
if x < y && x <= y {
    println(1)
}
if y > x && y >= x {
    println(2)
}
if x <= x && x >= x && x == x && x != y {
    println(3)
}
if x < y || y < x {
    println(4)
}
if (x * y) / y == x {
    println(5)
}
x = 5.0
y = 6.0
z = 7.0
if x < y && x <= y {
    println(1)
}
if y > x && y >= x {
    println(2)
}
if x <= x && x >= x && x == x && x != y {
    println(3)
}
if x < y || y < x {
    println(4)
}
"#,
    );

    cmd.assert()
        .success()
        .stdout(predicate::eq("1\n2\n3\n4\n5\n1\n2\n3\n4\n"));

    Ok(())
}

#[test]
fn index_list_tuple() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin("tech")?;
    cmd.write_stdin(
        r#"
println([1, 2, 3, "hello"][0])
println(["we", 123.01, 999][2])
println((1, 2, 3, "hello")[0])
println(("we", 123.01, 999)[2])
"#,
    );

    cmd.assert()
        .success()
        .stdout(predicate::eq("1\n999\n1\n999\n"));

    Ok(())
}

#[test]
fn test_list_arith() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin("tech")?;
    cmd.write_stdin(
        r#"
l = [1] * 3 + [2] * 3

println(l)
"#,
    );

    cmd.assert()
        .success()
        .stdout(predicate::eq("[1, 1, 1, 2, 2, 2]\n"));

    Ok(())
}

#[test]
fn test_list_tuple_eq() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin("tech")?;
    cmd.write_stdin(
        r#"
println((1, 2) == (1, 2))
println((1, 2) == (1, 3))
println([1, 2, 3] == [1])
println([1, 2, [3, [4]]] == [1, 2, [3, [4]]])
"#,
    );

    cmd.assert()
        .success()
        .stdout(predicate::eq("true\nfalse\nfalse\ntrue\n"));

    Ok(())
}

#[test]
fn test_bitwise_operators() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin("tech")?;
    cmd.write_stdin(
        r#"
println(123 & 321)
println(123 | 321)
println(123 ^ 321)
"#,
    );

    cmd.assert()
        .success()
        .stdout(predicate::eq("65\n379\n314\n"));

    Ok(())
}

#[test]
fn test_tuples() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin("tech")?;
    cmd.write_stdin(
        r#"
l = (1,)

println(type(l))

println(l)

l2 = (1, 2, 3)

println(l2)
"#,
    );

    cmd.assert()
        .success()
        .stdout(predicate::eq("tuple\n(1,)\n(1, 2, 3)\n"));

    Ok(())
}

#[test]
fn test_slice_eq() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin("tech")?;
    cmd.write_stdin(
        r#"
println(range(3)[0:2] == [0, 1])
println(range(3)[0:2] == [0, 1, 2])
println(range(3)[0:2] == [1, 2])
println(range(3)[0:2] == [0, 1][0:2])
println(range(3)[0:2] == [])
println(range(3)[0:2] == [0, 2])

println([0, 1] == range(3)[0:2])
println([0, 2] == range(3)[0:2])

mystring = "hello!"

println("he" == mystring[0:2])

"#,
    );

    cmd.assert()
        .success()
        .stdout(predicate::eq("true\nfalse\nfalse\ntrue\nfalse\nfalse\ntrue\nfalse\ntrue\n"));

    Ok(())
}
