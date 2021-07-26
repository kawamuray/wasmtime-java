use crate::errors::Result;
use cap_std::fs::Dir;
use std::fs::File;
use std::path::Path;

pub fn open_dir<P: AsRef<Path>>(path: P) -> Result<Dir> {
    Ok(unsafe { Dir::open_ambient_dir(path)? })
}

pub fn open_wasi_dir<P: AsRef<Path>>(path: P) -> Result<wasi_cap_std_sync::dir::Dir> {
    Ok(wasi_cap_std_sync::dir::Dir::from_cap_std(open_dir(path)?))
}

pub fn open_wasi_file<P: AsRef<Path>>(path: P) -> Result<wasi_cap_std_sync::file::File> {
    let file = File::create(path)?;
    let file = unsafe { cap_std::fs::File::from_std(file) };
    Ok(wasi_cap_std_sync::file::File::from_cap_std(file))
}
