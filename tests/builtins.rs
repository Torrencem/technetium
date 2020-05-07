use assert_cmd::Command;
use predicates::prelude::*;

type TestError = Box<dyn std::error::Error>;

#[test]
fn test_builtins() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin("tc")?;
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
fn index_list_tuple() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin("tc")?;
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
fn test_list_arith() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin("tc")?;
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
fn test_list_tuple_eq() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin("tc")?;
    cmd.write_stdin(
        r#"
print((1, 2) == (1, 2))
print((1, 2) == (1, 3))
print([1, 2, 3] == [1])
print([1, 2, [3, [4]]] == [1, 2, [3, [4]]])
"#,
    );

    cmd.assert().success().stdout(predicate::eq("true\nfalse\nfalse\ntrue\n"));

    Ok(())
}

