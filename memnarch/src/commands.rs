use std::ffi::OsStr;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

use anyhow::Result;

pub fn call(command: &str) -> Result<()> {
    call_with::<[_; 0], &str>(command, [])
}

pub fn call_with<I, S>(command: &str, args: I) -> Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut command = Command::new(command)
        .stdout(Stdio::piped())
        .args(args)
        .spawn()?;

    {
        let stdout = command.stdout.take().unwrap();
        let stdout_reader = BufReader::new(stdout);
        let stdout_lines = stdout_reader.lines();

        for line in stdout_lines {
            println!("{}", line?);
        }
    }

    command.wait().unwrap();

    Ok(())
}
