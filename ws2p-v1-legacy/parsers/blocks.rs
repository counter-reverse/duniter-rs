extern crate serde_json;

use super::excluded::parse_exclusions_from_json_value;
use super::identities::parse_compact_identity;
use super::transactions::parse_transaction;
use duniter_crypto::hashs::Hash;
use duniter_crypto::keys::*;
use duniter_documents::blockchain::v10::documents::block::{BlockV10Parameters, TxDocOrTxHash};
use duniter_documents::blockchain::v10::documents::membership::*;
use duniter_documents::blockchain::v10::documents::BlockDocument;
use duniter_documents::CurrencyName;
use duniter_documents::{BlockHash, BlockId};
use duniter_network::{NetworkBlock, NetworkBlockV10};
use std::str::FromStr;

fn parse_previous_hash(block_number: BlockId, source: &serde_json::Value) -> Option<Hash> {
    match source.get("previousHash")?.as_str() {
        Some(hash_str) => match Hash::from_hex(hash_str) {
            Ok(hash) => Some(hash),
            Err(_) => None,
        },
        None => if block_number.0 > 0 {
            None
        } else {
            Some(Hash::default())
        },
    }
}

fn parse_previous_issuer(source: &serde_json::Value) -> Option<PubKey> {
    match source.get("previousIssuer")?.as_str() {
        Some(pubkey_str) => match ed25519::PublicKey::from_base58(pubkey_str) {
            Ok(pubkey) => Some(PubKey::Ed25519(pubkey)),
            Err(_) => None,
        },
        None => None,
    }
}

fn parse_memberships(
    currency: &str,
    membership_type: MembershipType,
    json_memberships: &serde_json::Value,
) -> Option<Vec<MembershipDocument>> {
    let mut memberships = Vec::new();
    for membership in super::memberships::parse_memberships_from_json_value(
        currency,
        membership_type,
        &json_memberships.as_array()?,
    ) {
        if let Ok(membership) = membership {
            memberships.push(membership);
        } else {
            warn!("dal::parsers::blocks::parse_memberships() : MembershipParseError !")
        }
    }
    Some(memberships)
}

pub fn parse_json_block(source: &serde_json::Value) -> Option<NetworkBlock> {
    let number = BlockId(source.get("number")?.as_u64()? as u32);
    let currency = source.get("currency")?.as_str()?.to_string();
    let issuer = match ed25519::PublicKey::from_base58(source.get("issuer")?.as_str()?) {
        Ok(pubkey) => PubKey::Ed25519(pubkey),
        Err(_) => return None,
    };
    let sig = match ed25519::Signature::from_base64(source.get("signature")?.as_str()?) {
        Ok(sig) => Sig::Ed25519(sig),
        Err(_) => return None,
    };
    let hash = match Hash::from_hex(source.get("hash")?.as_str()?) {
        Ok(hash) => hash,
        Err(_) => return None,
    };
    let parameters = if let Some(params_json) = source.get("parameters") {
        if let Ok(params) = BlockV10Parameters::from_str(params_json.as_str()?) {
            Some(params)
        } else {
            None
        }
    } else {
        None
    };
    let previous_hash = parse_previous_hash(number, source)?;
    let previous_issuer = parse_previous_issuer(source);
    let inner_hash = match Hash::from_hex(source.get("inner_hash")?.as_str()?) {
        Ok(hash) => Some(hash),
        Err(_) => return None,
    };
    let dividend = match source.get("dividend")?.as_u64() {
        Some(dividend) => Some(dividend as usize),
        None => None,
    };
    let mut identities = Vec::new();
    for raw_idty in source.get("identities")?.as_array()? {
        identities.push(parse_compact_identity(&currency, &raw_idty)?);
    }
    let joiners = parse_memberships(&currency, MembershipType::In(), source.get("joiners")?)?;
    let actives = parse_memberships(&currency, MembershipType::In(), source.get("actives")?)?;
    let leavers = parse_memberships(&currency, MembershipType::Out(), source.get("actives")?)?;
    let mut transactions = Vec::new();
    for json_tx in source.get("transactions")?.as_array()? {
        transactions.push(TxDocOrTxHash::TxDoc(Box::new(parse_transaction(
            "g1", &json_tx,
        )?)));
    }
    let block_doc = BlockDocument {
        nonce: source.get("nonce")?.as_i64()? as u64,
        number: BlockId(source.get("number")?.as_u64()? as u32),
        pow_min: source.get("powMin")?.as_u64()? as usize,
        time: source.get("time")?.as_u64()?,
        median_time: source.get("medianTime")?.as_u64()?,
        members_count: source.get("membersCount")?.as_u64()? as usize,
        monetary_mass: source.get("monetaryMass")?.as_u64()? as usize,
        unit_base: source.get("unitbase")?.as_u64()? as usize,
        issuers_count: source.get("issuersCount")?.as_u64()? as usize,
        issuers_frame: source.get("issuersFrame")?.as_i64()? as isize,
        issuers_frame_var: source.get("issuersFrameVar")?.as_i64()? as isize,
        currency: CurrencyName(currency),
        issuers: vec![issuer],
        signatures: vec![sig],
        hash: Some(BlockHash(hash)),
        parameters,
        previous_hash,
        previous_issuer,
        inner_hash,
        dividend,
        identities,
        joiners,
        actives,
        leavers,
        revoked: Vec::with_capacity(0),
        excluded: parse_exclusions_from_json_value(&source.get("excluded")?.as_array()?),
        certifications: Vec::with_capacity(0),
        transactions,
        inner_hash_and_nonce_str: format!(
            "InnerHash: {}\nNonce: {}\n",
            inner_hash
                .expect("Try to get inner_hash of an uncompleted or reduce block !")
                .to_hex(),
            source.get("nonce")?.as_u64()?
        ),
    };
    Some(NetworkBlock::V10(Box::new(NetworkBlockV10 {
        uncompleted_block_doc: block_doc,
        revoked: source.get("revoked")?.as_array()?.clone(),
        certifications: source.get("certifications")?.as_array()?.clone(),
    })))
}
