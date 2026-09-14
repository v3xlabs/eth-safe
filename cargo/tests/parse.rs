use chrono::{DateTime, Utc};
use eth_safe::Page;
use eth_safe::scg::transactions::{
    ConflictType, Direction, ExecutionInfo, QueuedItem, TransactionInfo, TransferValue, TxStatus,
};
use eth_safe::stx::{MultisigTransaction, Operation, SafeInfo};

fn fixture<T: serde::de::DeserializeOwned>(name: &str) -> T {
    let path = format!("{}/tests/fixtures/{name}", env!("CARGO_MANIFEST_DIR"));
    let body = std::fs::read(&path).expect("fixture is readable");

    serde_json::from_slice(&body).unwrap_or_else(|error| panic!("{name} failed to parse: {error}"))
}

fn instant(rfc3339: &str) -> DateTime<Utc> {
    rfc3339
        .parse()
        .expect("the test constant is a valid timestamp")
}

#[test]
fn queued_page_keeps_the_label_grouping() {
    let page: Page<QueuedItem> = fixture("scg_queued.json");

    assert_eq!(page.count, Some(2));
    assert!(page.next.is_none());

    let QueuedItem::Label { label, .. } = &page.results[0] else {
        panic!(
            "expected the first item to be a label, got {:?}",
            page.results[0]
        );
    };
    assert_eq!(label, "Next");
}

#[test]
fn queued_erc20_transfer_is_fully_decoded() {
    let page: Page<QueuedItem> = fixture("scg_queued.json");

    let QueuedItem::Transaction {
        transaction,
        conflict_type,
        ..
    } = &page.results[1]
    else {
        panic!("expected a transaction item");
    };

    assert_eq!(conflict_type.as_ref(), Some(&ConflictType::None));
    assert_eq!(transaction.tx_status, Some(TxStatus::AwaitingConfirmations));
    assert!(transaction.tx_hash.is_none());
    assert_eq!(
        transaction.timestamp,
        Some(instant("2026-05-02T21:57:23.171Z"))
    );

    let Some(TransactionInfo::Transfer(transfer)) = &transaction.tx_info else {
        panic!("expected a transfer");
    };
    assert_eq!(transfer.direction, Some(Direction::Outgoing));

    let Some(TransferValue::Erc20 {
        value,
        token_symbol,
        decimals,
        ..
    }) = &transfer.transfer_info
    else {
        panic!("expected an erc20 transfer");
    };
    assert_eq!(*value, Some(alloy_primitives::U256::from(1_000_000u64)));
    assert_eq!(token_symbol.as_deref(), Some("USDC"));
    assert_eq!(*decimals, Some(6));

    let Some(ExecutionInfo::Multisig {
        nonce,
        confirmations_required,
        confirmations_submitted,
        missing_signers,
        ..
    }) = &transaction.execution_info
    else {
        panic!("expected multisig execution info");
    };
    assert_eq!(*nonce, Some(20));
    assert_eq!(*confirmations_required, Some(2));
    assert_eq!(*confirmations_submitted, Some(1));
    assert_eq!(missing_signers.as_ref().map(Vec::len), Some(2));
}

#[test]
fn a_future_transaction_type_survives_as_unknown() {
    let body = br#"{"count":1,"next":null,"previous":null,"results":[
        {"type":"TRANSACTION","conflictType":"None","transaction":{
            "id":"multisig_0x00_0x01","timestamp":1,"txStatus":"AWAITING_EXECUTION",
            "txInfo":{"type":"SomeTypeFromTheFuture","payload":{"a":1}}}}]}"#;

    let page: Page<QueuedItem> = serde_json::from_slice(body).expect("still parses");

    let QueuedItem::Transaction { transaction, .. } = &page.results[0] else {
        panic!("expected a transaction item");
    };
    let Some(TransactionInfo::Unknown(raw)) = &transaction.tx_info else {
        panic!("expected the unknown fallback");
    };

    assert_eq!(raw["type"], "SomeTypeFromTheFuture");
    assert_eq!(raw["payload"]["a"], 1);
    assert_eq!(transaction.tx_status, Some(TxStatus::AwaitingExecution));
}

#[test]
fn a_renamed_field_degrades_the_field_not_the_variant() {
    let body = br#"{"type":"Transfer","direction":"OUTGOING","recipientRenamed":{"value":"0x0000000000000000000000000000000000000001"}}"#;

    let info: TransactionInfo = serde_json::from_slice(body).expect("still parses");

    let TransactionInfo::Transfer(transfer) = info else {
        panic!("a renamed field must not collapse the variant into Unknown");
    };
    assert_eq!(transfer.direction, Some(Direction::Outgoing));
    assert!(transfer.recipient.is_none());
}

#[test]
fn safe_info_reads_the_string_nonce() {
    let safe: SafeInfo = fixture("stx_safe.json");

    assert_eq!(safe.nonce, Some(alloy_primitives::U256::from(20u64)));
    assert_eq!(safe.threshold, Some(2));
    assert_eq!(safe.owners.len(), 3);
    assert_eq!(safe.version.as_deref(), Some("1.3.0"));
    assert_eq!(safe.modules.len(), 1);
}

#[test]
fn a_next_link_becomes_a_bare_query_string() {
    let page: Page<MultisigTransaction> = fixture("stx_multisig_page.json");

    assert_eq!(
        page.next.as_ref().map(eth_safe::Cursor::as_str),
        Some("limit=2&offset=2")
    );
    assert!(page.previous.is_none());

    let first = &page.results[0];
    assert_eq!(first.operation, Some(Operation::Call));
    assert_eq!(first.nonce, Some(20));
    assert_eq!(first.confirmations.len(), 1);
    assert_eq!(first.is_executed, Some(false));
    assert_eq!(
        first.submission_date,
        Some(instant("2026-05-02T21:57:23.171148Z"))
    );
    assert!(first.execution_date.is_none());
}

#[test]
fn a_gateway_link_loses_its_downgraded_scheme_and_host() {
    let page: Page<serde_json::Value> = fixture("scg_history_page.json");

    let cursor = page.next.expect("this fixture is paginated");

    assert_eq!(
        cursor.as_str(),
        "page_size=4&cursor=limit%3D20%26offset%3D20"
    );
    assert!(!cursor.as_str().contains("http"));
    assert!(!cursor.as_str().contains("safe-client.safe.global"));
}

#[test]
fn an_unrecognised_field_is_kept_in_extra() {
    let body = br#"{"type":"Transfer","direction":"OUTGOING","someNewField":{"a":1}}"#;

    let TransactionInfo::Transfer(transfer) =
        serde_json::from_slice::<TransactionInfo>(body).expect("still parses")
    else {
        panic!("expected a transfer");
    };

    eprintln!("extra = {:?}", transfer.extra);
    assert_eq!(transfer.extra["someNewField"]["a"], 1);
    assert!(
        !transfer.extra.contains_key("type"),
        "tag leaked into extra"
    );
    assert!(
        !transfer.extra.contains_key("direction"),
        "parsed field leaked"
    );
}

#[test]
fn a_pagination_link_that_cannot_become_a_cursor_is_an_error() {
    // A present `next` that yields no cursor must not decode to None: that is the same value
    // as "last page", so paging would stop silently in the middle of a safe's history.
    let no_query = br#"{"count":9,"next":"https://host/v1/safes","previous":null,"results":[]}"#;
    let malformed = br#"{"count":9,"next":"not a url","previous":null,"results":[]}"#;
    let absent = br#"{"count":9,"next":null,"previous":null,"results":[]}"#;

    assert!(serde_json::from_slice::<Page<serde_json::Value>>(no_query).is_err());
    assert!(serde_json::from_slice::<Page<serde_json::Value>>(malformed).is_err());

    let last = serde_json::from_slice::<Page<serde_json::Value>>(absent).expect("null is valid");
    assert!(last.next.is_none());
}
