use std::{collections::HashMap};

use tokio::process::Command;
use tokio::fs;

use crate::{Res, error, info, r#type::Type};

async fn get_version(user: &str, pascal_pkg: &str) -> String {
  fs::read_to_string(format!("/home/{}/.dvm/{}/version", user, pascal_pkg)).await
    .expect("could not read version file: malformed installation detected")
    .replace("\n", "")

}

pub async fn install_version(update: bool, release_type: Type, verbose: bool, user: String) -> Res<(String, String)> {
  let pkg_name = release_type.pkg_name();

  let pascal_pkg = release_type.directory();

  let dl_sub = release_type.dl_sub();

  // request api for latest version
  let res = reqwest::get(release_type.updates_url())
  .await?
  .json::<HashMap<String, String>>()
  .await?;
  if verbose {
    info!("requested api for latest version")
  }

  // exit if the api doesn't return a name (latest version)
  let latest = match res.get("name") {
    Some(v) => v,
    None => std::process::exit(1),
  };
  info!("found latest version {}:{}", release_type, latest);

  let mut version = String::new();
  if update {
    version = get_version(&user, pascal_pkg).await;
    // check if the version is the same & clean file of \n's
    if verbose {
      info!("checking if existing version and latest match")
    }

    if version.eq(latest) {
      error!(
        "you already have the latest version of {}",
        release_type
      );
    }

    // remove installed to make room for upgrade
    fs::remove_dir_all(format!("/home/{}/.dvm/{}", user, pascal_pkg)).await?;
    info!("removing old components");
  }

  // download tarball
  let tar_name = format!("{}-{}", pkg_name, latest);
  info!("downloading version {}:{}", release_type, latest);

  let tar_bytes = reqwest::get(format!(
    "https://{}.discordapp.net/apps/linux/{}/{}.tar.gz",
    dl_sub, latest, tar_name
  ))
  .await?
  .bytes()
  .await?;
  if verbose {
    info!("downloaded tarball")
  }

  // write tar to /tmp
  let tmp_file = format!("/tmp/{}.tar.gz", tar_name);
  fs::write(&tmp_file, tar_bytes).await?;
  if verbose {
    info!("wrote tar to /tmp")
  }

  // extract tar to .dvm
  Command::new("tar")
    .arg("xf")
    .arg(&tmp_file)
    .arg("-C")
    .arg(format!("/home/{}/.dvm/", user))
    .spawn()?
    .wait()
    .await?;
  info!(
    "extracting components to {}",
    format!("/home/{}/.dvm/{}", user, pascal_pkg)
  );

  // change Exec= to dvm path from the desktop file
  Command::new("sed")
    .arg("-i")
    .arg(format!(
      "s#/usr/share/{}/{}#/home/{}/.dvm/bin/{}#",
      pkg_name, pascal_pkg, user, pkg_name
    ))
    .arg(format!(
      "/home/{}/.dvm/{}/{}.desktop",
      user, pascal_pkg, pkg_name
    ))
    .spawn()?
    .wait()
    .await?;
  if verbose {
    info!("changing bin locations in desktop entries")
  }

  // write a shell script to .dvm/bin to run discord
  let bin_path = format!("/home/{}/.dvm/bin/{}", user, pkg_name);
  fs::write(
    &bin_path,
    format!(
      r#"#!/usr/bin/env bash

USER_FLAGS_FILE="$HOME/.dvm/{}-flags.conf"
if [[ -f $USER_FLAGS_FILE ]]; then
  USER_FLAGS="$(cat $USER_FLAGS_FILE | sed 's/#.*//')"
fi

exec /home/{}/.dvm/{}/{} "$@" $USER_FLAGS
"#,
      pkg_name, user, pascal_pkg, pascal_pkg
    ),
  ).await?;

  if verbose {
    info!("created executable bin")
  }

  // make bin executable
  Command::new("chmod")
    .arg("+x")
    .arg(bin_path)
    .spawn()?
    .wait()
    .await?;
  if verbose {
    info!("allowed execution for bin")
  }

  // copy desktop file to .local/share/applications
  Command::new("install")
    .arg("-Dm644")
    .arg(format!(
      "/home/{}/.dvm/{}/{}.desktop",
      user, pascal_pkg, pkg_name
    ))
    .arg(format!("/home/{}/.local/share/applications", user))
    .spawn()?
    .wait()
    .await?;
  info!("installing desktop file");

  // copy icon to .local/share/icons
  fs::create_dir_all(format!("/home/{}/.local/share/icons", user)).await?;
  Command::new("cp")
    .arg(format!("/home/{}/.dvm/{}/discord.png", user, pascal_pkg))
    .arg(format!(
      "/home/{}/.local/share/icons/{}.png",
      user, pkg_name
    ))
    .spawn()?
    .wait()
    .await?;
  info!("installing icons");

  // create a file that contains the current version to use for updating
  fs::write(
    format!("/home/{}/.dvm/{}/version", user, pascal_pkg),
    latest,
  ).await?;
  if verbose {
    info!("created version file")
  }

  // remove tmp tar ball
  fs::remove_file(tmp_file).await?;
  if verbose {
    info!("remove tmp tar ball")
  }

  Ok((latest.to_string(), version))
}