#[cfg(test)]
mod tests {
    use assert_cmd::Command;

    #[test]
    fn test_tool_runs() {
        let mut cmd = Command::cargo_bin("Jay_Liu_week7_rust").unwrap();
        cmd.arg("--input")
            .arg("test_input.txt")
            .assert()
            .success();
    }
}
