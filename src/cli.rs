use crate::config;
use clap::{builder::PossibleValue, Arg, ArgAction, ArgGroup, Command};
use const_format::formatcp;

pub fn build_parser() -> Command {
    Command::new(config::info::APP_NAME)
        .about(config::info::DESCRIPTION)
        .author("Daniele Monzani")
        .subcommand_required(true)

        /////////////////////
        // Load subcommand //
        /////////////////////
        .subcommand(
            Command::new("load")
                .about("Load a new colorscheme")
                // Colorscheme positional argument
                .arg(
                    Arg::new("colorscheme")
                        .help(formatcp!(
                            "Name of the colorscheme to load. Run `{} list` to list the available themes",
                            config::info::APP_NAME
                        ))
                        .required(true),
                )
        )

        ///////////////////////
        // Reload subcommand //
        ///////////////////////
        .subcommand(
            Command::new("reload")
                .about("Reload the latest colorscheme")
                // Blueprint option
                .arg(
                    Arg::new("blueprint")
                        .long("blueprint")
                        .short('b')
                        .help(
                            "The specific blueprint to reload. If not specified, all blueprints are reloaded",
                        )
                        .action(ArgAction::Append)
                        .required(false),
                )
                // No-script option
                .arg(
                    Arg::new("no-script")
                        .long("no-script")
                        // TODO: replace `chromasync-post.sh` with
                        // config::environ::POST_SCRIPT
                        .help("Prevent the execution of `chromasync-post.sh`")
                        .action(ArgAction::SetTrue)
                        .required(false),
                )
        )

        /////////////////////
        // List subcommand //
        /////////////////////
        .subcommand(
            Command::new("list")
                .about("List the available colorschemes")
                // Dark/light group
                .arg(
                    Arg::new("dark")
                        .long("dark")
                        .short('d')
                        .help("List colorschemes with a dark background color")
                        .action(ArgAction::SetTrue)
                        .required(false),
                )
                .arg(
                    Arg::new("light")
                        .long("light")
                        .short('l')
                        .help("List colorschemes with a light background color")
                        .action(ArgAction::SetTrue)
                        .required(false),
                )
                .group(ArgGroup::new("light-dark-group").args(&["dark", "light"]))
                // Sort order
                .arg(
                    Arg::new("sort-by")
                        .long("sort-by")
                        .help("Specify the sorting order")
                        .required(false)
                        .value_parser([
                            PossibleValue::new("name")
                                .aliases(["n"]),
                            PossibleValue::new("background-luminance")
                                .aliases(["background_luminance", "bg-lum", "bg_lum", "bglum", "background", "bg", "luminance", "lum", "background-brightness", "background_brightness", "brightness"]),
                            PossibleValue::new("contrast")
                                .aliases(["contr", "cont", "con", "cntr", "cnt"]),
                        ])
                ),
        )

        /////////////////////
        // Verbosity group //
        /////////////////////
        .arg(
            Arg::new("quiet")
                .long("quiet")
                .short('q')
                .help("Disable logs on the terminal")
                .action(ArgAction::SetTrue)
                .required(false),
        )
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .short('v')
                .help("Show more logs on the terminal")
                .action(ArgAction::SetTrue)
                .required(false),
        )
        .group(ArgGroup::new("verbosity-group").args(&["quiet", "verbose"]))
}

// Tests the cli
#[cfg(test)]
mod tests {
    use super::*;
    use clap::{error::Result, ArgMatches};

    // Helper function
    fn parse_args(args: &str) -> Result<ArgMatches> {
        // Prepends the app's name and splits by whitespace
        let args = format!("{} {}", config::info::APP_NAME, args);
        let args_vec = args.split_whitespace();

        // Builds the parser and parses the arguments
        let parser = build_parser();

        parser.try_get_matches_from(args_vec)
    }

    fn should_parse(args: &str, error_msg: &str) {
        let matches = parse_args(args);
        assert!(matches.is_ok(), "{}", error_msg);
    }

    fn should_parse_auto_err(args: &str) {
        let error_msg = &format!("Can't parse `{} {}`", config::info::APP_NAME, args).to_string();

        should_parse(args, error_msg)
    }

    fn should_fail_to_parse(args: &str, error_msg: &str) {
        let matches = parse_args(args);
        assert!(matches.is_err(), "{}", error_msg);
    }

    #[test]
    fn load() {
        should_parse_auto_err("load my-theme");
        should_fail_to_parse(
            "load",
            "Should fail cause positional argument `colorscheme` is missing, but got ok",
        );
    }

    #[test]
    fn reload() {
        should_parse_auto_err("reload");
    }

    #[test]
    fn reload_blueprint() {
        should_parse_auto_err("reload -b my-blueprint");
        should_parse_auto_err("reload --blueprint my-blueprint");
    }

    #[test]
    fn reload_no_script() {
        should_parse_auto_err("reload --no-script");
    }

    #[test]
    fn list() {
        should_parse_auto_err("list");
    }

    #[test]
    fn list_light_dark() {
        should_parse_auto_err("list -l");
        should_parse_auto_err("list --light");

        should_parse_auto_err("list -d");
        should_parse_auto_err("list --dark");

        should_fail_to_parse(
            "list -d -l",
            "Should fail to parse when both --light and --dark options are specified, but got ok",
        );
    }

    #[test]
    fn list_sort_by() {
        should_parse_auto_err("list --sort-by name");
        should_parse_auto_err("list --sort-by n");

        should_parse_auto_err("list --sort-by background-luminance");
        should_parse_auto_err("list --sort-by background_luminance");
        should_parse_auto_err("list --sort-by bg-lum");
        should_parse_auto_err("list --sort-by bg_lum");
        should_parse_auto_err("list --sort-by bglum");
        should_parse_auto_err("list --sort-by background");
        should_parse_auto_err("list --sort-by bg");
        should_parse_auto_err("list --sort-by luminance");
        should_parse_auto_err("list --sort-by lum");
        should_parse_auto_err("list --sort-by background-brightness");
        should_parse_auto_err("list --sort-by background_brightness");
        should_parse_auto_err("list --sort-by brightness");

        should_parse_auto_err("list --sort-by contrast");
        should_parse_auto_err("list --sort-by contr");
        should_parse_auto_err("list --sort-by cont");
        should_parse_auto_err("list --sort-by con");
        should_parse_auto_err("list --sort-by cntr");
        should_parse_auto_err("list --sort-by cnt");

        should_fail_to_parse(
            "list --sort-by name --sort-by luminance",
            "Should fail to parse when more --sort-by args are specified",
        );
    }

    #[test]
    fn verbosity_group() {
        should_parse_auto_err("-v list");
        should_parse_auto_err("--verbose list");

        should_parse_auto_err("-q list");
        should_parse_auto_err("--quiet list");

        should_fail_to_parse(
            "-v -q list",
            "Should fail to parse when both --verbose and --quiet options are specified, but got ok",
        );
    }
}
