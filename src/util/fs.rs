use error::{ErrorKind, Result};
use error::custom::PathError;
use failure::ResultExt;
use fs2::FileExt;
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::path::Path;

pub fn lock_file<P: AsRef<Path>>(file_path: P) -> Result<File> {
    let file_path = file_path.as_ref();

    let flock = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_path)
        .map_err(|e| PathError::new(file_path, e))
        .context(ErrorKind::LockFileOpen)?;

    flock
        .try_lock_exclusive()
        .map_err(|e| PathError::new(file_path, e))
        .context(ErrorKind::LockFileExclusiveLock)?;

    Ok(flock)
}

pub fn read_from_file<P: AsRef<Path>>(p: P) -> Result<String> {
    let mut buf = String::new();
    let p = p.as_ref();

    let mut file = File::open(p)
        .map_err(|e| PathError::new(p.to_string_lossy().to_string(), e))
        .context(ErrorKind::FileIo)?;

    file.read_to_string(&mut buf)
        .context(ErrorKind::FileIo)?;
    Ok(buf)
}
