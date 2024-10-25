use anyhow::Result;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

pub fn call(command: &str) -> Result<()> {
    let mut command = Command::new(command).stdout(Stdio::piped()).spawn()?;

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
