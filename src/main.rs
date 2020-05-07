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
    fn find_items_in_shop() {
        let shop = _create_generic_shop();
        let found: Vec<&Item> = shop._get_item_by_name("Jeans");

        assert_eq!(found[0].name, "Jeans");
        assert_eq!(found.len(), 2);
    }

    #[test]
    fn go_into_shop_and_buy() {
        let mut mall = _create_generic_mall();
        let shopper = Shopper::_new(1000.0, 300);

        let mut shopper = shopper
            ._go_into_shop(String::from("Best Shop ever"), &mut mall)
            .unwrap();

        shopper._take_item(String::from("Jeans"), 3).unwrap();
        shopper._take_item(String::from("Necklace"), 1).unwrap();

        shopper._buy_basket().unwrap();

        assert_eq!(shopper.inventory[0].name, String::from("Jeans"));
        assert_eq!(shopper.inventory.len(), 2);
        assert_eq!(shopper.money, 781.0);
        assert_eq!(shopper.capacity, 235);
        //assert_eq!(shopper.cur)
    }
}

fn _create_generic_shop() -> Shop {
    let item1 = Item::_new(String::from("Bread"), 6.5, 5, 20);
    let item2 = Item::_new(String::from("Jeans"), 63.0, 20, 9);
    let item21 = Item::_new(String::from("Jeans"), 63.0, 20, 9);
    let item3 = Item::_new(String::from("Necklace"), 30.0, 5, 6);
    let item4 = Item::_new(String::from("Painting"), 200.0, 50, 2);
    let item5 = Item::_new(String::from("Gold"), 2000.0, 80, 0);
    let inv = vec![item1, item2, item21, item3, item4, item5];
    Shop::_new(String::from("Best Shop ever"), inv)
}

fn _create_generic_mall() -> Mall {
    let shop1 = _create_generic_shop();
    let shop2 = _create_generic_shop();
    let shops = vec![shop1, shop2];
    Mall::_new(shops)
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

    fn _get_shops(&mut self) -> &Vec<Shop> {
        &self.shops
    }

    fn _take_shop(&mut self, name: String) -> Result<Shop, &str> {
        for (index, shop) in &mut self.shops.iter().enumerate() {
            if shop.name == name {
                return Ok(self.shops.remove(index));
            }
        }
        Err("shop not found.")
    }

    fn _give_shop(&mut self, shop: Shop) {
        self.shops.push(shop);
    }
}

//trait Actions {}

struct Shopper {
    money: f32,
    capacity: u32,
    inventory: Vec<Item>,
}

impl Shopper {
    fn _new(money: f32, capacity: u32) -> Shopper {
        Shopper {
            money,
            capacity,
            inventory: vec![],
        }
    }

    // enter shop - this changes trait to InShop
    fn _go_into_shop<'a>(
        self,
        shop_name: String,
        mall: &'a mut Mall,
    ) -> Result<ShopperInShop, &str> {
        // Problem here is possibly that the value we get from mall doesn't
        // live as long as the returned ShopperInShop;
        // Possible solution would be to work with lifetime annotations or
        // rethink how the ShopperInShop and its relation to Shop is handeled.
        let shop = mall._take_shop(shop_name);

        let res = match shop {
            Ok(s) => Ok(ShopperInShop {
                money: self.money,
                capacity: self.capacity,
                inventory: self.inventory,
                basket: vec![],
                current_location: s,
            }),
            Err(e) => Err(e),
        };
        res

        //Ok(ShopperInShop {
        //money: self.money,
        //capacity: self.capacity,
        //inventory: self.inventory,
        //basket: vec![],
        //current_location: shop,
        //})
    }

    // exit the game and display end statistics
    //fn go_home(&self) {}
}

struct ShopperInShop {
    money: f32,
    capacity: u32,
    inventory: Vec<Item>,
    basket: Vec<Item>,
    current_location: Shop,
}

impl ShopperInShop {
    fn _buy_basket(&mut self) -> Result<(), &str> {
        let mut tot_cost: f32 = 0.0;
        let mut tot_size: u32 = 0;

        // shorten this with map?
        for b in &self.basket {
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

        self._update_inventory();

        Ok(())
    }

    fn _take_item(&mut self, item: String, quantity: u32) -> Result<(), &str> {
        let item = self.current_location._take_item(item, quantity)?;
        self.basket.push(item);
        Ok(())
    }

    fn _update_inventory(&mut self) -> &Vec<Item> {
        for b in &mut self.basket[..].iter() {
            for i in &mut self.inventory {
                if b.name == i.name {
                    i.quantity += b.quantity;
                    break;
                }
            }
            self.inventory.push(b.clone());
        }
        self.basket = vec![];
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

    fn _get_inventory(&self) -> &Vec<Item> {
        &self.inventory
    }

    fn _get_item_by_name(&self, name: &str) -> Vec<&Item> {
        self.inventory.iter().filter(|x| x.name == name).collect()
    }

    fn _take_item(
        &mut self,
        item: String,
        quantity: u32,
    ) -> Result<Item, &str> {
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
            }
        }
        Err("Item not found in shop.")
    }

    fn _put_item_back(&mut self, item: Item) {
        for i in &mut self.inventory {
            if item.name == i.name {
                i.quantity += item.quantity;
                return;
            }
        }
    }
}

#[derive(Debug, Clone)]
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

fn main() {}
