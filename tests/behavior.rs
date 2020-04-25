use assert_cmd::Command;
// use assert_cmd::prelude::*;
use predicates::prelude::*;

type TestError = Box<dyn std::error::Error>;

#[test]
fn capture_variables() -> Result<(), TestError> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    // cmd.arg(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/counter.rsh"));
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
    // cmd.arg(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/counter.rsh"));
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
