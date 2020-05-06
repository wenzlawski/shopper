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
    fn get_shops(&self);
    fn go_into_shop(&self);
    fn go_home(&self);
}

trait InShop {
    fn add_to_basket(&self);
    fn buy_basket(&self);
    fn put_back(&self);
    fn leave_without_buying(&self);
    // fn return_item(&self);
    // fn steal(&self); // percent probability of getting caught -- lose
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