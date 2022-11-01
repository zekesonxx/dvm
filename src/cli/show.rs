use std::{
  collections::HashMap,
  env,
  path::PathBuf,
};

use tokio::fs::{self, DirEntry};

use colored::*;

use crate::{r#type::Type, Res};

// fn dirent(dirent: DirEntry) -> Res<(String, Type)> {
//   let rl_type = match dirent.file_name().to_str().unwrap() {
//     "Discord" => Type::STABLE,
//     "DiscordCanary" => Type::CANARY,
//     "DiscordPTB" => Type::PTB,
//     "DiscordDevelopment" => Type::DEVELOPMENT,
//     _ => Type::STABLE,
//   };
//   let version = fs::read_to_string(dirent.path().join("version"))?.replace("\n", "");

//   Ok((version, rl_type))
// }

async fn dirent_verbose(dirent: DirEntry) -> Res<(String, Type, PathBuf)> {
  let rl_type = Type::from_dirname(dirent.file_name().to_str().unwrap()).unwrap_or(Type::STABLE);

  let path = dirent.path();
  let version = fs::read_to_string(path.join("version")).await?.replace("\n", "");

  Ok((version, rl_type, path))
}

async fn needs_update(version: String, release_type: Type) -> Res<(bool, String)> {
  let res = reqwest::get(format!(
    "https://discordapp.com/api/v8/updates/{}?platform=linux",
    release_type
  ))
  .await?
  .json::<HashMap<String, String>>()
  .await?;

  let latest = match res.get("name") {
    Some(v) => v,
    None => std::process::exit(1),
  };

  Ok((version.ne(latest), latest.to_string()))
}

pub async fn show(verbose: bool, check: bool) -> Res<()> {
  // create user var & create .dvm dirs
  let user = env::var("USER")?;
  fs::create_dir_all(format!("/home/{}/.dvm/bin", user)).await?;

  let mut dirs = fs::read_dir(format!("/home/{}/.dvm", user)).await?;
  let mut types = vec![]; //Vec::new::<(String, Type, PathBuf)>();
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
  
  let mut types = types.iter();
  while let Some(x) = types.next() {
    if check {
      let ver = needs_update(x.0.clone(), x.1.clone()).await?;
      if verbose {
        println!(
          "{}:{} -> {}",
          x.1
            .to_string()
            .color(if ver.0 { "red" } else { "green" })
            .bold(),
          x.0.bold().white(),
          x.2.display()
        );
      } else {
        println!(
          "{}:{}",
          x.1
            .to_string()
            .color(if ver.0 { "red" } else { "green" })
            .bold(),
          x.0.bold().white()
        );
      }
    } else {
      if verbose {
        println!(
          "{}:{} -> {}",
          x.1.to_string().white().bold(),
          x.0.bold().white(),
          x.2.display()
        );
      } else {
        println!("{}:{}", x.1.to_string().white().bold(), x.0.bold().white());
      }
    }
  }

  Ok(())
}
