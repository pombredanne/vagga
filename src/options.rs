use argparse::{ArgumentParser, Collect, StoreFalse, StoreOption};
use super::env::Environ;


pub fn env_options(env: &mut Environ, ap: &mut ArgumentParser) {
    ap.refer(&mut env.variables)
      .add_option(&["-v", "--variant"], box Collect::<String>,
            "Use variant where KEY equals VALUE (repeatable)")
      .metavar("KEY=VALUE");
    ap.refer(&mut env.settings.version_check)
      .add_option(&["--no-check"], box StoreFalse,
            "Do not check if container is up to date when running command");
    ap.refer(&mut env.set_env)
      .add_option(&["-E", "--env", "--environ"], box Collect::<String>,
            "Set environment variable for running command")
      .metavar("NAME=VALUE");
    ap.refer(&mut env.propagate_env)
      .add_option(&["-e", "--use-env"], box Collect::<String>,
            "Propagate variable VAR into command environment")
      .metavar("VAR");
    ap.refer(&mut env.container)
      .add_option(&["-C", "--force-container"], box StoreOption::<String>,
            "Use container NAME for the following command")
      .metavar("NAME");
}
