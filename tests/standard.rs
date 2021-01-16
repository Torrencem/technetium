use assert_cmd::Command;
use predicates::prelude::*;

type TestError = Box<dyn std::error::Error>;

#[test]
fn test_singleton_set() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin("tech")?;
    cmd.write_stdin(
        r#"
l = {1}

println(l)
        "#,
        );
    
    cmd.assert().success().stdout(predicate::eq("{1}\n"));

    Ok(())
}

#[test]
fn test_index_set() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin("tech")?;
    cmd.write_stdin(
        r#"
l = [1, 2, 3]

l[0] = "Pie"

println(l[0])
"#,
    );

    cmd.assert().success().stdout(predicate::eq("Pie\n"));

    let mut cmd = Command::cargo_bin("tech")?;
    cmd.write_stdin(
        r#"
l = [1, "w", 3]

l[2] += 100

println(l[2])
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
	println(i)
}

for i in range(-10, -5) {
	println(i)
}

for i in range(10, 100, 10) {
	println(i)
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

println(a == b)
println(a != b)
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
println(false && true)

println(true && true)

println(false || true)
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

println(my_name)
println(my_name[2])
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

println(a)
"#,
    );

    cmd.assert().success().stdout(predicate::eq("Hello\n"));

    let mut cmd = Command::cargo_bin("tech")?;
    cmd.write_stdin(
        r#"
a = "Hello"
b = a

b[0] = 'h'

println(a)
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
println(int(f) == i)
println(abs(float(s) - f) < 0.001)
println(int(si) == i)
println(bool(f) && bool(i) && bool(s) && bool(si))
"#,
    );

    cmd.assert()
        .success()
        .stdout(predicate::eq("true\ntrue\ntrue\ntrue\n"));

    Ok(())
}

#[test]
fn test_char_conversions() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin("tech")?;
    cmd.write_stdin(
        r#"
c = '💯'

println(int(c))
println(char(int(c)))
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

println(s.escape())
"#,
    );

    cmd.assert()
        .success()
        .stdout(predicate::eq("random\\t\\\"string\\\"\\n\\n\n"));

    Ok(())
}

#[test]
fn test_slices() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin("tech")?;
    cmd.write_stdin(
        r#"
list = [1, 2, 5, 10]

println(list[::2])
println(list[1::2])
println(list[0:3])
println(list[2:-1:-1])
"#,
    );

    cmd.assert()
        .success()
        .stdout(predicate::eq("[1, 5]\n[2, 10]\n[1, 2, 5]\n[5, 2, 1]\n"));

    let mut cmd = Command::cargo_bin("tech")?;
    cmd.write_stdin(
        r#"
list = [1, 2, 5, 10]

list[::2][1] = 100

println(list)
"#,
    );

    cmd.assert()
        .success()
        .stdout(predicate::eq("[1, 2, 100, 10]\n"));

    Ok(())
}

#[test]
fn test_lines() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin("tech")?;
    cmd.write_stdin(
        r#"
s = "abc\n123\nj\n"
for line in s.lines() {
    println(line)
    println("-")
}
"#,
    );

    cmd.assert()
        .success()
        .stdout(predicate::eq("abc\n-\n123\n-\nj\n-\n"));

    Ok(())
}

#[test]
fn test_contains() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin("tech")?;
    cmd.write_stdin(
        r#"
s = "hello world"

println(s.contains('w'))
println(s.contains('!'))

l = [1, 2, {15, "hi"}]

println(l.contains({15, "hi"}))
println(l.contains({20, "hi"}))

t = (1, 2, {15, "hi"})

println(t.contains({15, "hi"}))
println(t.contains({20, "hi"}))
"#,
    );

    cmd.assert()
        .success()
        .stdout(predicate::eq("true\nfalse\ntrue\nfalse\ntrue\nfalse\n"));

    Ok(())
}

#[test]
fn test_args() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin("tech")?;
    cmd.write_stdin(
        r#"
println(args())
"#,
    );

    cmd.assert().success().stdout(predicate::eq("[]\n"));

    let mut cmd = Command::cargo_bin("tech")?;
    cmd.args(&["--", "alpha", "beta", "gamma"]);
    cmd.write_stdin(
        r#"
println(args())
"#,
    );

    cmd.assert()
        .success()
        .stdout(predicate::eq("[alpha, beta, gamma]\n"));

    Ok(())
}

#[test]
fn test_map() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin("tech")?;
    cmd.write_stdin(
        r#"
l = [10, 100]

println(list(map(l, \x -> x / 10)))
"#,
    );

    cmd.assert().success().stdout(predicate::eq("[1, 10]\n"));

    Ok(())
}

#[test]
fn test_filter() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin("tech")?;
    cmd.write_stdin(
        r#"
l = [1, 2, 3, 4, 5, 6]

l = list(filter(l, \x -> x % 2 == 0))

println(l)
"#,
    );

    cmd.assert().success().stdout(predicate::eq("[2, 4, 6]\n"));

    Ok(())
}

#[test]
fn test_dict_conversion() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin("tech")?;
    cmd.write_stdin(
        r#"
println(dict([["a", 1], ["b", 2]]))
println(dict())
"#,
    );

    cmd.assert().success().stdout(predicate::eq("{b: 2, a: 1}\n{}\n"));

    Ok(())
}
