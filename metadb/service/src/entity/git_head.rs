use super::git_commit::CommitHash;
use super::machine_id::MachineID;

#[derive(Clone, Debug)]
pub struct GitRepoID(u128);

impl GitRepoID {
    pub fn new() -> Self {
        Self(uuid::Uuid::now_v7().as_u128())
    }
}

impl From<u128> for GitRepoID {
    fn from(value: u128) -> Self {
        Self(value)
    }
}

impl From<GitRepoID> for u128 {
    fn from(value: GitRepoID) -> Self {
        value.0
    }
}

#[derive(Clone, Debug)]
pub struct GitBranchID(String);

impl From<String> for GitBranchID {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<GitBranchID> for String {
    fn from(value: GitBranchID) -> Self {
        value.0
    }
}

#[derive(Clone, Debug)]
pub struct GitHeadID {
    machine_id: MachineID,
    repo_id: GitRepoID,
    branch_id: GitBranchID,
}

impl GitHeadID {
    pub fn new(machine_id: MachineID, repo_id: GitRepoID, branch_id: GitBranchID) -> Self {
        Self {
            machine_id,
            repo_id,
            branch_id,
        }
    }

    pub fn get_machine_id(&self) -> &MachineID {
        &self.machine_id
    }

    pub fn get_repo_id(&self) -> &GitRepoID {
        &self.repo_id
    }

    pub fn get_branch_id(&self) -> &GitBranchID {
        &self.branch_id
    }
}

#[derive(Clone, Debug)]
pub struct GitHead {
    id: GitHeadID,
    hash: CommitHash,
}

impl GitHead {
    pub fn new(id: GitHeadID, hash: CommitHash) -> Self {
        Self { id, hash }
    }

    pub fn get_id(&self) -> &GitHeadID {
        &self.id
    }

    pub fn get_hash(&self) -> &CommitHash {
        &self.hash
    }
}

#[derive(Clone, Debug)]
pub struct GitHeadSnapshot {
    version: u128,
    event_id: Option<u128>,
    doc: GitHead,
}

impl GitHeadSnapshot {
    pub fn new(doc: GitHead) -> Self {
        Self {
            version: 0,
            event_id: None,
            doc,
        }
    }

    pub fn existing(version: u128, event_id: u128, doc: GitHead) -> Self {
        Self {
            version,
            event_id: Some(event_id),
            doc,
        }
    }

    pub fn get_version(&self) -> u128 {
        self.version
    }

    pub fn get_event_id(&self) -> Option<u128> {
        self.event_id
    }

    pub fn get_doc(&self) -> &GitHead {
        &self.doc
    }
}
