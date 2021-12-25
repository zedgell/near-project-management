use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::{AccountId, Promise, env};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen};
use near_sdk::PanicOnDefault;
use crate::structs::all_projects_return::AllProjectsReturn;
use crate::structs::project::{Project, Status};
use crate::structs::user_project_returns::UserProjectsReturn;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct ProjectManagement {
    // The reason for user_projects and user_ids is to reduce the loop time when getting/removing user_projects
    pub(crate) project_ids: UnorderedSet<String>,
    pub(crate) projects: LookupMap<String, Project>,
    // The reason for user_projects and user_ids is to reduce the loop time when getting/removing user_projects
    pub(crate) user_ids: UnorderedSet<String>,
    pub(crate) user_projects: LookupMap<AccountId, LookupMap<String, Project>>
}

#[near_bindgen]
impl ProjectManagement {
    #[init]
    pub fn new() -> Self {
        Self {
            project_ids: UnorderedSet::new(b"i"),
            projects: LookupMap::new(b"p"),
            user_ids: UnorderedSet::new(b"s"),
            user_projects: LookupMap::new(b"u")
        }
    }

    pub fn add_project(
        &mut self,
        id: String,
        github_issue_link: String,
        description: String,
        reward: String
    ) -> Result<String, String> {
        if env::current_account_id() != env::signer_account_id() {
            Err("Only the company can create a project.".to_string())
        } else {
            self.projects.insert(&id, &Project::new(
                id.clone(),
                github_issue_link,
                description,
                reward.parse().unwrap()
            ));
            self.project_ids.insert(&id);
            Ok(id)
        }
    }

    pub fn update_project(
        &mut self,
        id: String,
        github_issue_link: Option<String>,
        description: Option<String>,
        reward: Option<String>
    ) -> Result<String, String> {
        if env::current_account_id() != env::signer_account_id() {
            Err("Only the company can create a project.".to_string())
        } else {
            let mut project = self.projects.get(&id).unwrap();
            match github_issue_link {
                None => (),
                Some(link) => {
                    project.github_issue_link = link;
                }
            }
            match description {
                None => (),
                Some(desc) => {
                    project.description = desc;
                }
            }
            match reward {
                None => (),
                Some(reward_string) => {
                    project.reward = reward_string.parse().unwrap();
                }
            }
            self.projects.remove(&id);
            self.projects.insert(&id, &project);
            Ok(id)
        }
    }

    pub fn set_user_for_project(&mut self, id: String, worker_id: String) -> Result<(), String> {
        let mut project = self.projects.get(&id).unwrap();
        if project.worker.is_some() {
            Err("The project done has a user attached to it. Please remove the user before preceding.".to_string())
        } else {
            project.worker = Option::from(worker_id.clone());
            project.status = Status::InProgress;
            self.projects.remove(&id);
            self.projects.insert(&id, &project);
            let user_projects_result = self.user_projects.get(&worker_id);
            let user_projects = match user_projects_result {
                None => {
                    let mut set: LookupMap<String, Project> = LookupMap::new(b"p");
                    set.insert(&id, &self.projects.get(&id).unwrap());
                    set
                }
                Some(mut set) => {
                    set.insert(&id, &self.projects.get(&id).unwrap());
                    set
                }
            };
            self.user_ids.insert(&id);
            self.user_projects.insert(&worker_id, &user_projects);
            Ok(())
        }
    }

    pub fn set_project_complete(&mut self, id: String) -> Promise {
        let mut project = self.projects.get(&id).unwrap();
        assert_ne!(project.status, Status::Complete);
        let worker_id = project.worker.as_ref().unwrap();
        project.status = Status::Complete;
        self.projects.remove(&id);
        self.projects.insert(&id, &project);
        self.user_projects.get(&worker_id).unwrap().get(&id).unwrap().status = Status::Complete;
        Promise::new(worker_id.clone()).transfer(self.projects.get(&id).unwrap().reward)

    }

    pub fn get_all_projects(&self) -> AllProjectsReturn {
        let mut projects = AllProjectsReturn::new();
        for id in self.project_ids.to_vec() {
            let project = self.projects.get(&id).unwrap();
            match project.status {
                Status::Complete => {
                    projects.complete.push(project)
                }
                Status::InProgress => {
                    projects.in_progress.push(project);
                }
                Status::Created => {
                    projects.not_started.push(project);
                }
            }
        }
        projects
    }

    pub fn get_user_projects(&self, worker_id: String) -> UserProjectsReturn {
        let mut projects = UserProjectsReturn::new();
        for id in self.user_ids.to_vec() {
            let result = self.user_projects.get(&worker_id).unwrap().get(&id);
            match result {
                None => {}
                Some(project) => {
                    match project.status {
                        Status::Complete => {
                            projects.complete.push(project);
                        }
                        Status::InProgress => {
                            projects.in_progress.push(project);
                        }
                        Status::Created => {}
                    }
                }
            }
        }
        projects
    }

    pub fn remove_user_from_project(&mut self, job_id: String, worker_id: String) -> Result<(), String> {
        let mut project = self.projects.get(&job_id).unwrap();
        match project.status {
            Status::Complete => {
                Err("The job is marked as complete and therefore you cannot remove user from job.".to_string())
            }
            Status::InProgress => {
                // check if the worker or the the company requested it
                if project.worker.unwrap() == env::signer_account_id() || env::current_account_id() == env::signer_account_id() {
                    project.worker = None;
                    project.status = Status::Created;
                    self.user_projects.get(&worker_id).unwrap().remove(&job_id).unwrap();
                    self.user_ids.remove(&job_id);
                    self.projects.remove(&job_id).unwrap();
                    self.projects.insert(&job_id, &project).unwrap();
                    Ok(())
                } else {
                    Err("The job owner or the company must request it to be removed.".to_string())
                }
            }
            Status::Created => {
                Err("The job has no user assigned to it.".to_string())
            }
        }
    }

    pub fn remove_project(&mut self, job_id: String) -> Result<(), String> {
        let project = self.projects.get(&job_id).unwrap();
        match project.status {
            Status::Complete => {
                Err("The job is complete and therefore cannot be removed.".to_string())
            }
            Status::InProgress => {
                Err("The job is in progress. You must first remove the user from job.".to_string())
            }
            Status::Created => {
                // check if the company requested it
                if env::current_account_id() == env::signer_account_id() {
                    self.projects.remove(&job_id).unwrap();
                    self.project_ids.remove(&job_id);
                    Ok(())
                } else {
                    Err("The company must request this.".to_string())
                }
            }
        }
    }
}