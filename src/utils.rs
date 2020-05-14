use crate::structs::*;
use std::io;

pub fn create_generic_shop() -> Shop {
    let item1 = Item::new(String::from("Bread"), 6.5, 5, 20);
    let item2 = Item::new(String::from("Jeans"), 63.0, 20, 9);
    let item21 = Item::new(String::from("Jeans"), 63.0, 20, 9);
    let item3 = Item::new(String::from("Necklace"), 30.0, 5, 6);
    let item4 = Item::new(String::from("Painting"), 200.0, 50, 2);
    let item5 = Item::new(String::from("Gold"), 2000.0, 80, 0);
    let inv = vec![item1, item2, item21, item3, item4, item5];
    Shop::new(String::from("Best Shop ever"), inv, 322)
}

pub fn create_generic_mall() -> Mall {
    let shop1 = create_generic_shop();
    let shop2 = create_generic_shop();
    let shops = vec![shop1, shop2];
    Mall::new(shops)
}

pub fn find_item_in_inventory<'b>(
    inv: &'b Vec<Item>,
    name: &str,
) -> Result<Vec<usize>, &'b str> {
    let mut res: Vec<usize> = vec![];
    for (index, item) in inv.iter().enumerate() {
        if item.get_name() == name {
            res.push(index);
        }
    }
    if res.len() == 0 {
        return Err("Item not found");
    } else {
        return Ok(res);
    }
}

pub fn get_user_input(prompt: &str) -> String {
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

pub fn parse_f32(prompt: &str) -> f32 {
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

pub fn parse_u32(prompt: &str) -> u32 {
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

pub fn init_game() -> (Shopper, Mall) {
    let money = parse_f32("Starting money:");

    let capacity = parse_u32("Starting capacity:");

    let mall = create_generic_mall();
    let shopper = Shopper::new(money, capacity);
    (shopper, mall)
}

pub fn return_basket(basket: &mut Basket, shop: &mut Shop) {
    for item in &basket.basket {
        shop.put_item_back(item).unwrap();
    }
    basket.basket = vec![];
}

pub fn print_stolen(shopper: &Shopper) {
    let mut stolen_val = 0.0;
    let mut stolen_cap = 0;
    let mut stolen_goods = vec![];
    for item in &shopper.inventory {
        if *item.get_is_stolen() {
            stolen_goods.push(item);
            stolen_cap += item.get_size() * item.get_quantity();
            stolen_val += item.get_cost() * *item.get_quantity() as f32;
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
            (stolen_cap as f32 / shopper.cal_taken_capacity() as f32) * 100.0
        );
    }
}

pub fn print_stats(shopper: &Shopper) {
    println!("inventory = {:#?}", &shopper.inventory);
    println!("money = ${:.2}", shopper.get_money());
    println!(
        "remaining capacity = {:.1}%",
        (shopper.cal_remaining_capacity() as f32
            / *shopper.get_capacity() as f32)
            * 100.0
    );
    print_stolen(shopper);
}
