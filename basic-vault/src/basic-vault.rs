// v1
#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct Vault<M: ManagedTypeApi> {
    pub id: u64,
    pub creator: ManagedAddress<M>,
    pub name: ManagedBuffer<M>,
    pub users: ManagedVec<M, ManagedAddress<M>>,
    pub amount: BigUint<M>
}

#[elrond_wasm::contract]
pub trait BasicVault {

    #[init]
    fn init(&self) {
        self.total_vaults().set_if_empty(&0u64);
    }

    // endpoints

    // create the vault and return the vault_id
    #[payable("*")]
    #[endpoint(createVault)]
    fn create_vault(
        &self,
        #[payment_token] token: TokenIdentifier,
        #[payment_amount] initial_amount: BigUint,
        name: ManagedBuffer,
    ) -> SCResult<u64> {
        require!(token.is_egld(), "The token must be egld");
        require!(initial_amount > 0, "Supply some egld in the vault");

        let vault = Vault {
            id: self.total_vaults().get(),
            creator: self.blockchain().get_caller(),
            name,
            users: ManagedVec::new(),
            amount: initial_amount,
        };

        // store the vault with its associated id
        self.vaults(vault.id).set(&vault);

        // update the id counter
        self.total_vaults().update(|nb| *nb + 1);

        // return the id of the created vault
        Ok(vault.id)
    }

    #[endpoint(addUser)]
    fn add_user(
        &self, 
        vault_id: u64, 
        new_user: ManagedAddress
    ) {
        // retrieve the vault
        let mut vault = self.vaults(vault_id).get();
        require!(self.blockchain().get_caller() == vault.creator, "Not vault owner");
        vault.users.push(new_user);
        self.vaults(vault_id).set(&vault);
    }

    #[payable("*")]
    #[endpoint(addAmount)]
    fn add_amount(
        &self, 
        #[payment_token] token: TokenIdentifier,
        #[payment_amount] amount: BigUint,
        vault_id: u64, 
    ) {
        require!(token.is_egld(), "The token must be egld");
        require!(amount > 0, "Supply some egld in the vault");
        // retrieve the vault
        let mut vault = self.vaults(vault_id).get();
        require!(self.blockchain().get_caller() == vault.creator, "Not vault owner");

        // add the amount
        vault.amount += amount;
        self.vaults(vault_id).set(vault);
    }

    #[endpoint(distribute)]
    fn distribute(&self, vault_id: u64) {
        // retrieve the vault
        let mut vault = self.vaults(vault_id).get();
        require!(vault.users.len() > 0, "No addresses to distribute to");
        let vault_amount = &vault.amount;
        let amount_per_user = vault_amount / &BigUint::from(vault.users.len());
        require!(vault.amount != 0, "No funds available");
        for users in vault.users.iter() {
            vault.amount -= &amount_per_user;
            self.send().direct_egld(
                &users,
                &amount_per_user,
                &[]
            );
        }
    }

    // view 


    // storage

    // know the number of vaults created
    #[storage_mapper("totalVaults")]
    fn total_vaults(&self) -> SingleValueMapper<u64>;

    // map from the id of a vault to its storage
    #[storage_mapper("vaults")]
    fn vaults(&self, id: u64) -> SingleValueMapper<Vault<Self::Api>>;


}
