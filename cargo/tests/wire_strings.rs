use eth_safe::scg::transactions::{ConflictType, Direction, TxStatus};

fn round_trip<T>(cases: &[(&str, T)])
where
    T: std::fmt::Debug + PartialEq + From<String> + std::fmt::Display,
{
    for (wire, expected) in cases {
        let parsed = T::from((*wire).to_owned());

        assert_eq!(&parsed, expected, "{wire} parsed to the wrong variant");
        assert_eq!(&parsed.to_string(), wire, "{wire} did not survive Display");
    }
}

#[test]
fn tx_status_wire_strings() {
    round_trip(&[
        ("SUCCESS", TxStatus::Success),
        ("FAILED", TxStatus::Failed),
        ("CANCELLED", TxStatus::Cancelled),
        ("AWAITING_CONFIRMATIONS", TxStatus::AwaitingConfirmations),
        ("AWAITING_EXECUTION", TxStatus::AwaitingExecution),
    ]);
}

#[test]
fn conflict_type_wire_strings() {
    round_trip(&[
        ("None", ConflictType::None),
        ("HasNext", ConflictType::HasNext),
        ("End", ConflictType::End),
    ]);
}

#[test]
fn direction_wire_strings() {
    round_trip(&[
        ("INCOMING", Direction::Incoming),
        ("OUTGOING", Direction::Outgoing),
    ]);
}

#[test]
fn an_unknown_value_is_captured_verbatim() {
    let status = TxStatus::from("AWAITING_SOMETHING_NEW".to_owned());

    assert_eq!(
        status,
        TxStatus::Unknown("AWAITING_SOMETHING_NEW".to_owned())
    );
    assert_eq!(status.to_string(), "AWAITING_SOMETHING_NEW");

    assert_eq!(
        TxStatus::from("success".to_owned()),
        TxStatus::Unknown("success".to_owned())
    );
}
