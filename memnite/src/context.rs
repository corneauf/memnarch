use std::path::PathBuf;
use std::env;

pub trait ContextProvider {
    type Args;

    fn with(to_hold: &Self::Args) -> Self;
}
pub struct ChangeCwd {
    cwd: PathBuf,
}

impl ContextProvider for ChangeCwd {
    type Args = PathBuf;

    fn with(path: &Self::Args) -> Self {
        let context = ChangeCwd {
            cwd: env::current_dir().unwrap()
        };

        env::set_current_dir(path).unwrap();

        context
    }
}

impl Drop for ChangeCwd {
    fn drop(&mut self) {
        env::set_current_dir(&self.cwd).unwrap();
    }
}
