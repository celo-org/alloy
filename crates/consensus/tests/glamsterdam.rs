//! Regression coverage for the Sepolia Amsterdam L1 header that failed proving.
use alloy_consensus::Header;
use alloy_primitives::{hex, B256};
use alloy_rlp::{Decodable, Encodable};

const RLP: &str = "f902a3a07d4b1c49c4adcbc2f653404b4693a5a6e15a44a3bebbee41f07331b22705f430a01dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d4934794455e5aa18469bc6ccef49594645666c587a3a71ba0936793c10125cb1dee69535b8e258c541e8bb2e0832c8309fa32378040fe87a4a0066c2f0770c3072424557e9a14ce9188b23ee854ff96e6c8126a6e155eef68afa09cde632620f8e0b64ff3675362586c9e806512a294a5af42d2c97612b538cce2b9010095ac20248397965188483d329405a2401950635540814c8c101880042839a73249db452101843a031200a122013200112b0ac40d46a629440d400db1e0f4bcbe12445d0464e0222a082c297e42ca7032468d864620d40116e8e184eaa08ec401ee9409832628441aa8049462b1610c4c32244b49149d740051105190008d3108a84cf0840956588b7882014841a41c204180320380f35068a816006c407606160330420d0575318b524841648231e10b9888b530041e02c088e0420b7e004198682a9e16b120808e006825a042a22c86121702c092088872e9cb6f0a45cc71c527f0021150974844892300342e219508b202153cc8888c60eb21fe42001a288d8083b4ead88403ddcba784022fc40a846ac50c24914e65746865726d696e642076322e312e30a019cf9662201cbb962db6a57faa5cfb6db110539506cbd90528869c2b3ffa1ebc880000000000000000845841e1b1a0d3933055f95177feee4d4f6247eec39c853fec461c6911870511a66f5d81a1ce830e0000840c9d2095a0628e9e7ee77e756a3d3e04c183038f5de9c298403eb721ef530def64dbfe70daa0e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855a046fd058ca2c40cd92dbc6efe82b30dfa5c4686ec409d6336e18620cfe2140b3b83ac613b";
const HASH: B256 =
    alloy_primitives::b256!("fd46050a13e31903f97479ef96a8430d308eff7e80f04dc9eeca3fef95424ff9");

#[test]
fn glamsterdam_rlp_hash_roundtrip() {
    let bytes = hex::decode(RLP).unwrap();
    let mut input = bytes.as_slice();
    let header = Header::decode(&mut input).unwrap();
    assert!(input.is_empty());
    assert_eq!(header.hash_slow(), HASH);
    assert_eq!(header.length(), bytes.len());
    assert_eq!(alloy_rlp::encode(&header), bytes);
}

#[cfg(feature = "serde")]
#[test]
fn glamsterdam_json_hash_roundtrip() {
    let header: Header = serde_json::from_str(include_str!("glamsterdam_header.json")).unwrap();
    assert_eq!(header.hash_slow(), HASH);
    let value = serde_json::to_value(&header).unwrap();
    assert_eq!(value["slotNumber"], "0xac613b");
    assert_eq!(
        value["blockAccessListHash"],
        "0x46fd058ca2c40cd92dbc6efe82b30dfa5c4686ec409d6336e18620cfe2140b3b"
    );
}

#[test]
fn pre_amsterdam_header_hash_is_unchanged() {
    let bytes = hex::decode(RLP).unwrap();
    let mut payload = bytes.as_slice();
    alloy_rlp::Header::decode(&mut payload).unwrap();
    let mut old_bytes = Vec::new();
    alloy_rlp::Header { list: true, payload_length: 638 }.encode(&mut old_bytes);
    old_bytes.extend_from_slice(&payload[..638]);
    let header = Header::decode(&mut old_bytes.as_slice()).unwrap();
    assert_eq!(header.block_access_list_hash, None);
    assert_eq!(header.slot_number, None);
    assert_eq!(
        header.hash_slow(),
        alloy_primitives::b256!("cec5a0a19e01ab12e4bda01730e7b3aba6af5f908219c98b2b7a5bd3279d5d41")
    );
    assert_eq!(alloy_rlp::encode(header), old_bytes);
}

#[test]
fn malformed_amsterdam_suffix_is_rejected() {
    let bytes = hex::decode(RLP).unwrap();
    let mut payload = bytes.as_slice();
    alloy_rlp::Header::decode(&mut payload).unwrap();
    let mut extra_field = Vec::new();
    alloy_rlp::Header { list: true, payload_length: payload.len() + 1 }.encode(&mut extra_field);
    extra_field.extend_from_slice(payload);
    extra_field.push(0x80);
    assert!(Header::decode(&mut extra_field.as_slice()).is_err());
    assert!(Header::decode(&mut &bytes[..bytes.len() - 1]).is_err());
}
