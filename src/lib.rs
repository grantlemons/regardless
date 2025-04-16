use std::{
    error::Error as StdError,
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
};

#[macro_export]
macro_rules! regardless {
    ($s:literal) => {
        Error::from_str($s)
    };
    ($fstring:literal, $($arg:tt)*) => {
        Error::from_string(format!($fstring, $($arg)*))
    };
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub struct Error {
    inner: Box<dyn StdError + Send + Sync + 'static>,
    context: Vec<String>,
}

impl Error {
    pub fn from_str(s: &str) -> Self {
        Self {
            inner: s.into(),
            context: Vec::new(),
        }
    }

    pub fn from_string(s: String) -> Self {
        Self {
            inner: s.into(),
            context: Vec::new(),
        }
    }
}

impl Deref for Error {
    type Target = dyn StdError + Send + Sync + 'static;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

impl DerefMut for Error {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.as_mut()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.inner,
            self.context
                .iter()
                .fold(String::new(), |acc, c| acc + "\n" + c)
        )
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl<E> From<E> for Error
where
    E: StdError + Send + Sync + 'static,
{
    fn from(value: E) -> Self {
        Self {
            inner: Box::new(value),
            context: Vec::new(),
        }
    }
}

impl Error {
    pub fn extend_context(&mut self, s: String) {
        self.context.push(s)
    }
}

pub trait Context<T, E> {
    fn context<C>(self, context: C) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static;
    fn with_context<C, F>(self, f: F) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C;
}

impl<T, E> Context<T, E> for Result<T, E>
where
    E: StdError + Send + Sync + 'static,
{
    fn context<C>(self, context: C) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static,
    {
        match self {
            Ok(ok) => Ok(ok),
            Err(error) => Err({
                let mut res = Error::from(error);
                res.extend_context(context.to_string());
                res
            }),
        }
    }

    fn with_context<C, F>(self, context: F) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        match self {
            Ok(ok) => Ok(ok),
            Err(error) => Err({
                let mut res = Error::from(error);
                res.extend_context(context().to_string());
                res
            }),
        }
    }
}

impl<T> Context<T, Error> for Result<T, Error> {
    fn context<C>(self, context: C) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static,
    {
        match self {
            Ok(ok) => Ok(ok),
            Err(error) => Err({
                let mut res = error;
                res.extend_context(context.to_string());
                res
            }),
        }
    }

    fn with_context<C, F>(self, context: F) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        match self {
            Ok(ok) => Ok(ok),
            Err(error) => Err({
                let mut res = error;
                res.extend_context(context().to_string());
                res
            }),
        }
    }
}

impl From<Error> for Box<dyn StdError + Send + 'static> {
    fn from(error: Error) -> Self {
        error.inner
    }
}

impl AsRef<dyn StdError + Send + Sync> for Error {
    fn as_ref(&self) -> &(dyn StdError + Send + Sync + 'static) {
        &**self
    }
}
