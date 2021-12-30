use crate::H256;

static mut BLOCK_PRODUCER: Option<User> = None;

pub fn block_producer() -> &'static mut User {
    unsafe { BLOCK_PRODUCER.get_or_insert_with(Default::default) }
}

#[derive(Default, Debug, Clone)]
pub struct User {
    pub id: H256,
    pub free: usize,
    pub reserved: usize,
}

impl User {
    pub fn new(id: H256, free: usize) -> Self {
        Self {
            id,
            free,
            reserved: 0,
        }
    }

    pub fn reserve(&mut self, amount: usize) -> Result<(), ()> {
        if self.free < amount {
            return Err(());
        };

        self.free -= amount;
        self.reserved += amount;

        Ok(())
    }

    pub fn unreserve(&mut self, amount: usize) -> Result<(), ()> {
        if self.reserved < amount {
            return Err(());
        };

        self.reserved -= amount;
        self.free += amount;

        Ok(())
    }

    pub fn repatriate_reserved(&mut self, dest: &mut Self, amount: usize) -> Result<(), ()> {
        if self.reserved < amount {
            return Err(());
        };

        self.reserved -= amount;
        dest.free += amount;

        Ok(())
    }
}
