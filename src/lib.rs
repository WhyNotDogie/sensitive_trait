#![no_std]
#![feature(auto_traits)]
#![feature(negative_impls)]

//! A marker trait for sensitive information.
//! For more information, see documentation on the NonSensitive struct.

use core::ops::{Deref, DerefMut};

/// A marker trait for sensitive information.  
/// # Examples
/// This will compile error:
/// ```compile_fail
/// # mod twitter {pub fn post<T>(v: &str, a: T) {}}
/// # use sensitive_trait::*;
/// #
/// struct Password(SensitiveData<String>);
/// impl AsRef<str> for Password {
///     fn as_ref(&self) -> &str {
///         self.0.as_str()
///     }
/// }
/// 
/// fn post_tweet<T: NonSensitive + AsRef<str>, P: AsRef<str>>(msg: T, pwd: P) {
///     twitter::post(msg.as_ref(), pwd);
/// }
/// 
/// fn main() {
///     let pwd = Password(SensitiveData::new("wutdedogdoin1!".to_string()));
///     let msg = "Hello, world! *Beep boops aggressively*";
/// 
///     post_tweet(pwd, msg); // Uh oh, we flipped the message and the password!
/// }
/// ```
/// But `NonSensitive` comes to the rescue!  
/// Instead of posting our twitter password, we get an error.  
/// Error message:
/// ```text
/// the trait bound `sensitive_trait::SensitiveData<String>: sensitive_trait::NonSensitive` is not satisfied in `Password`
/// ```
pub auto trait NonSensitive {}

/// A transparent wrapper type that makes any value implement !NonSensitive.  
/// This type is guarunteed to have the same size as the inner data.
/// For an example, see the documentation on `NonSensitive`.
#[repr(transparent)]
pub struct SensitiveData<T>(T);

impl<T> !NonSensitive for SensitiveData<T> {}

impl<T> SensitiveData<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }
    pub fn unwrap(self) -> T {
        self.0
    }
}

impl<T> Clone for SensitiveData<T> where T: Clone {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Deref for SensitiveData<T> {
    fn deref(&self) -> &Self::Target {
        &self.0
    }

    type Target = T;
}

impl<T> DerefMut for SensitiveData<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Copy for SensitiveData<T> where T: Copy {}