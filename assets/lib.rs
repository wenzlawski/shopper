use rand::Rng;
use std::cmp::Ordering;
use std::io;
extern crate rand;

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
    fn find_items_in_shop() {
        let shop = create_generic_shop();
        let found: Vec<usize> =
            find_item_in_inventory(&shop.inventory, "Jeans").unwrap();
        let f_item = &shop.inventory[found[0]];

        assert_eq!(f_item.name, "Jeans");
        assert_eq!(found.len(), 2);
    }
}

fn create_generic_shop() -> Shop {
    let item1 = Item::new(String::from("Bread"), 6.5, 5, 20);
    let item2 = Item::new(String::from("Jeans"), 63.0, 20, 9);
    let item21 = Item::new(String::from("Jeans"), 63.0, 20, 9);
    let item3 = Item::new(String::from("Necklace"), 30.0, 5, 6);
    let item4 = Item::new(String::from("Painting"), 200.0, 50, 2);
    let item5 = Item::new(String::from("Gold"), 2000.0, 80, 0);
    let inv = vec![item1, item2, item21, item3, item4, item5];
    Shop::new(String::from("Best Shop ever"), inv, 322)
}

fn create_generic_mall() -> Mall {
    let shop1 = create_generic_shop();
    let shop2 = create_generic_shop();
    let shops = vec![shop1, shop2];
    Mall::new(shops)
}

struct Mall {
    shops: Vec<Shop>,
}

impl Mall {
    // creates new mall
    fn new(shops: Vec<Shop>) -> Mall {
        Mall { shops }
    }

    // get mutable refrence to shop
    fn get_mut_shop(&mut self, index: usize) -> Option<&mut Shop> {
        self.shops.get_mut(index)
    }

    fn find_shop(&self, sh_name: &str) -> Option<(&Shop, usize)> {
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
    fn new(money: f32, capacity: u32) -> Shopper {
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
        } else if tot_size > self.remaining_capacity() {
            return Err("Not enough space in inventory");
        }

        self.money -= tot_cost;
        //self.capacity -= tot_size;

        self.update_inventory(basket, false);

        Ok(())
    }

    fn update_inventory(
        &mut self,
        basket: &mut Basket,
        stolen: bool,
    ) -> &Vec<Item> {
        for b in &mut basket.basket[..].iter_mut() {
            for i in &mut self.inventory {
                if b.name == i.name {
                    i.quantity += b.quantity;
                    break;
                }
            }
            if stolen {
                b.is_stolen = true
            }
            self.inventory.push(b.clone());
        }
        basket.basket = vec![];
        &self.inventory
    }

    fn remaining_capacity(&self) -> u32 {
        self.capacity - self.taken_capacity()
    }

    fn taken_capacity(&self) -> u32 {
        self.inventory
            .iter()
            .map(|x| x.size * x.quantity)
            .sum::<u32>()
    }
}

#[derive(Debug)]
struct Shop {
    name: String,
    inventory: Vec<Item>,
    basket_capacity: u32,
}

pub impl Shop {
    pub fn new(
        name: String,
        inventory: Vec<Item>,
        basket_capacity: u32,
    ) -> Shop {
        Shop {
            name: name.split_whitespace().collect::<Vec<&str>>().join(" "),
            inventory,
            basket_capacity,
        }
    }

    fn take_item(
        &mut self,
        item: &String,
        quantity: &u32,
    ) -> Result<Item, &str> {
        for i in &mut self.inventory {
            if *item == i.name {
                if *quantity <= i.quantity {
                    i.quantity -= *quantity;
                    return Ok(Item::new(
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

    fn put_item_back(&mut self, item: &Item) -> Result<(), &str> {
        self.put_amount_back(item, &item.quantity)?;
        Ok(())
    }

    fn put_amount_back(
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
    is_stolen: bool,
}

pub impl Item {
    pub fn new(name: String, cost: f32, size: u32, quantity: u32) -> Item {
        Item {
            name,
            cost,
            size,
            quantity,
            is_stolen: false,
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

    let mall = create_generic_mall();
    let shopper = Shopper::new(money, capacity);
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
                    match mall.find_shop(&sh_name) {
                        Some((_, index)) => {
                            if let Err(e) = in_shop(
                                shopper,
                                mall.get_mut_shop(index).unwrap(),
                            ) {
                                println!("{}", e);
                                print_stats(shopper);
                                break;
                            }
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
fn in_shop<'a>(
    shopper: &'a mut Shopper,
    shop: &mut Shop,
) -> Result<(), &'a str> {
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
                match shop.take_item(&item_name, &item_quanity) {
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
                                shop.put_item_back(&item).unwrap();
                            },
                            _ => {
                                match quantity.parse::<u32>() {
                                    Ok(n) => {
                                        if n == 0 { continue }
                                        let founditem = basket.basket.get_mut(*i.first().unwrap()).unwrap();
                                        match founditem.quantity.cmp(&n) {
                                            // return partial amount
                                            Ordering::Greater => {
                                                founditem.quantity -= n;
                                                shop.put_amount_back(founditem, &n).unwrap();
                                            },
                                            // same as 'all'
                                            Ordering::Equal => {
                                                // duplicate at 'all' --> refactor
                                                let item = basket.remove(*i.first().unwrap()).unwrap();
                                                shop.put_item_back(&item).unwrap();
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
                return Ok(());
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
            "steal" => {
                let mut rng = rand::thread_rng();
                let y: f64 = rng.gen(); // generates a float between 0 and 1
                let prob_of_getting_caught = 0.1;
                if y < prob_of_getting_caught {
                    return Err("You got caught :( Game over...");
                } else {
                    println!("Success!");
                    // add bool to item if stolen --> for stats
                    shopper.update_inventory(&mut basket, true);
                }
            }
            _ => continue,
        }
    }
}

fn return_basket(basket: &mut Basket, shop: &mut Shop) {
    for item in &basket.basket {
        shop.put_item_back(item).unwrap();
    }
    basket.basket = vec![];
}

fn print_stolen(shopper: &Shopper) {
    let mut stolen_val = 0.0;
    let mut stolen_cap = 0;
    let mut stolen_goods = vec![];
    for item in &shopper.inventory {
        if item.is_stolen {
            stolen_goods.push(item);
            stolen_cap += item.size * item.quantity;
            stolen_val += item.cost * item.quantity as f32;
        }
    }
    if stolen_cap == 0 {
        println!("You haven't stolen anything yet... boooring!")
    } else {
        println!("So far you have stolen:");
        for s in stolen_goods {
            println!("{:?}", s);
        }
        println!("Total value of stolen goods: ${:.2}", &stolen_val);
        println!(
            "Percentage of stolen goods: {:.1}%",
            (stolen_cap as f32 / shopper.taken_capacity() as f32) * 100.0
        );
    }
}

fn print_stats(shopper: &Shopper) {
    println!("inventory = {:#?}", &shopper.inventory);
    println!("money = ${:.2}", &shopper.money);
    println!(
        "remaining capacity = {:.1}%",
        (shopper.remaining_capacity() as f32 / shopper.capacity as f32) * 100.0
    );
    print_stolen(shopper);
}

pub fn game() {
    let (mut shopper, mut mall) = init_game();
    // start prompt has to be recurring to be able to reselect whenever you need to go in a shop
    // start prompt needs to have: 'go_in_shop', 'exit' and 'stats' (display current statistics)
    base_loop(&mut shopper, &mut mall);
}
