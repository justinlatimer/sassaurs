use std::borrow::Cow;

pub fn compile<'a, S>(sass: S) -> Result<String, Cow<'a, str>> where S : Into<String> {
    Ok(sass.into())
}
