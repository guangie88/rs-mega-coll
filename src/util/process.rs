use error::{Error, ErrorKind};
use error::custom::{CodeMsgError, MsgError};
use failure::{Context, Fail, ResultExt};
use std::fmt::Debug;
use std::io::Read;
use std::process::{Child, ChildStdout, Output};

pub fn extract_child_stdout<K>(child: Child) -> Result<ChildStdout, Error<K>>
where
    K: From<ErrorKind> + Copy + Clone + Eq + PartialEq + Debug + Fail,
{
    let (stdout, stderr) = (child.stdout, child.stderr);

    let stdout = stdout.ok_or_else(|| {
        let msg_err = stderr
            .ok_or_else(|| -> Error<K> {
                Context::new(ErrorKind::StderrEmpty).into()
            })
            .and_then(|mut bytes| -> Result<Error<K>, Error<K>> {
                let mut msg = String::new();

                bytes
                    .read_to_string(&mut msg)
                    .context(ErrorKind::StderrRead)?;

                Ok(MsgError::new(msg).context(ErrorKind::StderrValidMsg).into())
            });

        match msg_err {
            Ok(e) | Err(e) => e,
        }
    })?;

    Ok(stdout)
}

pub fn extract_output_stdout_str<K>(output: Output) -> Result<String, Error<K>>
where
    K: From<ErrorKind> + Copy + Clone + Eq + PartialEq + Debug + Fail,
{
    let output = if output.status.success() {
        String::from_utf8(output.stdout)
            .context(ErrorKind::StdoutUtf8Conversion)
    } else {
        let msg = String::from_utf8(output.stderr)
            .context(ErrorKind::StderrUtf8Conversion)?;

        Err(CodeMsgError::new(output.status.code(), msg))
            .context(ErrorKind::ChildOutput)
    }?;

    Ok(output)
}
