use super::dto::update::Update;
use crate::{
    entity::{
        error::{DBResult, MetaDBError},
        git_commit::{CommitHash, RCGitCommit},
        git_head::{GitBranchID, GitHeadID, GitHeadSnapshot, GitRepoID},
        machine_id::MachineID,
    },
    repo::{git_commit_repo::GitCommitRepo, git_head_repo::GitHeadRepo},
};

pub struct MetaDB<GitCommitRepoImpl: GitCommitRepo, GitHeadRepoImpl: GitHeadRepo> {
    machine_id: MachineID,
    git_commit_repo: GitCommitRepoImpl,
    git_head_repo: GitHeadRepoImpl,
}

impl<GitCommitRepoImpl: GitCommitRepo, GitHeadRepoImpl: GitHeadRepo>
    MetaDB<GitCommitRepoImpl, GitHeadRepoImpl>
{
    /// Create a new instance of Main Service
    pub fn new(
        machine_id: MachineID,
        git_commit_repo: GitCommitRepoImpl,
        git_head_repo: GitHeadRepoImpl,
    ) -> Self {
        Self {
            machine_id,
            git_commit_repo,
            git_head_repo,
        }
    }

    /// Get GitHead from machine id, branch id and repo id
    pub fn get_git_head_from_id(
        &self,
        id: GitHeadID,
    ) -> DBResult<impl Future<Output = GitHeadSnapshot>> {
        self.git_head_repo.get(id)
    }

    /// Get GitHead of current machine from branch id and repo id
    pub fn get_local_git_head_from_branch_and_repo(
        &self,
        git_branch_id: GitBranchID,
        git_repo_id: GitRepoID,
    ) -> DBResult<impl Future<Output = GitHeadSnapshot>> {
        self.get_git_head_from_id(GitHeadID::new(
            self.machine_id.clone(),
            git_repo_id,
            git_branch_id,
        ))
    }

    /// Get GitHead(s) from machine id and repo id / all branches
    pub fn get_git_heads_from_machine_and_repo(
        &self,
        machine_id: MachineID,
        git_repo_id: GitRepoID,
    ) -> DBResult<impl Future<Output = Vec<GitHeadSnapshot>>> {
        self.git_head_repo
            .get_from_machine_and_repo(machine_id, git_repo_id)
    }

    /// Get GitHead(s) of current machine from machine id and repo id / all branches
    pub fn get_local_git_heads_from_machine_and_branch(
        &self,
        git_repo_id: GitRepoID,
    ) -> DBResult<impl Future<Output = Vec<GitHeadSnapshot>>> {
        self.get_git_heads_from_machine_and_repo(self.machine_id.clone(), git_repo_id)
    }

    /// Get GitHead(s) page
    pub fn get_local_git_heads_page(
        &self,
        from: i32,
        count: i64,
    ) -> DBResult<impl Future<Output = Vec<GitHeadSnapshot>>> {
        self.git_head_repo.get_page(from, count)
    }

    ///  Get GitHead(s) last event ID
    pub fn get_git_heads_last_event_sno(&self) -> DBResult<impl Future<Output = i32>> {
        self.git_head_repo.get_last_event_sno()
    }

    /// Get GitCommit
    pub fn get_git_commit(&self, id: CommitHash) -> DBResult<impl Future<Output = RCGitCommit>> {
        self.git_commit_repo.get(id)
    }

    /// Only means of updating the database
    pub fn update(&self, _update: Update) -> Result<(), MetaDBError> {
        todo!()
    }

    // TODO define functions for push and pull from remote
}
