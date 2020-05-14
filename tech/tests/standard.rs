use assert_cmd::Command;
use predicates::prelude::*;

type TestError = Box<dyn std::error::Error>;

#[test]
fn index_set() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin("tech")?;
    cmd.write_stdin(
        r#"
l = [1, 2, 3]

l[0] = "Pie"

print(l[0])
"#,
    );

    cmd.assert().success().stdout(predicate::eq("Pie\n"));

    let mut cmd = Command::cargo_bin("tech")?;
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
fn test_range() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin("tech")?;
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
fn test_no_string_deadlock() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin("tech")?;
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
    let mut cmd = Command::cargo_bin("tech")?;
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
    let mut cmd = Command::cargo_bin("tech")?;
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
fn test_clone() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin("tech")?;
    cmd.write_stdin(
        r#"
a = "Hello"
b = clone(a)

b[0] = 'h'

print(a)
"#,
    );

    cmd.assert().success().stdout(predicate::eq("Hello\n"));

    let mut cmd = Command::cargo_bin("tech")?;
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
    let mut cmd = Command::cargo_bin("tech")?;
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
    let mut cmd = Command::cargo_bin("tech")?;
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
    let mut cmd = Command::cargo_bin("tech")?;
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
fn test_slices() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin("tech")?;
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
    
    let mut cmd = Command::cargo_bin("tech")?;
    cmd.write_stdin(
        r#"
list = [1, 2, 5, 10]

list[::2][1] = 100

print(list)
"#,
    );

    cmd.assert().success().stdout(predicate::eq("[1, 2, 100, 10]\n"));

    Ok(())
}

#[test]
fn test_lines() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin("tech")?;
    cmd.write_stdin(
        r#"
s = "abc\n123\nj\n"
for line in s.lines() {
    print(line)
    print("-")
}
"#,
    );

    cmd.assert().success().stdout(predicate::eq("abc\n-\n123\n-\nj\n-\n"));
    
    Ok(())
}
