use eth_safe::{NetworkId, NetworkIdOrSafeSlug, NetworkSafeSlug};

#[test]
fn the_slug_is_not_the_chain_short_name() {
    assert_eq!(
        NetworkId(137).safe_slug().map(|slug| slug.to_string()),
        Some("pol".to_owned())
    );
    assert_eq!(
        NetworkSafeSlug::from("pol").network_id(),
        Some(NetworkId(137))
    );
    assert_eq!(NetworkSafeSlug::from("matic").network_id(), None);
}

#[test]
fn id_and_slug_round_trip() {
    for id in [1u64, 10, 137, 8453, 42161] {
        let slug = NetworkId(id).safe_slug().expect("a known network");
        assert_eq!(slug.network_id(), Some(NetworkId(id)));
    }
}

#[test]
fn either_form_resolves_to_both() {
    let from_id: NetworkIdOrSafeSlug = 1u64.into();
    let from_slug: NetworkIdOrSafeSlug = "eth".into();

    assert_eq!(from_id.network_id(), Some(NetworkId(1)));
    assert_eq!(from_slug.network_id(), Some(NetworkId(1)));
    assert_eq!(from_id.safe_slug(), from_slug.safe_slug());
    assert_eq!(
        from_id.safe_slug().map(|s| s.to_string()),
        Some("eth".to_owned())
    );
}

#[test]
fn an_unknown_network_resolves_to_nothing() {
    let unknown: NetworkIdOrSafeSlug = 999_999_999u64.into();
    assert_eq!(unknown.safe_slug(), None);
    assert_eq!(unknown.to_string(), "999999999");

    let custom: NetworkIdOrSafeSlug = "my-devnet".into();
    assert_eq!(custom.network_id(), None);
    assert_eq!(
        custom.safe_slug().map(|s| s.to_string()),
        Some("my-devnet".to_owned())
    );
}

#[test]
fn a_network_id_converts_both_ways() {
    assert_eq!(NetworkId::from(1u64), NetworkId(1));
    assert_eq!(u64::from(NetworkId(1)), 1u64);
}
