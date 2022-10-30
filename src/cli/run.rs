use std::{env, fs, path::Path};

use tokio::process::Command;

use crate::{Res, error, info, r#type::Type};

pub async fn run(release_type: Option<Type>, args: Vec<String>, verbose: bool) -> Res<()> {
  // create user var & create .dvm dirs
  let user = env::var("USER")?;
  fs::create_dir_all(format!("/home/{}/.dvm/bin", user))?;

  // create user var & create .dvm dirs
  let user = env::var("USER")?;
  fs::create_dir_all(format!("/home/{}/.dvm/bin", user))?;
  if verbose {
    info!("created .dvm dir")
  }

  let release_type = release_type.unwrap_or(Type::STABLE);

  let install_dir = release_type.directory();

  let exists = Path::new(&format!("/home/{}/.dvm/{}", user, install_dir)).exists();

  if !exists {
    error!("{} is not installed", release_type);
  }

  Command::new(format!("/home/{}/.dvm/{}/{}", user, install_dir, install_dir))
    .args(&args)
    .spawn()?
    .wait_with_output().await?;

  Ok(())
}
