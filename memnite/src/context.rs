use std::path::PathBuf;
use std::env;

pub trait ContextProvider: Drop {
    type HeldType;

    fn with(to_hold: Self::HeldType) -> Self;
}
pub struct ChangeCwd {
    cwd: PathBuf,
}

impl ContextProvider for ChangeCwd {
    type HeldType = PathBuf;

    fn with(path: PathBuf) -> Self {
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
