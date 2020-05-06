#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_shopper() {
        let shopper = Shopper::new(100.0, 100);

        assert_eq!(shopper.money, 100.0);
        assert_eq!(shopper.capacity, 100);
    }

    #[test]
    fn buy_item() {
        let shopper = Shopper::new(100.0, 100);

        let item = Item::new(12.0, 50);

        let shopper = item.buy(shopper);

        assert_eq!(shopper.money, 88.0);
        assert_eq!(shopper.capacity, 50);

        let shopper = item.buy(shopper);

        assert_eq!(shopper.money, 76.0);
        assert_eq!(shopper.capacity, 0);
    }
}

/*
   Store:
    ** when 'in' store have extra carrying capacity.
    ** pick up items in the store (don't buy -- add to basket)
    ** this *temporarily* changes item count in store
    ** can place items back
    ** to buy basket it has to be smaller/equal in remaining shopper capacity
    ** buying permanently changes item capacity

** Out of store and in store changes traits to add basket to shopper

*/

trait OutShop {
    // get list of all shops
    fn get_shops(&self);

    // enter shop - this changes trait to InShop
    fn go_into_shop(&self);

    // exit the game and display end statistics
    fn go_home(&self);
}

trait InShop {
    // Add item amount to basket and remove from shop inventory
    fn add_to_basket(&self, quantity: u32);

    // buy all items in basket, basket size has
    // to be less/equal than shopper capacity
    // and cost must be lower/equal than shopper money
    fn buy_basket(&self);

    // Remove item from basket and add it back to shop inventory
    fn put_back(&self);

    // Return whole basket and leave shop
    fn leave_without_buying(&self);

    // Return an item you bought from the store
    // fn return_item(&self);

    // Steal basket - no change in money, but basket size <= shopper capacity
    // percent probability of getting caught -- lose
    // fn steal(&self);
}

#[derive(Debug)]
struct Shopper {
    money: f32,
    capacity: u32,
}

impl Shopper {
    fn new(money: f32, capacity: u32) -> Shopper {
        Shopper { money, capacity }
    }
}

#[derive(Debug)]
struct Item {
    cost: f32,
    size: u32,
}

impl Item {
    fn new(cost: f32, size: u32) -> Item {
        Item { cost, size }
    }
    fn buy(&self, shopper: Shopper) -> Shopper {
        Shopper {
            money: shopper.money - self.cost,
            capacity: shopper.capacity - self.size,
        }
    }
}

fn main() {
    // Functions needed:

    let money: f32 = 100.0;
    let capacity: u32 = 100;

    let shopper = Shopper::new(money, capacity);
    println!("{:?}", shopper);

    let cost: f32 = 30.0;
    let size: u32 = 30;

    let item = Item::new(cost, size);
    item.buy(shopper);
    println!("{:?}", item);
}
//println!("Test");
