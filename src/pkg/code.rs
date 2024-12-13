#[repr(i32)]
pub enum ExitCode {
    Exit = 65,
    Continue = 0,
}

impl From<ExitCode> for i32 {
    fn from(value: ExitCode) -> Self {
        value as i32
    }
}