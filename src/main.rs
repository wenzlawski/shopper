#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_shopper() {
        let shopper = Shopper::_new(100.0, 100);

        assert_eq!(shopper.money, 100.0);
        assert_eq!(shopper.capacity, 100);
    }

    #[test]
    fn create_shop() {
        let item1 = Item::_new(String::from("Bread"), 6.5, 5, 20);
        let item2 = Item::_new(String::from("Jeans"), 63.0, 8, 6);
        let inv = vec![item1, item2];
        let _shop = Shop::_new(String::from("BreadnJeans"), inv);
    }

    #[test]
    fn shopper_buy_basket() {
        //let item1 = Item::_new(String::from("Bread"), 6.5, 5, 2);
        //let item2 = Item::_new(String::from("Jeans"), 63.0, 8, 6);
        //let inv = vec![item1, item2];
        //let mut shopper = Shopper::_new(5000.0, 100);

        //match shopper._buy(inv) {
        //Ok(_) => println!("Purchase successful!"),
        //Err(e) => println!("{}", e),
        //}

        //assert_eq!(shopper.inventory.len(), 2);
        //assert_eq!(shopper.capacity, 42);
        //assert_eq!(shopper.money, 4609.0);
    }

    #[test]
    fn shopper_buy_from_shop() {
        //let shop = _create_generic_shop();
        //let mut shopper = Shopper::_new(5000.0, 100);

        //match shopper._buy(inv) {
        //Ok(_) => println!("Purchase successful!"),
        //Err(e) => println!("{}", e),
        //}

        //assert_eq!(shopper.inventory.len(), 2);
        //assert_eq!(shopper.capacity, 42);
        //assert_eq!(shopper.money, 4609.0);
    }

    #[test]
    fn take_items_in_shop() {}
}

fn _create_generic_shop() -> Shop {
    let item1 = Item::_new(String::from("Bread"), 6.5, 5, 20);
    let item2 = Item::_new(String::from("Jeans"), 63.0, 20, 9);
    let item3 = Item::_new(String::from("Necklace"), 30.0, 5, 6);
    let item4 = Item::_new(String::from("Painting"), 200.0, 50, 2);
    let item5 = Item::_new(String::from("Gold"), 2000.0, 80, 0);
    let inv = vec![item1, item2, item3, item4, item5];
    Shop::_new(String::from("Best Shop ever"), inv)
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

struct Mall {
    shops: Vec<Shop>,
}

impl Mall {
    fn _new(shops: Vec<Shop>) -> Mall {
        Mall { shops }
    }

    fn get_shops(&mut self) -> &Vec<Shop> {
        &self.shops
    }

    fn take_shop(&mut self, name: String) -> Result<Shop, &str> {
        for (index, shop) in &mut self.shops.iter().enumerate() {
            if shop.name == name {
                return Ok(self.shops.remove(index));
            }
        }
        Err("shop not found.")
    }

    fn give_shop(&mut self, shop: Shop) {
        self.shops.push(shop);
    }
}

//trait Actions {}

struct Shopper {
    money: f32,
    capacity: u32,
    inventory: Vec<Item>,
    current_location: Option<Shop>,
}

impl Shopper {
    fn _new(money: f32, capacity: u32) -> Shopper {
        Shopper {
            money,
            capacity,
            inventory: vec![],
            current_location: None,
        }
    }

    // enter shop - this changes trait to InShop
    //fn go_into_shop(
    //&mut self,
    //shop_name: String,
    //mall: &mut Mall,
    //) -> Result<ShopperInShop, &str> {
    //match mall.take_shop(shop_name) {
    //Ok(s) => {
    //Ok(ShopperInShop {
    //money: self.money,
    //capacity: self.capacity,
    //inventory: self.inventory,
    //current_location: Some(s),
    //})
    //}
    //Err(e) => return Err(e),
    //}
    //Ok(ShopperInShop {
    //money: self.money,
    //capacity: self.capacity,
    //inventory: self.inventory,
    //current_location: Some(shop),
    //})
    //}

    // exit the game and display end statistics
    //fn go_home(&self) {}
}

struct ShopperInShop {
    money: f32,
    capacity: u32,
    inventory: Vec<Item>,
    current_location: Option<Shop>,
}

impl ShopperInShop {
    fn _buy(&mut self, basket: Vec<Item>) -> Result<(), &str> {
        let mut tot_cost: f32 = 0.0;
        let mut tot_size: u32 = 0;

        // shorten this with map?
        for b in &basket {
            tot_cost += b.quantity as f32 * b.cost;
            tot_size += b.quantity * b.size;
        }

        if tot_cost > self.money {
            return Err("Not enough money");
        } else if tot_size > self.capacity {
            return Err("Not enough space in inventory");
        }

        self.money -= tot_cost;
        self.capacity -= tot_size;

        self._update_inventory(basket);

        Ok(())
    }

    fn _update_inventory(&mut self, basket: Vec<Item>) -> &Vec<Item> {
        let mut addn = vec![];
        for b in basket {
            for i in &mut self.inventory {
                if b.name == i.name {
                    i.quantity += b.quantity;
                    break;
                }
            }
            addn.push(b)
        }
        self.inventory.extend(addn);
        &self.inventory
    }
    // Add item amount to basket and remove from shop inventory
    //fn add_to_basket(&self, quantity: u32) {}

    // buy all items in basket, basket size has
    // to be less/equal than shopper capacity
    // and cost must be lower/equal than shopper money
    //fn buy_basket(&self) {}

    // Remove item from basket and add it back to shop inventory
    //fn put_back(&self) {}

    // Return whole basket and leave shop
    //fn leave_without_buying(&self) {}

    // Return an item you bought from the store
    // fn return_item(&self);

    // Steal basket - no change in money, but basket size <= shopper capacity
    // percent probability of getting caught -- lose
    // fn steal(&self);
}

#[derive(Debug)]
struct Shop {
    name: String,
    inventory: Vec<Item>,
}

impl Shop {
    fn _new(name: String, inventory: Vec<Item>) -> Shop {
        Shop { name, inventory }
    }

    fn take_item(&mut self, item: String, quantity: u32) -> Result<Item, &str> {
        for i in &mut self.inventory {
            if item == i.name {
                if quantity <= i.quantity {
                    i.quantity -= quantity;
                    return Ok(Item::_new(
                        i.name.clone(),
                        i.cost,
                        i.size,
                        quantity,
                    ));
                } else {
                    return Err("Not enough of the item left.");
                }
            } else {
                return Err("Item not found in shop.");
            }
        }
        Err("Something went wrong")
    }

    fn put_item_back(&mut self, item: Item) {
        for i in &mut self.inventory {
            if item.name == i.name {
                i.quantity += item.quantity;
                return;
            }
        }
    }
}

#[derive(Debug)]
struct Item {
    name: String,
    cost: f32,
    size: u32,
    quantity: u32,
}

impl Item {
    fn _new(name: String, cost: f32, size: u32, quantity: u32) -> Item {
        Item {
            name,
            cost,
            size,
            quantity,
        }
    }
}

fn main() {
    // Functions needed:

    //let money: f32 = 100.0;
    //let capacity: u32 = 100;

    //let shopper = Shopper::_new(money, capacity);
    //println!("{:?}", shopper);

    //let cost: f32 = 30.0;
    //let size: u32 = 30;

    //let item = Item::_new("Cho"cost, size);
    //println!("{:?}", item);
}
//println!("Test");
