use std::{env, path::Path, path::PathBuf};

use tokio::fs::{self, DirEntry};

use crate::{Res, error, info, r#type::Type, success, util::install_version};

async fn dirent_verbose(dirent: DirEntry) -> Res<(String, Type, PathBuf)> {
  let rl_type = Type::from_dirname(dirent.file_name().to_str().unwrap()).unwrap_or(Type::STABLE);

  let path = dirent.path();
  let version = fs::read_to_string(path.join("version")).await?.replace("\n", "");

  Ok((version, rl_type, path))
}

pub async fn update_all(verbose: bool) -> Res<()> {
  // create user var & create .dvm dirs
  let user = env::var("USER")?;
  fs::create_dir_all(format!("/home/{}/.dvm/bin", user)).await?;
  if verbose {
    info!("created .dvm dir")
  }
  let mut dirs = fs::read_dir(format!("/home/{}/.dvm", user)).await?;
  let mut types = vec![];
  while let Ok(item) = dirs.next_entry().await {
    if let Some(dir) = item {
      if dir.file_name() == "bin" {
        continue;
      } else {
        types.push(dirent_verbose(dir).await?)
      }
    } else {
      break;
    }
  }
  for install in types {
    update(install.1, verbose).await?;
  }
  Ok(())
}

pub async fn update(release_type: Type, verbose: bool) -> Res<()> {
  // create user var & create .dvm dirs
  let user = env::var("USER")?;
  fs::create_dir_all(format!("/home/{}/.dvm/bin", user)).await?;
  if verbose {
    info!("created .dvm dir")
  }

  let pascal_pkg = release_type.directory();

  let exists = Path::new(&format!("/home/{}/.dvm/{}", user, pascal_pkg)).exists();

  if !exists {
    error!("{} is not installed", release_type);
  }

  let (latest, version) = install_version(true, release_type.clone(), verbose, user).await?;

  success!(
    "updated {}:{} -> {}:{}",
    release_type, version, release_type, latest
  );

  Ok(())
}
