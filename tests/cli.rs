use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn valid_command() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("tictoc")?;

    cmd.arg("pwd");
    cmd.assert().success();

    Ok(())
}

#[test]
fn valid_command_arg() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("test.md")?;
    file.write_str("test")?;

    let mut cmd = Command::cargo_bin("tictoc")?;

    cmd.arg("find").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test.md"));

    Ok(())
}

#[test]
fn valid_command_hyphen() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("tictoc")?;

    cmd.arg("ls").arg("-a");
    cmd.assert().success();

    Ok(())
}

#[test]
fn valid_command_many_args() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("file-to-look-in")?;
    file.write_str("phrase-to-look-for")?;
    let mut cmd = Command::cargo_bin("tictoc")?;

    cmd.arg("grep")
        .arg("--colour-auto")
        .arg("-i")
        .arg("-v")
        .arg("-q")
        .arg("phrase-to-look-for")
        .arg("file-to-look-in");
    cmd.assert().success();

    Ok(())
}

#[test]
fn invalid_command() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("tictoc")?;

    cmd.arg("this-is-not-a-valid-command");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Could not execute command"));

    Ok(())
}

#[test]
fn invalid_args() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("tictoc")?;

    cmd.arg("ls").arg("-j");
    cmd.assert()
        .success()
        .stderr(predicate::str::contains("invalid"));

    Ok(())
}
