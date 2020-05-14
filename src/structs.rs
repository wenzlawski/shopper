pub struct Mall {
    shops: Vec<Shop>,
}

impl Mall {
    // creates new mall
    pub fn new(shops: Vec<Shop>) -> Mall {
        Mall { shops }
    }

    pub fn get_shops(&self) -> &Vec<Shop> {
        &self.shops
    }

    // get mutable refrence to shop
    pub fn get_mut_shop(&mut self, index: usize) -> Option<&mut Shop> {
        self.shops.get_mut(index)
    }

    pub fn find_shop(&self, sh_name: &str) -> Option<(&Shop, usize)> {
        for (index, shop) in self.shops.iter().enumerate() {
            if shop.name == sh_name {
                return Some((shop, index));
            }
        }
        None
    }
}

pub struct Shopper {
    money: f32,
    capacity: u32,
    pub inventory: Vec<Item>,
}

impl Shopper {
    // create new shopper
    pub fn new(money: f32, capacity: u32) -> Shopper {
        Shopper {
            money,
            capacity,
            inventory: vec![],
        }
    }

    pub fn get_money(&self) -> &f32 {
        &self.money
    }

    pub fn get_capacity(&self) -> &u32 {
        &self.capacity
    }

    pub fn buy_basket(&mut self, basket: &mut Basket) -> Result<(), &str> {
        let mut tot_cost: f32 = 0.0;
        let mut tot_size: u32 = 0;

        for b in &basket.basket {
            tot_cost += b.quantity as f32 * b.cost;
            tot_size += b.quantity * b.size;
        }

        if tot_cost > self.money {
            return Err("Not enough money");
        } else if tot_size > self.cal_remaining_capacity() {
            return Err("Not enough space in inventory");
        }

        self.money -= tot_cost;
        //self.capacity -= tot_size;

        self.update_inventory(basket, false);

        Ok(())
    }

    pub fn update_inventory(
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

    pub fn cal_remaining_capacity(&self) -> u32 {
        &self.capacity - &self.cal_taken_capacity()
    }

    pub fn cal_taken_capacity(&self) -> u32 {
        self.inventory
            .iter()
            .map(|x| x.size * x.quantity)
            .sum::<u32>()
    }
}

#[derive(Debug)]
pub struct Shop {
    name: String,
    inventory: Vec<Item>,
    basket_capacity: u32,
}

impl Shop {
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

    pub fn get_basket_capacity(&self) -> &u32 {
        &self.basket_capacity
    }

    pub fn get_inventory(&self) -> &Vec<Item> {
        &self.inventory
    }

    pub fn take_item(
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

    pub fn put_item_back(&mut self, item: &Item) -> Result<(), &str> {
        self.put_amount_back(item, &item.quantity)?;
        Ok(())
    }

    pub fn put_amount_back(
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
    pub fn get_item(&self, item_name: &str) -> Option<&Item> {
        for item in &self.inventory {
            if item.name == item_name {
                return Some(&item);
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct Item {
    name: String,
    cost: f32,
    size: u32,
    pub quantity: u32,
    pub is_stolen: bool,
}

impl Item {
    pub fn new(name: String, cost: f32, size: u32, quantity: u32) -> Item {
        Item {
            name,
            cost,
            size,
            quantity,
            is_stolen: false,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_size(&self) -> &u32 {
        &self.size
    }

    pub fn get_quantity(&self) -> &u32 {
        &self.quantity
    }

    pub fn get_cost(&self) -> &f32 {
        &self.cost
    }

    pub fn get_is_stolen(&self) -> &bool {
        &self.is_stolen
    }
}

pub struct Basket {
    pub basket: Vec<Item>,
    basket_capacity: u32,
}

impl Basket {
    pub fn new(basket_capacity: u32) -> Basket {
        Basket {
            basket: vec![],
            basket_capacity,
        }
    }

    pub fn get_basket_capacity(&self) -> &u32 {
        &self.basket_capacity
    }

    pub fn add(&mut self, item: Item) {
        self.basket.push(item);
    }

    pub fn remove(&mut self, item_index: usize) -> Result<Item, &str> {
        if item_index < self.basket.len() {
            return Ok(self.basket.remove(item_index));
        } else {
            return Err("Out of range");
        }
    }
}
