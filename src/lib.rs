pub mod storage;
mod user;

pub use storage::NodeKey;
pub use user::{block_producer, User};

pub type H256 = usize;

#[derive(Clone, Debug)]
pub struct ValueNode {
    key: NodeKey,

    owner: H256,
    parent: Option<NodeKey>,

    value: usize,

    finished: bool,
    child_counter: usize,
}

impl ValueNode {
    pub fn create(owner: H256, key: NodeKey, value: usize) -> Self {
        let user = storage::get_user(&owner).expect("Can't fail");
        user.reserve(value).expect("Can't fail");

        Self::create_with_parent(owner, key, value, None);

        storage::get_node(&key).expect("Can't fail")
    }

    fn create_with_parent(owner: H256, key: NodeKey, value: usize, parent: Option<NodeKey>) {
        let node = Self {
            owner,
            key,
            value,
            parent,
            finished: false,
            child_counter: 0,
        };

        println!("Created new node: {:?}", node);
        let _ = storage::set_node(node);
    }

    pub fn get(key: &NodeKey) -> Option<ValueNode> {
        storage::get_node(key)
    }

    pub fn burn(&mut self, amount: usize) {
        if self.value < amount {
            panic!("Should be checked before");
        }

        let owner = storage::get_user(&self.owner).expect("Can't fail for the tests");

        owner
            .repatriate_reserved(block_producer(), amount)
            .expect("Can't fail for the tests");
        self.value -= amount;
        println!("Burned from node {}: {}", self.key, amount);
        storage::update_node(self);
    }

    pub fn split(&mut self, key: NodeKey, amount: usize) {
        if self.value < amount {
            panic!("Should be checked before");
        }

        println!("Node {} was splitted", self.key);
        Self::create_with_parent(self.owner, key, amount, Some(self.key));
        self.value -= amount;
        self.child_counter += 1;
        storage::update_node(self);
    }

    fn unwind(&mut self) {
        println!("Start unwinding: {:?}", self);
        if self.finished {
            let alone = self.child_counter == 0;

            if alone {
                storage::delete_node(&self.key).expect("Shouldn't fail");
            }

            if let Some(parent) = self.parent {
                let mut parent = storage::get_node(&parent).expect("Should not fail");
                parent.value += self.value;
                if alone {
                    parent.child_counter -= 1;
                } else {
                    self.value = 0;
                    storage::update_node(self);
                }
                storage::update_node(&parent);
                parent.unwind();
            } else if self.value != 0 {
                let owner = storage::get_user(&self.owner).expect("Can't fail for the tests");
                owner
                    .unreserve(self.value)
                    .expect("Can't fail for the tests");
                self.value = 0;
                storage::update_node(self);
            }
        };
        println!("End unwinding: {:?}", self);
    }

    pub fn consume(mut self) {
        println!("Consuming {} node", self.key);
        self.finished = true;
        storage::update_node(&self);
        self.unwind();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn user<'a>(id: &'static H256) -> &'a mut User {
        let user = User::new(*id, 31_000);
        storage::set_user(user).expect("Can't fail");
        let user = storage::get_user(&id).expect("Can't fail");
        user
    }

    #[test]
    fn tests() {
        println!(" * * Initial block_producer: {:?}", block_producer());

        let user = user(&10001);
        println!(" * * * * Initial user: {:?}", user);
        println!();

        let mut node1 = ValueNode::create(user.id, 1, 11_000);
        println!(" * * * * User after first sending: {:?}", user);
        println!();

        node1.burn(1000);

        println!(" * * Block producer after burn: {:?}", block_producer());
        println!(" * * * * User after burn: {:?}", user);
        println!();

        ValueNode::get(&1).expect("Can't fail").split(2, 2000);

        println!(" * Nodes storage: {:#?}", storage::storage());
        println!(" * * * * User after splitting 1 to 2: {:?}", user);
        println!();

        ValueNode::get(&1).expect("Can't fail").split(3, 4000);

        println!(" * Nodes storage: {:#?}", storage::storage());
        println!(" * * * * User after splitting 1 to 3: {:?}", user);
        println!();

        ValueNode::get(&3).expect("Can't fail").split(4, 1500);

        println!(" * Nodes storage: {:#?}", storage::storage());
        println!(" * * * * User after splitting 3 to 4: {:?}", user);
        println!();

        ValueNode::get(&3).expect("Can't fail").split(5, 2000);

        println!(" * Nodes storage: {:#?}", storage::storage());
        println!(" * * * * User after splitting 3 to 5: {:?}", user);

        ValueNode::get(&2).expect("Can't fail").burn(200);
        ValueNode::get(&3).expect("Can't fail").burn(300);
        ValueNode::get(&4).expect("Can't fail").burn(400);
        ValueNode::get(&5).expect("Can't fail").burn(500);

        println!(" * Nodes storage: {:#?}", storage::storage());
        println!(" * * * * User after burnes on node 2, 3, 4, 5: {:?}", user);
        println!(" * * Block producer after burns: {:?}", block_producer());

        ValueNode::get(&5).expect("Can't fail").consume();
        println!(" * Nodes storage after consume: {:#?}", storage::storage());
        println!(" * * * * User after consume: {:?}", user);

        ValueNode::get(&1).expect("Can't fail").consume();
        println!(" * Nodes storage after consume: {:#?}", storage::storage());
        println!(" * * * * User after consume: {:?}", user);

        ValueNode::get(&2).expect("Can't fail").consume();
        println!(" * Nodes storage after consume: {:#?}", storage::storage());
        println!(" * * * * User after consume: {:?}", user);

        ValueNode::get(&4).expect("Can't fail").consume();
        println!(" * Nodes storage after consume: {:#?}", storage::storage());
        println!(" * * * * User after consume: {:?}", user);

        ValueNode::get(&3).expect("Can't fail").consume();
        println!(" * Nodes storage after consume: {:#?}", storage::storage());
        println!(" * * * * User after consume: {:?}", user);
    }

    // scheme:
    //             2
    //           /
    //  user - 1       4
    //           \   /
    //             3
    //               \
    //                 5
}
