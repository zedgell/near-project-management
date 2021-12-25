

pub mod structs {
    pub mod project;
    pub mod project_management;
    pub mod all_projects_return;
    pub mod user_project_returns;
}

#[cfg(test)]
mod tests {
    use near_sdk::{AccountId, MockedBlockchain};
    use near_sdk::{testing_env, VMContext};
    use crate::structs::project_management::ProjectManagement;

    fn get_context(input: Vec<u8>, is_view: bool, signer: String) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: signer,
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 200000000,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    // This test should insert a new project with no errors
    fn test_get_all_projects() {
        // set up the mock context into the testing environment
        let context = get_context(vec![], false, "alice.testnet".to_string());
        testing_env!(context.clone());
        // instantiate a contract
        let mut contract = ProjectManagement::new();
        for i in 1..5 {
            let result = contract.add_project(i.to_string(),
                                 "https://github.com/test-project/issues/1".to_string(),
                                 "This is a test".to_string(),
                                 "2000".parse().unwrap());
            assert_eq!(result.is_ok(), true);
            let projects = contract.get_all_projects();
            assert_eq!(projects.not_started.len(), i as usize);
            assert_eq!(projects.in_progress.len(), 0 as usize);
            assert_eq!(projects.complete.len(), 0 as usize);
        }
        for i in 1..3 {
            let result = contract.set_user_for_project(i.to_string(), context.clone().current_account_id);
            assert_eq!(result.is_ok(), true);
            let projects = contract.get_all_projects();
            assert_eq!(projects.not_started.len(), 4 - i as usize);
            assert_eq!(projects.in_progress.len(), i as usize);
            assert_eq!(projects.complete.len(), 0 as usize);
        }
        for i in 1..3 {
            contract.set_project_complete(i.to_string());
            let projects = contract.get_all_projects();
            assert_eq!(projects.not_started.len(), 2 as usize);
            assert_eq!(projects.in_progress.len(), 2 - i as usize);
            assert_eq!(projects.complete.len(), i as usize);
        }
    }

    #[test]
    fn test_get_user_projects() {
        // set up the mock context into the testing environment
        let context = get_context(vec![], false, "alice.testnet".to_string());
        testing_env!(context.clone());
        // instantiate a contract
        let mut contract = ProjectManagement::new();
        for i in 1..5 {
            let result = contract.add_project(i.to_string(),
                                 "https://github.com/test-project/issues/1".to_string(),
                                 "This is a test".to_string(),
                                 "2000".parse().unwrap());
            assert_eq!(result.is_ok(), true);
            let projects = contract.get_user_projects(context.clone().current_account_id);
            assert_eq!(projects.in_progress.len(), 0 as usize);
            assert_eq!(projects.complete.len(), 0 as usize);
        }
        for i in 1..3 {
            let result = contract.set_user_for_project(i.to_string(), context.clone().current_account_id);
            assert_eq!(result.is_ok(), true);
            let projects = contract.get_user_projects(context.clone().current_account_id);
            assert_eq!(projects.in_progress.len(), i as usize);
            assert_eq!(projects.complete.len(), 0 as usize);
        }
        for i in 1..3 {
            contract.set_project_complete(i.to_string());
            let projects = contract.get_user_projects(context.clone().current_account_id);
            assert_eq!(projects.in_progress.len(), 2 - i as usize);
            assert_eq!(projects.complete.len(), i as usize);
        }
    }

    #[test]
    // This test should insert a new project with no errors
    fn test_create_project() {
        // set up the mock context into the testing environment
        let context = get_context(vec![], false, "alice.testnet".to_string());
        testing_env!(context.clone());
        // instantiate a contract
        let mut contract = ProjectManagement::new();
        let result = contract.add_project("1".parse().unwrap(),
                                      "https://github.com/test-project/issues/1".to_string(),
                                      "This is a test".to_string(),
                                      "2000".parse().unwrap());
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    // This test should add a user to a project with no errors
    fn test_add_user_to_project() {
        // set up the mock context into the testing environment
        let context = get_context(vec![], false, "alice.testnet".to_string());
        testing_env!(context.clone());
        // instantiate a contract
        let worker_account = AccountId::from("subaccount.example.near");
        let mut contract = ProjectManagement::new();
        let id = contract.add_project("1".parse().unwrap(),
                                      "https://github.com/test-project/issues/1".to_string(),
                                      "This is a test".to_string(),
                                      "100".parse().unwrap());
        assert_eq!(id.is_ok(), true);
        let result = contract.set_user_for_project(id.clone().unwrap(), worker_account.clone());
        assert_eq!(result.is_ok(), true);
        // This second part test that it can add to a existing set
        let id = contract.add_project("2".parse().unwrap(),
                                      "https://github.com/test-project/issues/2".to_string(),
                                      "This is a test 2".to_string(),
                                      "2000".parse().unwrap());
        assert_eq!(id.is_ok(), true);
        let result = contract.set_user_for_project(id.clone().unwrap(), worker_account.clone());
        assert_eq!(result.is_ok(), true);
        contract.set_project_complete(id.clone().unwrap());
    }
}
