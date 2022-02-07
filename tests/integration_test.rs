use std::{
    io::Write,
    process::{Command, Stdio},
};

#[test]
fn fails_no_message() {
    let mut cmd = Command::new("./target/debug/foxsay");
    cmd.stdin(Stdio::null());
    let out = cmd.output().unwrap();

    assert!(!out.status.success());
    assert!(!out.stderr.is_empty());
}

#[test]
fn takes_stdin_input() {
    let mut cmd = Command::new("./target/debug/foxsay");
    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::null());
    let mut child = cmd.spawn().unwrap();
    child
        .stdin
        .as_mut()
        .unwrap()
        .write(b"Hello, world!")
        .unwrap();
    let out = child.wait_with_output().unwrap();

    assert!(out.status.success());
    assert!(out.stderr.is_empty());
}

#[test]
fn fails_with_no_message_and_stdin() {
    let mut cmd = Command::new("./target/debug/foxsay");
    cmd.stdin(Stdio::piped());
    let out = cmd.output().unwrap();

    assert!(!out.status.success());
    assert!(!out.stderr.is_empty());
}
