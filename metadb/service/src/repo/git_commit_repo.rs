use crate::entity::{
    error::DBResult,
    git_commit::{CommitHash, RCGitCommit},
};

pub trait GitCommitRepo {
    fn get(&self, id: CommitHash) -> DBResult<impl Future<Output = RCGitCommit>>;
    fn delete(&self, id: CommitHash) -> DBResult<impl Future<Output = bool>>;
}
