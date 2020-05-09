use std::io;

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
        let found: Vec<usize> =
            find_item_in_inventory(&shop.inventory, "Jeans");
        let f_item = &shop.inventory[found[0]];

        assert_eq!(f_item.name, "Jeans");
        assert_eq!(found.len(), 2);
    }

    //#[test]
    //fn go_into_shop_and_buy() {
    //let mut mall = _create_generic_mall();
    //let shopper = Shopper::_new(1000.0, 300);

    //let mut shopper = shopper
    //._go_into_shop(String::from("Best Shop ever"), &mut mall)
    //.unwrap();

    //shopper._add_to_basket(String::from("Jeans"), 3).unwrap();
    //shopper._add_to_basket(String::from("Necklace"), 1).unwrap();

    //shopper._buy_basket().unwrap();

    //assert_eq!(shopper.inventory[0].name, String::from("Jeans"));
    //assert_eq!(shopper.inventory.len(), 2);
    //assert_eq!(shopper.money, 781.0);
    //assert_eq!(shopper.capacity, 235);
    ////assert_eq!(shopper.cur)
    //}
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

struct Shopper {
    money: f32,
    capacity: u32,
    inventory: Vec<Item>,
}

impl Shopper {
    // create new shopper
    fn _new(money: f32, capacity: u32) -> Shopper {
        Shopper {
            money,
            capacity,
            inventory: vec![],
        }
    }

    // enter shop - this changes struct to ShopperInShop
    fn _go_into_shop<'a>(self, shop: Shop) -> ShopperInShop {
        ShopperInShop {
            money: self.money,
            capacity: self.capacity,
            inventory: self.inventory,
            basket: vec![],
            shop: shop,
        }
    }

    // exit the game and display end statistics
    fn go_home(&self) {
        println!("inventory = {:?}", &self.inventory);
        println!("money = {:?}", self.money);
        println!("capacity = {:?}", self.capacity);
    }
}

struct ShopperInShop {
    money: f32,
    capacity: u32,
    inventory: Vec<Item>,
    basket: Vec<Item>,
    shop: Shop,
}

impl ShopperInShop {
    // buy all items in basket, basket size has
    // to be less/equal than shopper capacity
    // and cost must be lower/equal than shopper money
    fn _buy_basket(&mut self) -> Result<(), &str> {
        let mut tot_cost: f32 = 0.0;
        let mut tot_size: u32 = 0;

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

    // Add item amount to basket and remove from shop inventory
    fn _add_to_basket(
        &mut self,
        item: &String,
        quantity: &u32,
    ) -> Result<(), &str> {
        let item = self.shop._take_item(item, quantity)?;
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

    // Remove item from basket and add it back to shop inventory
    fn _put_back(&mut self, item: Item) {
        let found: usize =
            find_item_in_inventory(&self.shop.inventory, &item.name)[0];
        let mut found_item = self.shop.inventory.get_mut(found).unwrap();

        found_item.quantity += item.quantity;
        // TODO implement removal of item from buffer
        unimplemented!();
    }

    // Return whole basket and leave shop
    //fn leave_without_buying(&self) {}

    // Return an item you bought from the store
    // fn return_item(&self);

    // Steal basket - no change in money, but basket size <= shopper capacity
    // percent probability of getting caught -- lose
    // fn steal(&self);
}

fn find_item_in_inventory(inv: &Vec<Item>, name: &str) -> Vec<usize> {
    let mut res: Vec<usize> = vec![];
    for (index, item) in inv.iter().enumerate() {
        if item.name == name {
            res.push(index);
        }
    }
    res
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

    fn _take_item(
        &mut self,
        item: &String,
        quantity: &u32,
    ) -> Result<Item, &str> {
        for i in &mut self.inventory {
            if *item == i.name {
                if *quantity <= i.quantity {
                    i.quantity -= *quantity;
                    return Ok(Item::_new(
                        i.name.clone(),
                        i.cost,
                        i.size,
                        *quantity,
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

fn get_user_input(prompt: &str) -> String {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        if let Ok(_) = io::stdin().read_line(&mut input) {
            return input.trim().to_string();
        } else {
            println!("Error reading line. Please try again.");
        }
    }
}

fn parse_f32(prompt: &str) -> f32 {
    loop {
        match get_user_input(prompt).parse::<f32>() {
            Ok(i) => {
                if i > 0.0 {
                    return i;
                } else {
                    println!("Please type a number greater than zero.");
                }
            }
            Err(e) => println!("{}", e),
        }
    }
}

fn parse_u32(prompt: &str) -> u32 {
    loop {
        match get_user_input(prompt).parse::<u32>() {
            Ok(i) => {
                if i > 0 {
                    return i;
                } else {
                    println!("Please type a number greater than zero.");
                }
            }
            Err(_) => println!("Please type a positive number."),
        }
    }
}

fn init_game() -> (Shopper, Mall) {
    let money = parse_f32("Starting money:");

    let capacity = parse_u32("Starting capacity:");

    let mall = _create_generic_mall();
    let shopper = Shopper::_new(money, capacity);
    (shopper, mall)
}

fn go_in_shop(shopper: Shopper, mall: &mut Mall) -> ShopperInShop {
    println!("Choose a shop from this list and enter its name to enter it!");
    println!("{:?}", &mall._get_shops());
    //let mut _shop: Shop;
    loop {
        let input_shop = get_user_input("Which shop would you like to enter?");
        match mall._take_shop(input_shop) {
            Ok(i) => return shopper._go_into_shop(i),
            Err(_) => {
                println!("Please enter a valid shop name.");
                continue;
            }
        };
    }
}

fn basket_loop(shopper: &mut ShopperInShop) {
    loop {
        println!("{:?}", &shopper.shop.inventory);
        let item_name = get_user_input(
            "Please type the name of an item to add it to your basket.",
        );
        let item_quanity = parse_u32("Please enter the quantity.");
        if let Err(e) =
            shopper._add_to_basket(&item_name.to_string(), &item_quanity)
        {
            println!("{}", e);
            continue;
        }
        // TODO add management to put item back => match
        match &get_user_input(
            "[add] another item to basket, [return] item in basket",
        )[..]
        {
            "add" => continue,
            "return" => unimplemented!("Still need to add return logic"), // add logic to put back here
            _ => break,
        }
    }
}

fn game_loop() {
    let (shopper, mut mall) = init_game();
    let mut shopper = go_in_shop(shopper, &mut mall);

    // Let player add items to basket. This removes them from the shop inventory
    basket_loop(&mut shopper);
    println!("{:?}", shopper.basket);
    println!("{:?}", shopper.shop.inventory);
}

fn main() {
    game_loop();
}
