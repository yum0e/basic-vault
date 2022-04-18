// npx ts-node deploy.ts to execute the script

import { ProxyNetworkProvider } from "@elrondnetwork/erdjs-network-providers";
import { Address, Account } from "@elrondnetwork/erdjs";

const customEgldTx = async (amount: string, to: Address) => {};

async function main() {
  let networkProvider = new ProxyNetworkProvider(
    "https://devnet-gateway.elrond.com"
  );

  let networkConfig = await networkProvider.getNetworkConfig();

  let addresses = [
    // yum1
    new Address(
      "erd14q22erffu7r56mf26yx4erww9k0yresxmudte0etacl950ef7fys9qcus5"
    ),
    // yum2
    new Address(
      "erd1wx7h5rnyxre7avl5pkgj3c2fha9aknrwms8mspelfcapwvjac3vqncm7nm"
    ),
  ];
  let yum1 = new Account(addresses[0]);
  let yum1OnNetwork = await networkProvider.getAccount(addresses[0]);
  let yum2 = new Account(addresses[1]);
  let yum2OnNetwork = await networkProvider.getAccount(addresses[1]);

  yum1.update(yum1OnNetwork);
  yum2.update(yum2OnNetwork);

  console.log("Balance yum1: ", yum1.balance);
  console.log("Balance yum2: ", yum2.balance);
}

main();
