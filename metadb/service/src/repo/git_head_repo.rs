use crate::entity::{
    error::DBResult,
    git_head::{GitHeadID, GitHeadSnapshot, GitRepoID},
    machine_id::MachineID,
};

pub trait GitHeadRepo {
    fn get(&self, id: GitHeadID) -> DBResult<impl Future<Output = GitHeadSnapshot>>;

    fn get_from_machine_and_repo(
        &self,
        machine_id: MachineID,
        git_repo_id: GitRepoID,
    ) -> DBResult<impl Future<Output = Vec<GitHeadSnapshot>>>;

    fn get_page(
        &self,
        from: i32,
        count: i64,
    ) -> DBResult<impl Future<Output = Vec<GitHeadSnapshot>>>;

    fn get_last_event_sno(&self) -> DBResult<impl Future<Output = i32>>;
}
