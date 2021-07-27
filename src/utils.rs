use std::process::Command;
use std::str::from_utf8;

pub struct CommandResult {
    pub status: Option<i32>,
    pub stdout: Option<String>,
    pub stderr: Option<String>
}

pub fn execute_command(cmd: &str, args: Option<&[&str]>) -> CommandResult {
    // println!("DEB: {} {:?}", cmd, args);
    let mut command = Command::new(cmd);
    if args.is_some() {
        command.args(args.unwrap());
    }
    let mut result = CommandResult{status: None, stdout: None, stderr: None};
    let output = command.output().expect("ERR: Command failed!");
    result.status = output.status.code();
    if output.stderr.len() > 0 {
        result.stderr = match from_utf8(output.stdout.as_slice()) {
            Ok(v) => Some(v.to_string()),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
    }
    if output.stdout.len() > 0 {
        result.stdout = match from_utf8(output.stdout.as_slice()) {
            Ok(v) => Some(v.to_string()),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
    }
    result
}
