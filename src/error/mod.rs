use failure::{Backtrace, Context, Fail};
use std;
use std::fmt::{self, Debug, Display};

pub mod custom;

// suppress false positives from cargo-clippy
#[cfg_attr(feature = "cargo-clippy", allow(empty_line_after_outer_attr))]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "Child output error")]
    ChildOutput,

    #[fail(display = "CSV entry record parse error")]
    CsvEntryParse,

    #[fail(display = "Database entry cache creation error")]
    DbEntryCreate,

    #[fail(display = "Database entry cache delete error")]
    DbEntryDelete,

    #[fail(display = "Parse database entry expiry error")]
    DbEntryExpiryParse,

    #[fail(display = "Database entry serialize error")]
    DbEntrySerialize,

    #[fail(display = "Get database new iterator error")]
    DbNewIter,

    #[fail(display = "Database file open error")]
    DbOpen,

    #[fail(display = "Default logger initialization error")]
    DefaultLoggerInit,

    #[fail(display = "Directory flag is unexpectedly empty")]
    DirFlagEmpty,

    #[fail(display = "Unable to create directories for copying to destination")]
    DirsCreate,

    #[fail(display = "Error piping password echo")]
    EchoPwPipe,

    #[fail(display = "File I/O error")]
    FileIo,

    #[fail(display = "Initial fluent post check error")]
    FluentInitCheck,

    #[fail(display = "Fluent post from tagged record error")]
    FluentPostTaggedRecord,

    #[fail(display = "Match glob error")]
    Glob,

    #[fail(display = "Hash date time parse error")]
    HashDateTimeParse,

    #[fail(display = "Error invoking hdfs dfs -copyToLocal")]
    HdfsCopyToLocal,

    #[fail(display = "Error running hdfs dfs -df command")]
    HdfsDfCmd,

    #[fail(display = "Error invoking hdfs dfs -ls")]
    HdfsDfsLs,

    #[fail(display = "Cannot find hdfs command")]
    HdfsNotAvailable,

    #[fail(display = "Error creating regex for hdfs matches")]
    HdfsRegexMatch,

    #[fail(display = "Invalid date target")]
    InvalidDateTarget,

    #[fail(display = "Invalid path after glob")]
    InvalidPathAfterGlob,

    #[fail(display = "kinit for username and keytab combi returns error")]
    KinitKeytab,

    #[fail(display = "Cannot find kinit command")]
    KinitNotAvailable,

    #[fail(display = "kinit for username and password combi returns error")]
    KinitPw,

    #[fail(display = "Lock file open error")]
    LockFileOpen,

    #[fail(display = "Lock file exclusive lock error")]
    LockFileExclusiveLock,

    #[fail(display = "Cannot parse hdfs dfs -df size value")]
    ParseHdfsDfSizeValue,

    #[fail(display = "Cannot parse hdfs dfs -df used value")]
    ParseHdfsDfUsedValue,

    #[fail(display = "Cannot connect to Postgres server")]
    PgConnection,

    #[fail(display = "Cannot execute Postgres query to get database sizes")]
    PgGetDbSizes,

    #[fail(display = "Unable to parse naive date time")]
    NaiveDateTimeParse,

    #[fail(display = "Unable to parse file size from regex capture")]
    RegexCapFileSizeParse,

    #[fail(display = "Unable to regex capture permissions")]
    RegexCapPerm,

    #[fail(display = "Cannot capture values from hdfs dfs -df extraction")]
    RegexHdfsDfValuesCap,

    #[fail(display = "Cannot get initial hdfs dfs -df regex capture")]
    RegexInitialHdfsDfCap,

    #[fail(display = "Regex minimum capture error")]
    RegexMinCapture,

    #[fail(display = "Specialized logger initialization error")]
    SpecializedLoggerInit,

    #[fail(display = "Error getting statvfs on path")]
    Statvfs,

    #[fail(display = "Conversion from UTF8 stderr to string fail")]
    StderrUtf8Conversion,

    #[fail(display = "Stderr is empty")]
    StderrEmpty,

    #[fail(display = "Error reading from stderr pipe")]
    StderrRead,

    #[fail(display = "Error with message in stderr")]
    StderrValidMsg,

    #[fail(display = "Conversion from UTF8 stdout to string fail")]
    StdoutUtf8Conversion,

    #[fail(display = "Unable to strip root '/' from path")]
    StripRootPath,

    #[fail(display = "Pretty string conversion error")]
    ToStringPretty,

    #[fail(display = "TOML config parse error")]
    TomlConfigParse,
}

#[derive(Debug)]
pub struct Error<K>
where
    K: Copy + Clone + Eq + PartialEq + Debug + Fail,
{
    pub inner: Context<K>,
}

pub type Result<T> = std::result::Result<T, Error<ErrorKind>>;

impl<K> Fail for Error<K>
where
    K: Copy + Clone + Eq + PartialEq + Debug + Fail,
{
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

fn fail_opt_to_str(opt: &Option<&Fail>) -> String {
    match *opt {
        Some(cause) => format!(", cause: {}", rec_format_fail(cause)),
        None => "".to_owned(),
    }
}

fn backtrace_opt_to_str(opt: &Option<&Backtrace>) -> String {
    match *opt {
        Some(backtrace) => {
            let s = format!("{}", backtrace);

            if s.is_empty() {
                s
            } else {
                format!(", backtrace: {}", s)
            }
        }
        None => "".to_owned(),
    }
}

fn rec_format_fail(f: &Fail) -> String {
    format!(
        "{{ inner: {}{}{} }}",
        f,
        fail_opt_to_str(&f.cause()),
        backtrace_opt_to_str(&f.backtrace())
    )
}

impl<K> Display for Error<K>
where
    K: Copy + Clone + Eq + PartialEq + Debug + Fail,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{ inner: {}{}{} }}",
            self.inner,
            fail_opt_to_str(&self.cause()),
            backtrace_opt_to_str(&self.backtrace())
        )
    }
}

impl<K1, K2> From<Context<K1>> for Error<K2>
where
    K1: Into<K2> + Copy + Clone + Eq + PartialEq + Debug + Fail,
    K2: Copy + Clone + Eq + PartialEq + Debug + Fail,
{
    fn from(inner: Context<K1>) -> Error<K2> {
        let new_context = inner.get_context().clone().into();

        Error {
            inner: inner.context(new_context),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use failure::ResultExt;

    #[derive(Debug, Fail)]
    #[fail(display = "{{ msg: {} }}", msg)]
    struct FakeError {
        msg: &'static str,
    }

    impl FakeError {
        fn new(msg: &'static str) -> FakeError {
            FakeError { msg }
        }
    }

    #[cfg_attr(feature = "cargo-clippy", allow(empty_line_after_outer_attr))]
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
    enum FirstFakeErrorKind {
        #[fail(display = "FakeOneInner")]
        FakeOneInner,
    }

    #[cfg_attr(feature = "cargo-clippy", allow(empty_line_after_outer_attr))]
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
    enum SecondFakeErrorKind {
        #[fail(display = "FakeTwoInner")]
        FakeTwoInner,
    }

    #[cfg_attr(feature = "cargo-clippy", allow(empty_line_after_outer_attr))]
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
    enum MergedFakeErrorKind {
        #[fail(display = "FakeOneOuter")]
        FakeOneOuter,

        #[fail(display = "FakeTwoOuter")]
        FakeTwoOuter,
    }

    impl From<FirstFakeErrorKind> for MergedFakeErrorKind {
        fn from(_: FirstFakeErrorKind) -> MergedFakeErrorKind {
            MergedFakeErrorKind::FakeOneOuter
        }
    }

    impl From<SecondFakeErrorKind> for MergedFakeErrorKind {
        fn from(_: SecondFakeErrorKind) -> MergedFakeErrorKind {
            MergedFakeErrorKind::FakeTwoOuter
        }
    }

    #[test]
    fn test_from_context_for_error_fake_one() {
        let res = || -> std::result::Result<(), Error<MergedFakeErrorKind>> {
            Err(FakeError::new("Fake msg"))
                .context(FirstFakeErrorKind::FakeOneInner)?
        }();

        assert!(res.is_err());

        // can use `cargo test -- --nocapture` to see the result
        println!("{}", res.unwrap_err());
    }

    #[test]
    fn test_from_context_for_error_fake_two() {
        let res = || -> std::result::Result<(), Error<MergedFakeErrorKind>> {
            Err(FakeError::new("Fake msg"))
                .context(SecondFakeErrorKind::FakeTwoInner)?
        }();

        assert!(res.is_err());

        // can use `cargo test -- --nocapture` to see the result
        println!("{}", res.unwrap_err());
    }
}
