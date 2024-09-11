use std::str::FromStr;

pub enum ProjectType {
    Workspace,
    Standalone,
}

impl ToString for ProjectType {
    fn to_string(&self) -> String {
        match self {
            Self::Standalone => "standalone".into(),
            Self::Workspace => "workspace".into(),
        }
    }
}

impl FromStr for ProjectType {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standalone" => Ok(ProjectType::Standalone),
            "workspace" => Ok(Self::Workspace),
            _ => Err(crate::Error::InvalidProjectKind { kind: s.into() }),
        }
    }
}
