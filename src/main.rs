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
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("create") {
        let directories = matches
            .values_of("DIRECTORIES")
            .map(|v| v.collect::<Vec<&str>>())
            .unwrap();

        playlistr::create_playlists(&directories)?;
    }

    Ok(())
}
