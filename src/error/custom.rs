use failure::{Backtrace, Fail};
use regex::Regex;
use std::fmt::{self, Debug, Display};
use std::path::PathBuf;

#[derive(Debug, Fail)]
#[fail(display = "{{ code: {:?}, msg: {} }}", code, msg)]
pub struct CodeMsgError {
    pub code: Option<i32>,
    pub msg: String,
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
    pub msg: String,
}

impl MsgError {
    pub fn new<S>(msg: S) -> MsgError
    where
        S: Into<String>,
    {
        MsgError { msg: msg.into() }
    }
}

#[derive(Debug, Fail)]
#[fail(display = "{{ path: {:?}, inner: {} }}", path, inner)]
pub struct PathError<E>
where
    E: Fail,
{
    pub path: PathBuf,
    #[cause]
    pub inner: E,
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
#[fail(display = "{{ perm: {} }}", perm)]
pub struct PermError {
    pub perm: String,
}

impl PermError {
    pub fn new<P>(perm: P) -> PermError
    where
        P: Into<String>,
    {
        PermError { perm: perm.into() }
    }
}

#[derive(Debug, Fail)]
#[fail(display = "{{ query: {}, inner: {} }}", query, inner)]
pub struct QueryError<E>
where
    E: Fail,
{
    pub query: String,

    #[cause]
    pub inner: E,
}

impl<E> QueryError<E>
where
    E: Fail,
{
    pub fn new<Q>(query: Q, inner: E) -> QueryError<E>
    where
        Q: Into<String>,
    {
        QueryError {
            query: query.into(),
            inner,
        }
    }
}

#[derive(Debug, Fail)]
#[fail(display = "{{ pattern: {}, target: {} }}", pattern, target)]
pub struct RegexCaptureError {
    pub pattern: String,
    pub target: String,
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
#[fail(
    display = "{{ found len: {}, expected len: {}, target: {}, regex: {} }}",
    found_len,
    expected_len,
    target,
    re_str
)]
pub struct RegexMinCaptureError {
    pub found_len: usize,
    pub expected_len: usize,
    pub target: String,
    pub re_str: String,
}

impl RegexMinCaptureError {
    pub fn new<T>(
        found_len: usize,
        expected_len: usize,
        target: T,
        re: &Regex,
    ) -> RegexMinCaptureError
    where
        T: Into<String>,
    {
        RegexMinCaptureError {
            found_len,
            expected_len,
            target: target.into(),
            re_str: re.as_str().to_owned(),
        }
    }
}

#[derive(Debug, Fail)]
#[fail(display = "{{ target: {}, inner: {} }}", target, inner)]
pub struct TargetStringError<E>
where
    E: Fail,
{
    pub target: String,
    #[cause]
    pub inner: E,
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

#[derive(Debug)]
pub struct ValueError<T>
where
    T: Debug + Display + Send + Sync + 'static,
{
    pub desc: String,
    pub value: T,
}

impl<T> Fail for ValueError<T>
where
    T: Debug + Display + Sync + Send + 'static,
{
    fn cause(&self) -> Option<&Fail> {
        None
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        None
    }
}

impl<T> Display for ValueError<T>
where
    T: Debug + Display + Sync + Send + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ desc: {}, value: {} }}", self.desc, self.value)
    }
}

impl<T> ValueError<T>
where
    T: Debug + Display + Send + Sync + 'static,
{
    pub fn new<S>(desc: S, value: T) -> ValueError<T>
    where
        S: Into<String>,
    {
        ValueError {
            desc: desc.into(),
            value,
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
    fn test_perm_error_trait() {
        PermError::new("Fake perm").context(FakeErrorKind);
    }

    #[test]
    fn test_query_error_trait() {
        QueryError::new("Fake query", FakeError).context(FakeErrorKind);
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
    fn test_regex_min_capture_error_trait() {
        let fake_regex = Regex::new("");
        assert!(fake_regex.is_ok());
        let fake_regex = fake_regex.unwrap();

        RegexMinCaptureError::new(0, 0, "Fake target", &fake_regex)
            .context(FakeErrorKind);
    }

    #[test]
    fn test_target_string_error_trait() {
        TargetStringError::new("Fake", FakeError).context(FakeErrorKind);
    }

    #[test]
    fn test_value_error_trait() {
        ValueError::new("Fake description", 123).context(FakeErrorKind);
    }
}
