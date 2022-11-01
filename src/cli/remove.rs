use std::{env, path::Path};

use tokio::fs;

use crate::{error, info, r#type::Type, success, Res};

pub async fn remove(release_type: Type, verbose: bool) -> Res<()> {
  // create user var & create .dvm dirs
  let user = env::var("USER")?;
  fs::create_dir_all(format!("/home/{}/.dvm/bin", user)).await?;

  let pascal_pkg = release_type.directory();

  let pkg_name = release_type.pkg_name();

  let exists = Path::new(&format!("/home/{}/.dvm/{}", user, pascal_pkg)).exists();
  if verbose {
    info!("checking if installation exists")
  }

  if !exists {
    error!("{} not installed", release_type);
  }

  let version = fs::read_to_string(format!("/home/{}/.dvm/{}/version", user, pascal_pkg)).await
    .expect("could not read version file: malformed installation detected");
  if verbose {
    info!("reading version file")
  }

  info!("removing version {}:{}", release_type, version);

  // remove all {release type} associated files
  fs::remove_dir_all(format!("/home/{}/.dvm/{}", user, pascal_pkg)).await
    .expect("error when removing data dirs");
  if verbose {
    info!("removed data dirs")
  }

  fs::remove_file(format!("/home/{}/.dvm/bin/{}", user, pkg_name)).await
    .expect("error when removing bin file");
  if verbose {
    info!("removed bin file")
  }

  fs::remove_file(format!(
    "/home/{}/.local/share/applications/{}.desktop",
    user, pkg_name
  )).await
  .expect("error when removing desktop file");
  if verbose {
    info!("removed desktop file")
  }

  fs::remove_file(format!(
    "/home/{}/.local/share/icons/{}.png",
    user, pkg_name
  )).await
  .expect("error when removing icon");
  if verbose {
    info!("removed icon")
  }

  success!("removed version {}:{}", release_type, version);
  Ok(())
}
