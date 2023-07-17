# NonSensitive
A marker trait for sensitive information.  
# Examples
This will compile error:
```rust
struct Password(SensitiveData<String>);
impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

fn post_tweet<T: NonSensitive + AsRef<str>, P: AsRef<str>>(msg: T, pwd: P) {
    twitter::post(msg.as_ref(), pwd);
}

fn main() {
    let pwd = Password(SensitiveData::new("wutdedogdoin1!".to_string()));
    let msg = "Hello, world! *Beep boops aggressively*";

    post_tweet(pwd, msg); // Uh oh, we flipped the message and the password!
}
```
But `NonSensitive` comes to the rescue!  
Instead of posting our twitter password, we get an error.  
Error message:
```
the trait bound `sensitive_trait::SensitiveData<String>: sensitive_trait::NonSensitive` is not satisfied in `Password`
```