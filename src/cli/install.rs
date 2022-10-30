use std::{ env, fs, path::Path};

use crate::{error, info, r#type::Type, success, Res, util::install_version};

pub async fn install(release_type: Type, verbose: bool) -> Res<()> {
  // create user var & create .dvm dirs
  let user = env::var("USER")?;
  fs::create_dir_all(format!("/home/{}/.dvm/bin", user))?;
  if verbose {
    info!("created .dvm dir")
  }

  let pascal_pkg = release_type.directory();

  let exists = Path::new(&format!("/home/{}/.dvm/{}", user, pascal_pkg)).exists();

  if exists {
    error!("{} is already installed", release_type);
  }

  let (latest, _) = install_version(false, release_type.clone(), verbose, user).await?;

  success!("installed {}:{}", release_type, latest);
  Ok(())
}
