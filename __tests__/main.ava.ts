import {BN, Workspace} from 'near-workspaces-ava';
import {v4 as uuid} from 'uuid'

const workspace = Workspace.init(async ({root}) => {
  // Create a subaccount of the root account, like `alice.sandbox`
  // (the actual account name is not guaranteed; you can get it with `alice.accountId`)
  const company1 = await root.createAccount('company1');
  const company2 = await root.createAccount('company2');
  const worker1 = await root.createAccount('worker1');
  const worker2 = await root.createAccount('worker2');
  // Create a subaccount of the root account, and also deploy a contract to it
  const contract = await root.createAndDeploy(
    // Subaccount name
    'near_project_management',

    // Relative path (from package.json location) to the compiled contract file
    // which will be deployed to this account
    './target/wasm32-unknown-unknown/release/near_project_management.wasm',

    // Provide `method` and `args` to call in the same transaction as the deploy
    {
      method: 'new',
      args: {owner_id: root},
    }
  );

  // Return the accounts that you want available in subsequent tests
  // (`root` is always available)
  return {company1, company2, worker1, worker2, contract};
});

workspace.test('Root can add project', async (test, {contract, root}) => {
  let id: String = uuid().toString()
  let args = {
    id,
    github_issue_link: "https://github.com/dummy-repo/issues/1",
    description: "this is a test"
  }
  let return_value: any = await root.call(contract, 'add_project', args, {attachedDeposit: new BN(2000)});
  test.is(return_value.Ok, id);
});

workspace.test("Different company's can add projects", async (test, {
    company1,
    company2,
    worker1,
    worker2,
    contract,
    root
}) => {
  let id: String = uuid().toString()
  let args = {
    id,
    github_issue_link: "https://github.com/dummy-repo/issues/1",
    description: "this is a test"
  }
  let return_value: any = await company1.call(contract, 'add_project', args, {attachedDeposit: new BN(2000)});
  test.is(return_value.Ok, id);
  id = uuid().toString();
  args = {
    id,
    github_issue_link: "https://github.com/dummy-repo/issues/1",
    description: "this is a test"
  }
  return_value = await company2.call(contract, 'add_project', args, {attachedDeposit: new BN(2000)});
  test.is(return_value.Ok, id);
})

workspace.test("Test get all projects", async (test, {
  company1,
  company2,
  worker1,
  worker2,
  contract,
  root
}) => {
  let id: String = uuid().toString()
  let args = {
    id,
    github_issue_link: "https://github.com/dummy-repo/issues/1",
    description: "this is a test"
  }
  let return_value: any = await company1.call(contract, 'add_project', args, {attachedDeposit: new BN(2000)});
  test.is(return_value.Ok, id);
  let expectedResponse = {
    id,
    github_issue_link: "https://github.com/dummy-repo/issues/1",
    description: "this is a test",
    reward: 2000,
    status: "Created",
    worker: null,
    project_owner: company1.accountId
  }
  let projects: any = await contract.view("get_all_projects");
  test.is(projects.created.length, 1)
  test.deepEqual(projects.created[0], expectedResponse)
  id = uuid().toString();
  args = {
    id,
    github_issue_link: "https://github.com/dummy-repo/issues/1",
    description: "this is a test"
  }
  return_value = await company2.call(contract, 'add_project', args, {attachedDeposit: new BN(2000)});
  test.is(return_value.Ok, id);
  projects  = await contract.view("get_all_projects");
  test.is(projects.created.length, 2)
  expectedResponse.id = id
  expectedResponse.project_owner = company2.accountId
  test.deepEqual(projects.created[1], expectedResponse)
})

workspace.test("Test that a user cannot edit another users project", async (test, {
  company1,
  company2,
  worker1,
  worker2,
  contract,
  root
}) => {
  let id: String = uuid().toString()
  let args = {
    id,
    github_issue_link: "https://github.com/dummy-repo/issues/1",
    description: "this is a test"
  }
  let return_id1: any = await company1.call(contract, 'add_project', args, {attachedDeposit: new BN(2000)});
  id = uuid().toString();
  args = {
    id,
    github_issue_link: "https://github.com/dummy-repo/issues/1",
    description: "this is a test"
  }
  let return_id2: any = await company2.call(contract, 'add_project', args, {attachedDeposit: new BN(2000)});
  args = {
    id: return_id1.Ok,
    description: "test",
    github_issue_link: "https://github.com/dummy-repo/issues/1"
  }
  let result: any = await company2.call(contract, 'update_project', args)
  test.is(result.Err, 'You can only edit projects you own.')
  args.id = return_id2.Ok
  result = await company1.call(contract, 'update_project', args)
  test.is(result.Err, 'You can only edit projects you own.')
  result = await company2.call(contract, 'update_project', args)
  test.is(result.Err, undefined)
})
