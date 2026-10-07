//! Amsterdam fields must survive generic RPC header conversions.
#![cfg(feature = "serde")]

use alloy_consensus::{BlockHeader, Header};
use alloy_consensus_any::AnyHeader;

#[test]
fn rpc_header_conversions_preserve_amsterdam_fields() {
    let any: AnyHeader =
        serde_json::from_str(include_str!("../../consensus/tests/glamsterdam_header.json"))
            .unwrap();
    let header: Header = any.clone().try_into_header().unwrap();
    assert_eq!(
        header.hash_slow(),
        alloy_primitives::b256!("fd46050a13e31903f97479ef96a8430d308eff7e80f04dc9eeca3fef95424ff9")
    );
    assert_eq!(any.clone().into_header_with_defaults(), header);
    assert_eq!(AnyHeader::from(header.clone()), any);
    assert_eq!(BlockHeader::block_access_list_hash(&any), header.block_access_list_hash);
    assert_eq!(BlockHeader::slot_number(&any), header.slot_number);
    let wrapped = alloy_serde::WithOtherFields::new(header);
    assert_eq!(BlockHeader::block_access_list_hash(&wrapped), any.block_access_list_hash);
    assert_eq!(BlockHeader::slot_number(&wrapped), any.slot_number);
}
