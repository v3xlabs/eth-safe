<h1 align="center">eth-safe</h1>

<p align="center">
  A smol rust crate for reading Safe{Wallet} multisig state from the Safe apis.
</p>
<p align="center">
    <a href="https://docs.rs/eth-safe"><img src="https://img.shields.io/badge/Docs.rs-blue?logo=rust&color=brown&style=flat" alt="Documentation"></a>
    <a href="https://crates.io/crates/eth-safe"><img src="https://img.shields.io/badge/Crate.io-yellow?logo=data:image/x-icon;base64%2CiVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAMAAAAoLQ9TAAACylBMVEUAAADqvWfotVLot1n/wi3lt1/grEfls1HnuFzXnCnXnS3luWTfzavgq0PlwXvltFXlrkTnt4KJYzl4XDz/yHwUEAsIBgQXEgyhek+DZ0j///9JOShFNibsxHnls1Llsk7ntVTpu2Lnv3Lntlf///bhrkrkslHnumTouFvpuV3otlbntFDotlbZoDDpuV7ot1fntlbnumLWnjLirEHlsUvcpz/lr0XlqznmsEjsvGDaozbmrDrgq0LpyIbmtlnlslDntVbdqUXmrTzfq0TbqUXcp0DirELgrUjLlS3OlCPiqTjkrULDjCLDiRfbozfrs0WUahe4fw3hqTrhrEVYRCiQaCfDlk2yjV0yJxppUDZ1WzqEYCKMZzqZd1GxjmgAAAALCAYEAwIwJRt2WjmKZi6fbxeifVKKZ0J3VjNlTC8AAAAAAAALCAVCMiI0KB1DMx+NajaIZDp5WDSFZkOEak4AAAAcFQ5xVzyffViAYDxkTjXnu2XnuV/dpzzjtFnlsk/ntFPntlnnu2Tou2Tbozfjrkfntlfou2XoumLntFLntlbnumLnu2Pir0znvWznumPntljotFDnskrkqzzmrkDbojTgrEfgqkHmr0XiqTvgpzbmrT3mrDjnrDjQliLTmCXVmyvhqDjJljHMmDLnrj/prjzorz/kt17nvGnZp0XGjBjKjxvQlSTepTfEkjHTnTXpsEHpsUTXojzbpj7SnTTDjSTGkSrIjhzjqjnepzrjrD7lr0biqz/cpTnHjBjKjx3MlCfNlSjJlC3GjiLhqTzls1Djrkbkqz3UnjTCkC++gxC/hRLFjB65gxm9hhvPliffpzjlrT3lrDrPmzTKljKwgy+4gBO/hhWrdxGlcxLFjR7kqTfnrDnorz7jqjvRnj2ndx6xeg6odAy/hxfkqjjqsUHmrkHUoUSxhUG7gxXgpzjRo1CmgEewh00nmQYaAAAAe3RSTlMAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAMpetXWfCgBT9/9/d2TQg1v88FlCG7ZG2z+1xsOi9cbASl3xvbXGxHA1xwV0NgdFc/YHRrR2R004+ozJaHF1PauIQoaMmrr9vf0sk8MAQgcWnic+fGtSwsQPrm/UQuNPWZaAAABG0lEQVQY0wEQAe/+AAAAAAEdHh8gISIjJAIDBAUAAAAABiUmJ3t8KCkqKywHCAAAAAAJLX1+f4CBgoMuLzAxAAAAAAoyhIWGh4iJiouMMzQAAAsMDTWNjo+QkZI2k5Q3OAAODxA5OpWWl5iZmpucnTs8AD0+P0BBnp+goaKjpKWmQkMAREWnqKmqq6ytrq+wsYpGRwBISbKztLW2t7i5uru8vUpLAExNvr/AwcLDxMXGx8jJTk8AUFHKy8zNzs/Q0dKb09RSUwBUVdXW19jZ2tud3N3e31ZXAFhZWlvg4eLj5OXm5+hcXV4AX2BhYmNkZenq6+xmZ2hpEQAAamtsbW5vcO1xcnN0EhMUABUWABdqdXZ3eHl6GBkaGxxTKXBYeDUm8QAAAABJRU5ErkJggg==" alt="Crates.io"></a>
    <a href="https://github.com/v3xlabs/eth-safe"><img src="https://img.shields.io/badge/Repository-v3xlabs/eth--safe-blue?style=flat" alt="Repository"></a>
    <a href="#"><img src="https://img.shields.io/badge/Status-In%20Development-blue?style=flat" alt="Status: In Development"></a>
    <a href="#"><img src="https://img.shields.io/badge/License-LGPL--3.0-hotpink?style=flat" alt="License: LGPL-3.0"></a>
</p>

> [!IMPORTANT]
> eth-safe is under development and its api is not stable yet. The typescript implementation is not available.

Safe{Wallet} splits its data across two apis, the Client Gateway that backs the web interface and the Transaction Service that indexes the chain. They paginate, timestamp, and name networks differently.
To easily integrate pending multisig transactions into your application eth-safe wraps both, and keeps whatever it cannot parse in an `extra` field instead of dropping it.

## Quickstart

```sh
cargo add eth-safe
```

```rust
use alloy_primitives::address;
use eth_safe::scg::SCGClient;
use eth_safe::scg::transactions::QueuedItem;

// Create a client
let safe = SCGClient::default();

// Read the transactions still waiting on signatures
let address = address!("0xAC3EBDC2Dc0Cc20e937C970D46e2A232d3151aef");
let queued = safe.queued_transactions(1, address, None).await?;

for item in &queued.results {
    if let QueuedItem::Transaction { transaction, .. } = item {
        println!("{:?}", transaction.tx_status); // Some(AwaitingConfirmations)
    }
}
```

## Data Sources

Currently supported data sources include:

- [Client Gateway](https://github.com/v3xlabs/eth-safe/tree/master/cargo/src/scg) - queued transactions, decoded transfers and settings changes
- [Transaction Service](https://github.com/v3xlabs/eth-safe/tree/master/cargo/src/stx) - safe configuration, multisig transactions and confirmations

## Self Hosting

Both clients take their own base url, and the transaction service accepts a slug for networks the built-in table does not cover.

```rust
use eth_safe::{scg::SCGClient, stx::STXClient};
use url::Url;

let gateway = SCGClient::new(Url::parse("https://cgw.internal/")?)?;

let mut service = STXClient::new(Url::parse("https://tx.internal/")?)?;
service.set_slug(1, "mainnet");

// A network is addressed by id or by slug, whichever you have.
let safe = service.safe("eth", address).await?;
```

## Documentation

You can read the documentation at [docs.rs/eth-safe](https://docs.rs/eth-safe).
