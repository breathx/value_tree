use crate::{User, ValueNode, H256};
pub use std::collections::BTreeMap;

pub type NodeKey = usize;

static mut STORAGE: Option<BTreeMap<NodeKey, ValueNode>> = None;

static mut USER_STORAGE: Option<BTreeMap<H256, User>> = None;

pub fn storage() -> &'static mut BTreeMap<NodeKey, ValueNode> {
    unsafe { STORAGE.get_or_insert_with(BTreeMap::new) }
}

fn user_storage() -> &'static mut BTreeMap<H256, User> {
    unsafe { USER_STORAGE.get_or_insert_with(BTreeMap::new) }
}

pub fn delete_node(key: &NodeKey) -> Result<(), ()> {
    if storage().remove(key).is_none() {
        return Err(());
    }

    println!("Node was deleted: {}", key);
    Ok(())
}

pub fn get_node(key: &NodeKey) -> Option<ValueNode> {
    storage().get(key).cloned()
}

pub fn get_user(key: &H256) -> Option<&mut User> {
    user_storage().get_mut(key)
}

pub fn set_node(node: ValueNode) -> Result<(), ()> {
    if storage().insert(node.key, node).is_some() {
        return Err(());
    };

    Ok(())
}

pub fn update_node(node: &ValueNode) {
    if let Some(x) = storage().get_mut(&node.key) {
        *x = node.clone()
    }
}

pub fn set_user(user: User) -> Result<(), ()> {
    if user_storage().insert(user.id, user).is_some() {
        return Err(());
    };

    Ok(())
}
