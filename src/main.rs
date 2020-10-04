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
                    Arg::with_name("DIRECTORIES")
                        .help("The directories where the files are located")
                        .required(true)
                        .multiple(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("combine")
                .about("Combines several playlists into one")
                .arg(
                    Arg::with_name("PLAYLISTS")
                        .help("The playlists to combine")
                        .required(true)
                        .multiple(true),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("create") {
        let directories = matches
            .values_of("DIRECTORIES")
            .map(|v| v.collect::<Vec<&str>>())
            .unwrap();

        playlistr::create_playlists(&directories)?;
    } else if let Some(matches) = matches.subcommand_matches("combine") {
        let playlists = matches
            .values_of("PLAYLISTS")
            .map(|v| v.collect::<Vec<&str>>())
            .unwrap();

        playlistr::combine_playlists(&playlists)?;
    }

    Ok(())
}
