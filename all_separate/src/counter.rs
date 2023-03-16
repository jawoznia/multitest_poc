use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult};
use sylvia::interface;

#[cw_serde]
pub struct CountResponse {
    pub count: u32,
}

#[interface]
pub trait Counter {
    type Error: From<StdError>;

    #[msg(exec)]
    fn increase_count(&self, ctx: (DepsMut, Env, MessageInfo)) -> Result<Response, Self::Error>;

    #[msg(query)]
    fn count(&self, ctx: (Deps, Env)) -> StdResult<CountResponse>;
}

// =====================================================================
// ===================== Generated with Interface ======================
// =====================================================================
// TODO: Figure out this module name
#[cfg(test)]
pub mod trait_utils {
    use cosmwasm_std::Addr;
    use utils::sylvia_utils::App;

    pub struct CounterProxy<'app> {
        pub contract_addr: Addr,
        pub app: &'app App,
    }
    impl<'app> CounterProxy<'app> {
        pub fn new(contract_addr: Addr, app: &'app App) -> Self {
            CounterProxy { contract_addr, app }
        }
    }
    impl Into<Addr> for CounterProxy<'_> {
        fn into(self) -> Addr {
            self.contract_addr
        }
    }
}
// =====================================================================
// =====================================================================
