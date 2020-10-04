use eyre::{eyre, Report, Result};
use std::{fs, io, path::Path};

/// Writes playlists for the given directories to the working directory.
///
/// # Arguments
/// * `directories` - The directories to scan and build playlists for.
pub fn create_playlists(directories: &[impl AsRef<Path>]) -> Result<()> {
    for directory in directories {
        let directory_name = directory
            .as_ref()
            .file_name()
            .ok_or_else(|| eyre!("Abnormal file"))?
            .to_str()
            .ok_or_else(|| eyre!("Invalid unicode"))?;
        let playlist = format!("{}.m3u8", directory_name);

        create_playlist(directory, playlist)?;
    }

    Ok(())
}

/// Creates a playlist for a single directory.
///
/// # Arguments
/// * `directory` - The directory to scan and build the playlist for.
/// * `playlist` - The file to write to.
fn create_playlist(
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
