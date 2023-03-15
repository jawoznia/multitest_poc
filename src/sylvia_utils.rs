/// Trait to expose messages of the contract
pub trait Contract {
    type InstantiateMsg;
    type ExecMsg;
    type QueryMsg;
    type MigrationMsg;
}

/// Trait to expose multitest utils of the contract
pub trait Multitest {
    type CodeId;
    type Contract;
}
