use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Debug)]
pub enum NightBuiltInErrorType {
    SyntaxError,
    FloatError,
    ArgsLenError,
    ReadError,
    NameError,
    TypeError,
    OperatorError
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

#[macro_export]
macro_rules! spawn_name_error {
    ($($arg:tt)*) => {
        $crate::spawn_night_error!(
            $crate::frontend::errors::NightBuiltInErrorType::NameError, $($arg)*
        )
    };
}

#[macro_export]
macro_rules! spawn_operator_error {
    ($($arg:tt)*) => {
        $crate::spawn_night_error!(
            $crate::frontend::errors::NightBuiltInErrorType::OperatorError, $($arg)*
        )
    };
}

#[macro_export]
macro_rules! spawn_type_error {
    ($($arg:tt)*) => {
        $crate::spawn_night_error!(
            $crate::frontend::errors::NightBuiltInErrorType::TypeError, $($arg)*
        )
    };
}
