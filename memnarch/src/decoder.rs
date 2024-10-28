use std::fs;

use anyhow::{anyhow, Result};
use flate2::read::GzDecoder;
use tar::Archive;
use xz::read::XzDecoder;

pub fn decode(filename: &str) -> Result<String> {
    let file = fs::File::open(filename)?;

    if let Some(tarball) = filename.strip_suffix(".gz") {
        let gz = GzDecoder::new(file);
        let unzipped_name = tarball.strip_suffix(".tar").unwrap().to_string();

        return unzip_tar(gz).map(move |_| unzipped_name);
    } else if let Some(tarball) = filename.strip_suffix(".xz") {
        let xz = XzDecoder::new(file);
        let unzipped_name = tarball.strip_suffix(".tar").unwrap().to_string();

        return unzip_tar(xz).map(move |_| unzipped_name);
    }

    Err(anyhow!("Could not decode file {}", filename))
}

fn unzip_tar<T: std::io::Read>(stream: T) -> Result<()> {
    let mut archive = Archive::new(stream);

    archive.unpack(".")?;

    Ok(())
}
