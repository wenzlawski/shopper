use std::cmp::Ordering;
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
        let _shop = Shop::_new(String::from("BreadnJeans"), inv, 322);
    }

    #[test]
    fn find_items_in_shop() {
        let shop = _create_generic_shop();
        let found: Vec<usize> =
            find_item_in_inventory(&shop.inventory, "Jeans").unwrap();
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
    Shop::_new(String::from("Best Shop ever"), inv, 322)
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
    basket_capacity: u32,
}

impl Shop {
    fn _new(name: String, inventory: Vec<Item>, basket_capacity: u32) -> Shop {
        Shop {
            name: name.split_whitespace().collect::<Vec<&str>>().join(" "),
            inventory,
            basket_capacity,
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

    fn _put_item_back(&mut self, item: &Item) -> Result<(), &str> {
        self._put_amount_back(item, &item.quantity)?;
        Ok(())
    }

    fn _put_amount_back(
        &mut self,
        item: &Item,
        amount: &u32,
    ) -> Result<(), &str> {
        for i in &mut self.inventory {
            if item.name == i.name {
                i.quantity += amount;
                return Ok(());
            }
        }
        Err("Item not found in shop.")
    }
    fn get_item(&self, item_name: &str) -> Option<&Item> {
        for item in &self.inventory {
            if item.name == item_name {
                return Some(&item);
            }
        }
        None
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

    fn remove(&mut self, item_index: usize) -> Result<Item, &str> {
        if item_index < self.basket.len() {
            return Ok(self.basket.remove(item_index));
        } else {
            return Err("Out of range");
        }
    }
}

fn find_item_in_inventory<'b>(
    inv: &'b Vec<Item>,
    name: &str,
) -> Result<Vec<usize>, &'b str> {
    let mut res: Vec<usize> = vec![];
    for (index, item) in inv.iter().enumerate() {
        if item.name == name {
            res.push(index);
        }
    }
    if res.len() == 0 {
        return Err("Item not found");
    } else {
        return Ok(res);
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
                        Some((_, index)) => {
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
            _ => {
                println!("Please enter a valid option.");
                continue;
            }
        }
    }
}

// loop to add items to modify the basket while in a shop
fn in_shop(shopper: &mut Shopper, shop: &mut Shop) {
    // IDEA Return an item you bought from the store

    let mut basket = Basket::new(53); // shop.basket_capacity (u32) for each shop
    loop {
        match &get_user_input(
            "[add] another item to basket, [return] item in basket, [shopinv] to display shop inventory, [leave] to leave, [buy] to buy",
        )[..]
        {
            "add" => {
                // get item details
                let item_name = get_user_input(
                    "Please type the name of an item to add it to your basket:",
                );
                // get a reference for the found item
                let founditem = match shop.get_item(&item_name) {
                    Some(i) => i,
                    None => {
                        println!("Item not found in shop.");
                        continue;
                    }
                };

                let item_quanity = parse_u32("Please enter the quantity:");
                // check whether the quanitiy is right
                if item_quanity > founditem.quantity {
                    println!("Not enough of the item left.");
                    continue;
                }

                // check whether there's space in the basket
                if item_quanity * founditem.size > basket.basket_capacity {
                    println!("Basket cannot hold the items.");
                    continue;
                }

                // find entered item details in the shop and add them to basket if found
                match shop._take_item(&item_name, &item_quanity) {
                    Ok(i) => basket.add(i),
                    Err(e) => {
                        println!("{}", e);
                        continue;
                    }
                }
            }

            // Remove item from basket and add it back to shop inventory
            "return" => {
                let item_name = get_user_input("Enter the item to return:");
                match find_item_in_inventory(&basket.basket, &item_name){
                    Ok(i) => {
                        let quantity = get_user_input("Enter the amount to return ([all] for everything):");
                        match &quantity[..] {
                            "all" => {
                                let item = basket.remove(*i.first().unwrap()).unwrap();
                                shop._put_item_back(&item).unwrap();
                            },
                            _ => {
                                match quantity.parse::<u32>() {
                                    Ok(n) => {
                                        let founditem = basket.basket.get_mut(*i.first().unwrap()).unwrap();
                                        match founditem.quantity.cmp(&n) {
                                            // return partial amount
                                            Ordering::Greater => {
                                                founditem.quantity -= n;
                                                shop._put_amount_back(founditem, &n).unwrap();
                                            },
                                            // same as 'all'
                                            Ordering::Equal => {
                                                // duplicate at 'all' --> refactor
                                                let item = basket.remove(*i.first().unwrap()).unwrap();
                                                shop._put_item_back(&item).unwrap();
                                            },
                                            // invalid quantity
                                            Ordering::Less => {
                                                println!("Cannot return quantity larger than what is in basket.");
                                                continue;
                                            },
                                        }
                                    }, // _put_item_back works here too
                                    Err(_) => {
                                        println!("Error parsing input.");
                                        continue;
                                    },
                                }
                            },
                        }
                    }
                    Err(e) => {
                        println!("{}", e);
                        continue;
                    }
                }
            },

            // Display basket contents
            "basket" => println!("{:?}", basket.basket),

            // return all items in basket to shop inventory
            "empty basket" => return_basket(&mut basket, shop),

            // Display shop inventory
            "shopinv" => println!("{:?}", shop.inventory),

            // Return whole basket and leave shop
            "leave" => {
                return_basket(&mut basket, shop);
                return;
            },

            // Add contents of basket to inventory
            "buy" => {
                match shopper.buy_basket(&mut basket) {
                    Ok(_) => {},
                    Err(e) => {
                        println!("{}", e);
                        continue;
                    }
                }
            }

            // Steal basket - no change in money, but basket size <= shopper capacity
            // percent probability of getting caught -- lose
            "steal" => unimplemented!("Still need to add steal logic"),
            _ => continue,
        }
    }
}

fn return_basket(basket: &mut Basket, shop: &mut Shop) {
    for item in &basket.basket {
        shop._put_item_back(item).unwrap();
    }
    basket.basket = vec![];
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
