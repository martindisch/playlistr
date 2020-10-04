use clap::{App, AppSettings, Arg, SubCommand};
use eyre::Result;

fn main() -> Result<()> {
    let matches = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about("A tiny tool for creating and combining playlists.")
        .author(clap::crate_authors!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("create")
                .about("Creates a playlist from files in a directory")
                .arg(
                    Arg::with_name("DIRECTORY")
                        .help("The directory where the files are located")
                        .required(true),
                )
                .arg(
                    Arg::with_name("PLAYLIST")
                        .help("The path to the output playlist file")
                        .required(true),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("create") {
        let directory = matches.value_of("DIRECTORY").unwrap();
        let playlist = matches.value_of("PLAYLIST").unwrap();

        playlistr::create_playlist(directory, playlist)?;
    }

    Ok(())
}
