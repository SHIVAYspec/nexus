use super::error::EntityParseError;
use super::git_head::GitHeadID;

#[derive(Clone, Debug)]
pub struct CommitHash(Vec<u8>);

impl From<CommitHash> for Vec<u8> {
    fn from(value: CommitHash) -> Self {
        value.0
    }
}

impl TryFrom<Vec<u8>> for CommitHash {
    type Error = EntityParseError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.len() == 32 {
            Ok(Self(value))
        } else {
            Err(EntityParseError::Msg(format!(
                "Vector length is not 32 (for representing 256 bits). It is {}.",
                value.len()
            )))
        }
    }
}

#[derive(Clone, Debug)]
pub enum GitCommitParent {
    Root,
    Normal(CommitHash),
    Merge(CommitHash, CommitHash),
}

#[derive(Clone, Debug)]
pub struct GitCommit {
    hash: CommitHash,
    parent: GitCommitParent,
}

impl GitCommit {
    pub fn new(hash: CommitHash, parent: GitCommitParent) -> Self {
        Self { hash, parent }
    }

    pub fn get_hash(&self) -> &CommitHash {
        &self.hash
    }

    pub fn get_parent(&self) -> &GitCommitParent {
        &self.parent
    }
}

#[derive(Clone, Debug)]
pub struct RCGitCommit {
    refereres: Vec<GitHeadID>,
    payload: GitCommit,
}

impl RCGitCommit {
    pub fn new(refereres: Vec<GitHeadID>, payload: GitCommit) -> Self {
        Self { refereres, payload }
    }

    pub fn get_references(&self) -> &Vec<GitHeadID> {
        &self.refereres
    }

    pub fn get_mut_references(&mut self) -> &mut Vec<GitHeadID> {
        &mut self.refereres
    }

    pub fn get_payload(&self) -> &GitCommit {
        &self.payload
    }
}
