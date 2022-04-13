#![no_std]
#![feature(associated_type_bounds)]

elrond_wasm::imports!();

const DEGRADATION_COEFFICIENT: u64 = 1_000_000_000_000_000_000;
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
        self.last_report().set(self.blockchain().get_block_timestamp());
        self.activation().set(self.blockchain().get_block_timestamp());
        let locked_profit_degradation: BigUint = BigUint::from(DEGRADATION_COEFFICIENT) * 46u64 * BigUint::from(10u64.pow(6)); // 6 blocks
        self.locked_profit_degradation().set(&locked_profit_degradation);
    }
    
    // only owner

    #[only_owner]
    #[endpoint(setSymbol)]
    fn set_symbol(&self, name: ManagedBuffer) {
        self.symbol().set(&name);
    }

    #[only_owner]
    #[endpoint(setManagement)]
    fn set_management(&self, address: ManagedAddress) {
        self.management().set(&address);
    }

    #[only_owner]
    #[endpoint(setRewards)]
    fn set_rewards(&self, rewards: ManagedAddress) {
        require!(rewards != self.blockchain().get_sc_address() && !rewards.is_zero(), "Wrong address for rewards");
        self.rewards().set(&rewards);
    }

    #[only_owner]
    #[endpoint(setLockedProfitDegradation)]
    fn set_locked_profit_degradation(&self, degradation: BigUint) {
        require!(self.locked_profit_degradation().get() <= BigUint::from(DEGRADATION_COEFFICIENT), "");
        self.locked_profit_degradation().set(&degradation);
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

    #[only_owner]
    #[endpoint(setManagementFee)]
    fn set_management_fee(&self, fee: BigUint) {
        require!(fee <= MAX_BPS, "fee must be <= MAX_BPS");
        self.management_fee().set(&fee);
    }

    #[only_owner]
    #[endpoint(setGuardian)]
    fn set_guardian(&self, address: ManagedAddress) {
        self.guardian().set(&address);
    }

    // setEmergencyShutdown not implemented

    // setWithdrawalQueue

    // view

    #[view(totalAssets)]
    fn total_assets(&self) -> BigUint {
        self._total_assets()
    }

    // internal
    fn _total_assets(&self) -> BigUint {
        self.blockchain().get_sc_balance(&self.token().get(), 0) + self.total_debt().get()
    }

    fn _calculate_locked_profit(&self) -> BigUint {
        let locked_funds_ratio: BigUint = 
                (BigUint::from(self.blockchain().get_block_timestamp()) - self.last_report().get()) 
                * self.locked_profit_degradation().get();
        // DEGRADATION_COEFFICIENT = 10 ** 18
        if locked_funds_ratio < BigUint::from(DEGRADATION_COEFFICIENT) {
            let locked_profit = self.locked_profit().get();
            locked_profit * (BigUint::from(1u32) - locked_funds_ratio / BigUint::from(DEGRADATION_COEFFICIENT))
        } else {
            BigUint::zero()
        }
    }

    fn _free_funds(&self) -> BigUint {
        self._total_assets() - self._calculate_locked_profit()
    }

    fn _issues_shares_for_amount(&self, to: ManagedAddress, amount: BigUint) -> BigUint {
        let shares: BigUint= BigUint::zero();
        // implement shares by creating SFT with self.send().esdt_nft_create()
        // and then esdt_local_mint()
        // https://github.com/ElrondNetwork/sc-dex-rs/blob/main/dex/pair/src/lib.rs#L227
        // because with need totalSupply of the LP token (that will act as a Vault share)
        shares
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

    // amount
    #[storage_mapper("lockedProfit")]
    fn locked_profit(&self) -> SingleValueMapper<BigUint>;

    // rate 
    #[storage_mapper("lockedProfitDegradation")]
    fn locked_profit_degradation(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("depositLimit")]
    fn deposit_limit(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("performanceFee")]
    fn performance_fee(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("managementFee")]
    fn management_fee(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("totalDebt")]
    fn total_debt(&self) -> SingleValueMapper<BigUint>;

    // timestamp creation vault
    #[storage_mapper("activation")]
    fn activation(&self) -> SingleValueMapper<u64>;

    // timestamp creation vault
    #[storage_mapper("lastReport")]
    fn last_report(&self) -> SingleValueMapper<u64>;
}
