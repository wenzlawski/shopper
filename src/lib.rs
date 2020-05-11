use std::io;
use std::process;

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

    fn _get_mut_shop(&mut self, index: usize) -> Option<&mut Shop> {
        self.shops.get_mut(index)
    }

    fn _give_shop(&mut self, shop: Shop) {
        self.shops.push(shop);
    }

    fn _find_shop(&self, sh_name: &str) -> Option<(&Shop, usize)> {
        for (index, shop) in self.shops.iter().enumerate() {
            if shop.name == sh_name {
                return Some((shop, index));
            }
        }
        None
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

    fn buy_basket(&mut self, basket: &mut Basket) -> Result<(), &str> {
        let mut tot_cost: f32 = 0.0;
        let mut tot_size: u32 = 0;

        for b in &basket.basket {
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

    fn _update_inventory(&mut self, basket: &mut Basket) -> &Vec<Item> {
        for b in &mut basket.basket[..].iter() {
            for i in &mut self.inventory {
                if b.name == i.name {
                    i.quantity += b.quantity;
                    break;
                }
            }
            self.inventory.push(b.clone());
        }
        basket.basket = vec![];
        &self.inventory
    }
}

#[derive(Debug)]
struct Shop {
    name: String,
    inventory: Vec<Item>,
}

impl Shop {
    fn _new(name: String, inventory: Vec<Item>) -> Shop {
        Shop {
            name: name.split_whitespace().collect::<Vec<&str>>().join(" "),
            inventory,
        }
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

    fn _put_item_back(&mut self, item: &Item) {
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

struct Basket {
    basket: Vec<Item>,
    basket_capacity: u32,
}

impl Basket {
    fn new(basket_capacity: u32) -> Basket {
        Basket {
            basket: vec![],
            basket_capacity,
        }
    }

    fn add(&mut self, item: Item) {
        self.basket.push(item);
    }

    fn remove(&self, item: Item) {}
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

fn base_loop(shopper: &mut Shopper, mall: &mut Mall) {
    loop {
        let input = get_user_input("[go 'shop'] in shop, display all [shops], [exit] the game, display [stats]");
        let input: Vec<&str> = input.split_whitespace().collect();
        match input.first() {
            // go into shop
            Some(&"go") => {
                if input.len() < 2 {
                    println!("Usage: go [shop name]");
                    continue;
                } else {
                    let sh_name = input[1..].join(" ");
                    match mall._find_shop(&sh_name) {
                        Some((shop, index)) => {
                            in_shop(
                                shopper,
                                mall._get_mut_shop(index).unwrap(),
                            );
                        }
                        None => {
                            println!("Shop not found.");
                            continue;
                        }
                    }
                }
            }
            // display all shops
            Some(&"shops") => println!("{:?}", &mall.shops),
            // exit the game in ordered fashion. Display summary statistics
            Some(&"exit") => {
                print_stats(&shopper);
                break;
            }
            // display current game statistics
            Some(&"stats") => print_stats(&shopper),
            None => {
                println!("Please input an option.");
                continue;
            }
            _ => {
                println!("Wrong input.");
                continue;
            }
        }
    }
}

// loop to add items to modify the basket while in a shop
fn in_shop(shopper: &mut Shopper, shop: &mut Shop) {
    // Remove item from basket and add it back to shop inventory
    //fn _put_back(&mut self, item: Item) {
    //let found: usize =
    //find_item_in_inventory(&self.shop.inventory, &item.name)[0];
    //let mut found_item = self.shop.inventory.get_mut(found).unwrap();

    //found_item.quantity += item.quantity;
    //// TODO implement removal of item from buffer
    //unimplemented!();
    //}

    // Return whole basket and leave shop
    //fn leave_without_buying(&self) {}

    // Return an item you bought from the store
    // fn return_item(&self);

    // Steal basket - no change in money, but basket size <= shopper capacity
    // percent probability of getting caught -- lose
    // fn steal(&self);
    let mut basket = Basket::new(53); // shop.basket_capacity (u32) for each shop
    loop {
        // TODO add management to put item back => match
        // IDEA keywords 'leave', 'buy', 'steal' to perform respective actions
        match &get_user_input(
            "[add] another item to basket, [return] item in basket, [shopinv] to display shop inventory, [leave] to leave, [buy] to buy",
        )[..]
        {
            "add" => {
                // get item details
                let item_name = get_user_input(
                    "Please type the name of an item to add it to your basket.",
                );
                let item_quanity = parse_u32("Please enter the quantity.");

                // find entered item details in the shop and add them to basket if found
                match shop._take_item(&item_name, &item_quanity) {
                    Ok(i) => basket.add(i),
                    Err(e) => {
                        println!("{}", e);
                        continue;
                    }
                }
            }
            "return" => unimplemented!("Still need to add return logic"), // add logic to put back here
            "basket" => println!("{:?}", basket.basket),
            "shopinv" => println!("{:?}", shop.inventory),
            "leave" => {
                return_basket(&mut basket, shop);
                return;
            },
            "buy" => {
                match shopper.buy_basket(&mut basket) {
                    Ok(_) => {},
                    Err(e) => {
                        println!("{}", e);
                        continue;
                    }
                }
            }
            "steal" => unimplemented!("Still need to add steal logic"),
            _ => continue,
        }
    }
}

fn return_basket(basket: &mut Basket, shop: &mut Shop) {
    for item in &basket.basket {
        shop._put_item_back(item);
    }
    basket.basket = vec![];
}

fn exit_game(shopper: &Shopper) {
    print_stats(&shopper);
    process::exit(1);
}

fn print_stats(shopper: &Shopper) {
    println!("inventory = {:#?}", &shopper.inventory);
    println!("money = {}", &shopper.money);
    println!("capacity = {}", &shopper.capacity);
}

pub fn game() {
    let (mut shopper, mut mall) = init_game();
    // start prompt has to be recurring to be able to reselect whenever you need to go in a shop
    // start prompt needs to have: 'go_in_shop', 'exit' and 'stats' (display current statistics)
    base_loop(&mut shopper, &mut mall);
}
