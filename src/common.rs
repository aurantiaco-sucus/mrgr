use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};

#[repr(transparent)]
struct NotDebugged<T> {
    inner: T
}

impl<T> Deref for NotDebugged<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for NotDebugged<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T> NotDebugged<T> {
    pub fn new(data: T) -> NotDebugged<T> {
        NotDebugged { inner: data }
    }

    pub fn unwrap(self) -> T {
        self.inner
    }
}

impl<T> From<T> for NotDebugged<T> {
    fn from(value: T) -> Self {
        NotDebugged::new(value)
    }
}

impl<T> Debug for NotDebugged<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Object Not Debugged>")
    }
}