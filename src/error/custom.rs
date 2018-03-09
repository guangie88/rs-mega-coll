use failure::Fail;
use regex::Regex;
use std::path::PathBuf;

#[derive(Debug, Fail)]
#[fail(display = "{{ code: {:?}, msg: {} }}", code, msg)]
pub struct CodeMsgError {
    code: Option<i32>,
    msg: String,
}

impl CodeMsgError {
    pub fn new<C, S>(code: C, msg: S) -> CodeMsgError
    where
        C: Into<Option<i32>>,
        S: Into<String>,
    {
        CodeMsgError {
            code: code.into(),
            msg: msg.into(),
        }
    }
}

#[derive(Debug, Fail)]
#[fail(display = "{{ msg: {} }}", msg)]
pub struct MsgError {
    msg: String,
}

impl MsgError {
    pub fn new<S>(msg: S) -> MsgError
    where
        S: Into<String>,
    {
        MsgError {
            msg: msg.into(),
        }
    }
}

#[derive(Debug, Fail)]
#[fail(display = "{{ path: {:?}, inner: {} }}", path, inner)]
pub struct PathError<E>
where
    E: Fail,
{
    path: PathBuf,
    #[cause]
    inner: E,
}

impl<E> PathError<E>
where
    E: Fail,
{
    pub fn new<P>(path: P, inner: E) -> PathError<E>
    where
        P: Into<PathBuf>,
    {
        PathError {
            path: path.into(),
            inner,
        }
    }
}

#[derive(Debug, Fail)]
#[fail(display = "{{ pattern: {}, target: {} }}", pattern, target)]
pub struct RegexCaptureError {
    pattern: String,
    target: String,
}

impl RegexCaptureError {
    pub fn new<T>(pattern: &Regex, target: T) -> RegexCaptureError
    where
        T: Into<String>,
    {
        RegexCaptureError {
            pattern: pattern.as_str().to_owned(),
            target: target.into(),
        }
    }
}

#[derive(Debug, Fail)]
#[fail(display = "{{ target: {}, inner: {} }}", target, inner)]
pub struct TargetStringError<E>
where
    E: Fail,
{
    target: String,
    #[cause]
    inner: E,
}

impl<E> TargetStringError<E>
where
    E: Fail,
{
    pub fn new<S>(target: S, inner: E) -> TargetStringError<E>
    where
        S: Into<String>,
    {
        TargetStringError {
            target: target.into(),
            inner,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use failure::Fail;

    #[cfg_attr(feature = "cargo-clippy", allow(empty_line_after_outer_attr))]
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
    #[fail(display = "Fake error kind")]
    pub struct FakeErrorKind;

    #[derive(Debug, Fail)]
    #[fail(display = "Fake error")]
    struct FakeError;

    #[test]
    fn test_code_msg_error_trait() {
        CodeMsgError::new(None, "Fake").context(FakeErrorKind);
    }

    #[test]
    fn test_msg_error_trait() {
        MsgError::new("Fake").context(FakeErrorKind);
    }

    #[test]
    fn test_path_error_trait() {
        PathError::new("Fake path", FakeError).context(FakeErrorKind);
    }

    #[test]
    fn test_regex_capture_error_trait() {
        let fake_regex = Regex::new("");
        assert!(fake_regex.is_ok());
        let fake_regex = fake_regex.unwrap();

        RegexCaptureError::new(&fake_regex, "Fake target")
            .context(FakeErrorKind);
    }

    #[test]
    fn test_target_string_error_trait() {
        TargetStringError::new("Fake", FakeError).context(FakeErrorKind);
    }
}
