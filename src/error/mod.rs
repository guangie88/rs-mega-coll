use failure::{Backtrace, Context, Fail};
use std;
use std::fmt::{self, Display};

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
pub struct Error {
    pub inner: Context<ErrorKind>,
}

pub type Result<T> = std::result::Result<T, Error>;

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{ inner: {}, cause: {:?}, backtrace: {:?} }}",
            self.inner,
            self.cause(),
            self.backtrace()
        )
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }
}
