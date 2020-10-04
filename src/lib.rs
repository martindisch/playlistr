use eyre::{eyre, Report, Result};
use std::{fs, io, path::Path};

pub fn create_playlist(
    directory: impl AsRef<Path>,
    playlist: impl AsRef<Path>,
) -> Result<()> {
    let mut sorted_files = fs::read_dir(directory)?
        .map(|res| res.map(|e| e.path()))
        .map(|res| {
            res.map(|p| {
                p.to_str()
                    .ok_or_else(|| eyre!("Invalid unicode"))
                    .map(|f| f.to_owned())
            })
        })
        .collect::<Result<Result<Vec<_>, Report>, io::Error>>()??;
    sorted_files.sort();

    let file_content = sorted_files.join("\n");
    fs::write(playlist, file_content)?;

    Ok(())
}
