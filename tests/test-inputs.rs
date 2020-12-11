use std::path::Path;
use std::process::{Command, Stdio};

#[test]
fn everything_sandwich() -> anyhow::Result<()> {
    run_moonwave("passing/everything_sandwich.lua", 0)
}

#[test]
fn failing_function() -> anyhow::Result<()> {
    run_moonwave("failing/function.lua", 1)
}

fn run_moonwave(file_name: &str, expected_status: i32) -> anyhow::Result<()> {
    let path = Path::new("test-input").join(file_name);

    let child = Command::new(env!("CARGO_BIN_EXE_moonwave"))
        .arg("extract")
        .arg(path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .env("NO_COLOR", "1")
        .spawn()?;

    let output = child.wait_with_output()?;
    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;

    assert_eq!(output.status.code(), Some(expected_status));

    let stdout_name = format!("{}-stdout", file_name);
    let stderr_name = format!("{}-stderr", file_name);

    insta::assert_snapshot!(stdout_name, stdout);
    insta::assert_snapshot!(stderr_name, stderr);

    Ok(())
}