use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_version() {
    let mut cmd = Command::cargo_bin("fera").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("fera"));
}

#[test]
fn test_help() {
    let mut cmd = Command::cargo_bin("fera").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Fera programming language"));
}

#[test]
fn test_build_hello_world() {
    let temp_dir = TempDir::new().unwrap();
    let source_path = temp_dir.path().join("hello.fera");
    
    fs::write(&source_path, r#"
        export i32 main() {
            return 0;
        }
    "#).unwrap();
    
    let mut cmd = Command::cargo_bin("fera").unwrap();
    cmd.arg("build")
        .arg(&source_path)
        .current_dir(temp_dir.path())
        .assert()
        .success();
}

#[test]
fn test_check_valid_code() {
    let temp_dir = TempDir::new().unwrap();
    let source_path = temp_dir.path().join("valid.fera");
    
    fs::write(&source_path, r#"
        i32 add(i32 a, i32 b) {
            return a + b;
        }
        
        export i32 main() {
            i32 result = add(1, 2);
            return result;
        }
    "#).unwrap();
    
    let mut cmd = Command::cargo_bin("fera").unwrap();
    cmd.arg("check")
        .arg(&source_path)
        .assert()
        .success();
}

#[test]
fn test_clean() {
    let temp_dir = TempDir::new().unwrap();
    
    let mut cmd = Command::cargo_bin("fera").unwrap();
    cmd.arg("clean")
        .arg(temp_dir.path())
        .assert()
        .success();
}

