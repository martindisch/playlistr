use eyre::{eyre, Result};
use percent_encoding::{AsciiSet, CONTROLS};
use std::{
    collections::VecDeque,
    fs::{self, File},
    io::{BufRead, BufReader},
    iter::FromIterator,
    path::Path,
};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b'[').add(b']');

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

/// Writes the combination of the given playlists to the working directory.
///
/// # Arguments
/// * `playlists` - The playlists to combine.
pub fn combine_playlists(playlists: &[impl AsRef<Path>]) -> Result<()> {
    let playlists = playlists
        .iter()
        .map(|p| BufReader::new(File::open(p)?).lines().collect())
        .collect::<Result<Vec<_>, _>>()?;

    let combined_playlist = combine_lists(&playlists)
        .iter()
        .map(|l| l.as_str())
        .collect::<Vec<_>>();
    let file_content = combined_playlist.join("\n");
    fs::write("combined.m3u8", file_content)?;

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
                p.to_str().ok_or_else(|| eyre!("Invalid unicode")).map(|l| {
                    percent_encoding::utf8_percent_encode(l, FRAGMENT)
                        .to_string()
                })
            })
        })
        .collect::<Result<Result<Vec<_>, _>, _>>()??;
    sorted_files.sort();

    let file_content = sorted_files.join("\n");
    fs::write(playlist, file_content)?;

    Ok(())
}

/// Evenly distributes elements from lists of possibly different sizes.
///
/// # Arguments
/// * `lists` - The lists to combine.
fn combine_lists<T>(lists: &[Vec<T>]) -> Vec<&T> {
    let total_size = lists.iter().fold(0, |acc, x| acc + x.len());
    // Since we want to pop elements from the front, we need VecDeques
    let mut lists = lists
        .iter()
        .map(|p| (VecDeque::from_iter(p), p.len()))
        .collect::<Vec<_>>();
    let mut combined_lists = Vec::with_capacity(total_size);

    for i in 1..=total_size {
        let progress = i as f32 / total_size as f32;

        for (list, original_size) in lists.iter_mut() {
            let target_distributed = *original_size as f32 * progress;
            let already_distributed = (*original_size - list.len()) as f32;

            if already_distributed < target_distributed {
                if let Some(element) = list.pop_front() {
                    combined_lists.push(element);
                }
            }
        }
    }

    combined_lists
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distribute_equally() {
        let playlists = vec![
            vec!["11", "12", "13", "14"],
            vec!["21", "22", "23", "24"],
            vec!["31", "32", "33", "34"],
        ];
        let owned = to_owned(&playlists);
        let combined = combine_lists(&owned);
        assert_eq!(
            [
                "11", "21", "31", "12", "22", "32", "13", "23", "33", "14",
                "24", "34",
            ],
            &combined[..]
        );
    }

    #[test]
    fn distribute_different_length_2() {
        let playlists = vec![vec!["11", "12", "13", "14"], vec!["21", "22"]];
        let owned = to_owned(&playlists);
        let combined = combine_lists(&owned);
        assert_eq!(["11", "21", "12", "13", "22", "14",], &combined[..]);
    }

    #[test]
    fn distribute_different_length_3() {
        let playlists = vec![
            vec!["11", "12"],
            vec!["21", "22", "23", "24"],
            vec!["31", "32"],
        ];
        let owned = to_owned(&playlists);
        let combined = combine_lists(&owned);
        assert_eq!(
            ["11", "21", "31", "22", "12", "23", "32", "24"],
            &combined[..]
        );
    }

    #[test]
    fn distribute_uneven() {
        let playlists =
            vec![vec!["11"], vec!["21", "22", "23"], vec!["31", "32"]];
        let owned = to_owned(&playlists);
        let combined = combine_lists(&owned);
        assert_eq!(["11", "21", "31", "22", "32", "23",], &combined[..]);
    }

    fn to_owned(outer: &[Vec<&str>]) -> Vec<Vec<String>> {
        outer
            .iter()
            .map(|inner| inner.iter().map(|s| s.to_string()).collect())
            .collect()
    }
}
