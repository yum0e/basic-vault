#![no_std]
#![feature(associated_type_bounds)]

elrond_wasm::imports!();

const MANAGEMENT_FEE: u64 = 200;
const MAX_BPS: u64 = 10_000;

#[elrond_wasm::contract]
pub trait yVault {

    #[init]
    fn init(
        &self,
        token: TokenIdentifier,
        management: ManagedAddress,
        guardian: ManagedAddress,
        rewards: ManagedAddress,
        symbol_override: ManagedBuffer,
    ) {

        self.token().set(&token);
        self.symbol().set(&symbol_override);
        self.management().set(&management);
        self.guardian().set(&guardian);
        self.rewards().set(&rewards);
        self.performance_fee().set(BigUint::from(10_000u64));
        self.activation().set(self.blockchain().get_block_timestamp());
        let locked_profit_degradation: BigUint = BigUint::from(10u64.pow(18)) * 46u64 * BigUint::from(10u64.pow(6)); // 6 blocks
        self.locked_profit_degradation().set(&locked_profit_degradation);
    }
    
    // only owner

    #[only_owner]
    #[endpoint(setSymbol)]
    fn set_symbol(&self, name: ManagedBuffer) {
        self.symbol().set(&name);
    }

    #[only_owner]
    #[endpoint(setMangement)]
    fn set_management(&self, address: ManagedAddress) {
        self.management().set(&address);
    }

    #[only_owner]
    #[endpoint(setRewards)]
    fn set_rewards(&self, rewards: ManagedAddress) {
        require!(rewards != self.blockchain().get_sc_address() && !rewards.is_zero(), "Wrong address for rewards");
        require!(self.locked_profit_degradation().get() <= BigUint::from(10u64.pow(18)), "");
        self.rewards().set(&rewards);
    }

    #[only_owner]
    #[endpoint(setDepositLimit)]
    fn set_deposit_limit(&self, limit: BigUint) {
        self.deposit_limit().set(&limit);
    }

    #[only_owner]
    #[endpoint(setPerformanceFee)]
    fn set_performance_fee(&self, fee: BigUint) {
        require!(fee <= MAX_BPS, "fee must be <= MAX_BPS");
        self.performance_fee().set(&fee);
    }

    // storage 

    #[storage_mapper("token")]
    fn token(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("symbol")]
    fn symbol(&self) -> SingleValueMapper<ManagedBuffer>;

    #[storage_mapper("management")]
    fn management(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("guardian")]
    fn guardian(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("rewards")]
    fn rewards(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("lockedProfitDegradation")]
    fn locked_profit_degradation(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("depositLimit")]
    fn deposit_limit(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("performanceFee")]
    fn performance_fee(&self) -> SingleValueMapper<BigUint>;

    // timestamp creation vault
    #[storage_mapper("activation")]
    fn activation(&self) -> SingleValueMapper<u64>;
}
