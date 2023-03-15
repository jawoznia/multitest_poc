/// Trait to expose messages of the contract
pub trait Contract {
    type InstantiateMsg;
    type ExecMsg;
    type QueryMsg;
    type MigrationMsg;
}

/// Trait to expose multitest utils of the contract
pub trait Multitest<'app> {
    type CodeId;
    type Contract;

    fn store_code(app: &'app mut sylvia::multitest::App) -> Self::CodeId;
}
