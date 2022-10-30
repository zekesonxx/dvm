use std::fmt;

#[derive(Debug, Clone)]
pub enum Type {
  STABLE,
  PTB,
  CANARY,
  DEVELOPMENT,
}

impl fmt::Display for Type {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.slug())
  }
}

impl Type {
  /// Return the correct directory name for the particular version
  /// ex, "Discord" for stable or "DiscordCanary" for canary
  pub fn directory(&self) -> &str {
    match self {
      Type::STABLE => "Discord",
      Type::PTB => "DiscordPTB",
      Type::CANARY => "DiscordCanary",
      Type::DEVELOPMENT => "DiscordDevelopment",
    }
  }

  /// Return the short slug form of the particular version
  /// ex, "stable" for stable
  pub fn slug(&self) -> &str {
    match self {
      Type::STABLE => "stable",
      Type::PTB => "ptb",
      Type::CANARY => "canary",
      Type::DEVELOPMENT => "development",
    }
  }

  /// Return the pkg name of the particular version
  /// ex, "stable" for stable
  pub fn pkg_name(&self) -> &str {
    match self {
      Type::STABLE => "discord",
      Type::PTB => "discord-ptb",
      Type::CANARY => "discord-canary",
      Type::DEVELOPMENT => "discord-development",
    }
  }

  /// Return the download link suffix for the particular version
  /// ex, "dl" for stable or "dl-canary" for canary
  pub fn dl_sub(&self) -> &str {
    match self {
      Type::STABLE => "dl",
      Type::PTB => "dl-ptb",
      Type::CANARY => "dl-canary",
      Type::DEVELOPMENT => "dl-development",
    }
  }

  /// Return the URL to check the current version for the given version
  pub fn updates_url(&self) -> String {
    format!("https://discordapp.com/api/v8/updates/{}?platform=linux", self.slug())
  }

  pub fn from_dirname<S: AsRef<str>>(dirname: S) -> Option<Self> {
    match dirname.as_ref() {
      "Discord" => Some(Type::STABLE),
      "DiscordCanary" => Some(Type::CANARY),
      "DiscordPTB" => Some(Type::PTB),
      "DiscordDevelopment" => Some(Type::DEVELOPMENT),
      _ => None,
    }
  }
}