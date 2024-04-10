# Archway Drop Camp Exporter

Application to export specific on-chain activities for wallets participating in
Archway's Drop Camp. The initial list of participating addresses is fetch from
the Soulbound NFT token that validates social activity. From, there the
application will export multiple CSV files with the raw data for each of the
other activity groups.

## Exported Activities

- _Bridged assets:_ exports all assets currently available on the wallet, with
  one address / balance per line.
- _Stake:_ current `ARCH` delegations per address.
- _ArchID:_ total domains registered on the CW721 contract, with one address per
  line.
- _Astrovault:_ exports the LPs positions using Astrovault's proprietary API.
- _Liquid Finance:_ exports the `sARCH` balance per address from the CW20 contract.

## Usage

Clone the repository

```bash
git clone https://github.com/archway-network/dropcamp-exporter
cd dropcamp-exporter
```

Run it using `cargo`

```console
$ cargo run -- --help

Usage: dropcamp-exporter [OPTIONS] --soulbound-address <SOULBOUND_ADDRESS> --archid-address <ARCHID_ADDRESS> --liquid-finance-address <LIQUID_FINANCE_ADDRESS> --astrovault-url <ASTROVAULT_URL> --output <OUTPUT>

Options:
      --rpc-url <RPC_URL>
          Url for the RPC endpoint [default: https://rpc.mainnet.archway.io:443]
      --rpc-req-second <RPC_REQ_SECOND>
          Limits the number of requests per second to the RPC endpoint
      --height <HEIGHT>
          Runs the operation on a specific block height. Otherwise, it will query the chain to get the latest block height
      --soulbound-address <SOULBOUND_ADDRESS>
          Address for the soulbound token cw721 smart contract
      --archid-address <ARCHID_ADDRESS>
          Address for the ArchID registry smart contract
      --liquid-finance-address <LIQUID_FINANCE_ADDRESS>
          Address for the Liquid Finance cw20 smart contract
      --astrovault-url <ASTROVAULT_URL>
          Url for the Astrovault liquidity pools API
      --astrovault-req-second <ASTROVAULT_REQ_SECOND>
          Limits the number of requests per second to the Astrovault API
      --astrovault-api-key <ASTROVAULT_API_KEY>
          API key for the Astrovault API
  -o, --output <OUTPUT>
          Directory path to output the CSV files
      --log-level <LEVEL>
          Sets the log level [default: info]
  -h, --help
          Print help
  -V, --version
          Print version
```

### Example

```bash
cargo run -- -o ./data \
    --rpc-req-second 250 \
    --soulbound-address archway1cwypf946sdmhgcaz2tjrqmvnf9rnq8rmy9sfce22d6z84fdwddysx74xh6 \
    --archid-address archway1cf5rq0amcl5m2flqrtl4gw2mdl3zdec9vlp5hfa9hgxlwnmrlazsdycu4l \
    --liquid-finance-address archway1t2llqsvwwunf98v692nqd5juudcmmlu3zk55utx7xtfvznel030saclvq6 \
    --astrovault-url ${AV_URL} \
    --astrovault-api-key ${AV_API_KEY} \
    --astrovault-req-second 10
```

## Output

All CSV files will be exported to the folder specified in the `--output` flag:

```
📁 output
├── 📄 archid.csv
├── 📄 astrovault.csv
├── 📄 balances.csv
├── 📄 liquid-finance.csv
├── 📄 socials.csv
└── 📄 staking.csv
```

### Schema

#### `archid.csv`

- `address` (`string`): wallet address
- `ranking` (`float`): ranking percentage for this activity
- `domains` (`integer`): total number of domains
- `names` (`string`): list of domain names separated by `,`

#### `astrovault.csv`

- `address` (`string`): wallet address
- `has_lpd` (`bool`): flag if address has provided liquidity
- `has_traded` (`bool`): flag if address has traded
- `tvl` (`float`): total value locked

#### `balances.csv`

- `address` (`string`): wallet address
- `denom` (`string`): coin denominator (IBC tokens start with `ibc/`)
- `amount` (`bigint`): token balance

> [!NOTE]  
> The `address` might appear multiple times, one for each `denom` it has.

#### `liquid-finance.csv`

- `address` (`string`): wallet address
- `balance` (`bigint`): total `sARCH` balance

#### `socials.csv`

- `address` (`string`): wallet address
- `ranking` (`float`): ranking percentage for this activity
- `patch_name` (`string`): name of the drop camp patch associated with the score
- `social_score` (`integer`): score based on user activity in socials

#### `staking.csv`

- `address` (`string`): wallet address
- `ranking` (`float`): ranking percentage for this activity
- `delegated` (`float`): delegated amount rounded to 2 decimals
- `validators` (`string`): validator addresses separated by `,`
