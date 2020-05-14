use rand::Rng;
use std::cmp::Ordering;

mod lib;
mod shops;
mod structs;
mod utils;
use structs::*;
use utils::*;

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
            Some(&"shops") => println!("{:?}", mall.get_shops()),
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

    let mut basket = Basket::new(*shop.get_basket_capacity()); // shop.basket_capacity (u32) for each shop
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
                if item_quanity > *founditem.get_quantity() {
                    println!("Not enough of the item left.");
                    continue;
                }

                // check whether there's space in the basket
                if item_quanity * founditem.get_size() > *basket.get_basket_capacity() {
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
                                        match founditem.get_quantity().cmp(&n) {
                                            // return partial amount
                                            Ordering::Greater => {
                                                // set_quantity(get_quantity - n)
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
            "shopinv" => println!("{:?}", shop.get_inventory()),

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

fn main() {
    //lib::game();
    shops::main();
}
