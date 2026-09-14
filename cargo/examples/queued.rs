use alloy_primitives::address;
use eth_safe::{scg::SCGClient, stx::STXClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let safe = address!("0xAC3EBDC2Dc0Cc20e937C970D46e2A232d3151aef");

    let gateway = SCGClient::default();
    let queued = gateway.queued_transactions(1, safe, None).await?;
    println!(
        "queued: count={:?} items={}",
        queued.count,
        queued.results.len()
    );
    for item in &queued.results {
        println!("  {item:?}");
    }

    let service = STXClient::default();

    let info = service.safe("eth", safe).await?;
    let by_id = service.safe(1, safe).await?;
    println!("slug and id agree: {}", info.address == by_id.address);

    let by_slug = gateway.queued_transactions("eth", safe, None).await?;
    println!("queued via slug: {} items", by_slug.results.len());

    println!(
        "safe: threshold {:?} of {} owners, version {:?}",
        info.threshold,
        info.owners.len(),
        info.version
    );

    let page = service.multisig_transactions(1, safe, None).await?;
    println!(
        "multisig: count={:?} next={:?}",
        page.count,
        page.next.as_ref().map(eth_safe::Cursor::as_str)
    );

    if let Some(cursor) = page.next.as_ref() {
        let page = service.multisig_transactions(1, safe, Some(cursor)).await?;
        println!(
            "multisig page 2: first nonce {:?}",
            page.results.first().and_then(|tx| tx.nonce)
        );
    }

    println!(
        "unknown id: {:?}",
        service.safe(999_999_999u64, safe).await.err()
    );
    println!(
        "unknown slug: {:?}",
        gateway.queued_transactions("nope", safe, None).await.err()
    );
    Ok(())
}
