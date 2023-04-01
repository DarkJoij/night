use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub enum NightBuiltInErrorType {
    CoreError,
    SyntaxError,
    FloatError,
    ArgsLenError,
    ReadError
}

// This should be checked later, as it
// may not be skipped by various linters.
impl Display for NightBuiltInErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        <Self as Debug>::fmt(self, f)
    }
}

// Absurd or not?
#[macro_export]
macro_rules! spawn_core_error {
    ($($arg:tt)*) => {
        $crate::spawn_night_error!(
            $crate::frontend::errors::NightBuiltInErrorType::CoreError, $($arg)*
        )
    };
}

#[macro_export]
macro_rules! spawn_syntax_error {
    ($($arg:tt)*) => {
        $crate::spawn_night_error!(
            $crate::frontend::errors::NightBuiltInErrorType::SyntaxError, $($arg)*
        )
    };
}


#[macro_export]
macro_rules! spawn_float_error {
    ($($arg:tt)*) => {
        $crate::spawn_night_error!(
            $crate::frontend::errors::NightBuiltInErrorType::FloatError, $($arg)*
        )
    };
}

#[macro_export]
macro_rules! spawn_al_error {
    ($($arg:tt)*) => {
        $crate::spawn_night_error!(
            $crate::frontend::errors::NightBuiltInErrorType::ArgsLenError, $($arg)*
        )
    };
}

#[macro_export]
macro_rules! spawn_read_error {
    ($($arg:tt)*) => {
        $crate::spawn_night_error!(
            $crate::frontend::errors::NightBuiltInErrorType::ReadError, $($arg)*
        )
    };
}
