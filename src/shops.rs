use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;
use std::fs;

use crate::structs::*;

fn load_file(filename: String) -> String {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    contents
}

fn format_file(contents: String) -> Vec<String> {
    contents
        .split_terminator("\n")
        .map(|s| s.to_owned())
        .collect::<Vec<String>>()
}

struct ShopType {
    category_name: String,
    product_path: String,
    product_fields: Vec<String>,
    base_multiplier: Multiplier,
    shop_names: Vec<String>,
}

impl ShopType {
    fn _new(
        category_name: String,
        product_path: String,
        product_fields: Vec<String>,
        base_multiplier: Multiplier,
        shop_names: Vec<String>,
    ) -> Self {
        Self {
            category_name,
            product_path,
            product_fields,
            base_multiplier,
            shop_names,
        }
    }

    fn default() -> Self {
        Self {
            category_name: "".to_owned(),
            product_path: "".to_owned(),
            product_fields: vec![],
            base_multiplier: Multiplier::default(),
            shop_names: vec![],
        }
    }
}

struct Multiplier {
    money: f32,
    capacity: f32,
}

impl Multiplier {
    fn _new(money: f32, capacity: f32) -> Self {
        Self { money, capacity }
    }

    fn default() -> Self {
        Self {
            money: 0.0,
            capacity: 0.0,
        }
    }

    fn set_vals(&mut self, v: Vec<f32>) {
        if v.len() != 2 {
            panic!("len of v has to be 2")
        }
        self.money = *v.get(0).unwrap();
        self.capacity = *v.get(1).unwrap();
    }
}

struct ShopTypeDir {
    shop_types: Vec<ShopType>,
}

impl ShopTypeDir {
    fn _new(shop_types: Vec<ShopType>) -> Self {
        Self { shop_types }
    }

    fn empty() -> Self {
        Self { shop_types: vec![] }
    }

    fn get(&self, k: &str) -> Option<&ShopType> {
        for shop in &self.shop_types {
            if shop.category_name == k {
                return Some(&shop);
            }
        }
        None
    }

    fn get_idx(&self, i: usize) -> Option<&ShopType> {
        self.shop_types.get(i)
    }

    fn add(&mut self, s: ShopType) {
        self.shop_types.push(s);
    }
}

fn turn_into_hash(contents: Vec<String>) -> HashMap<String, Vec<String>> {
    let mut first = true;
    let mut hm = HashMap::new();
    let mut start = String::new();
    let mut vals = vec![];
    for e in contents {
        if first == true {
            start = e;
            first = false;
        } else if e == "" {
            first = true;
            hm.insert(start, vals);
            vals = vec![];
            start = String::new();
        } else {
            vals.push(e);
        }
    }
    hm
}

fn hash_to_typedir(hm: HashMap<String, Vec<String>>) -> ShopTypeDir {
    let mut shop_type = ShopType::default();
    let mut type_dir = ShopTypeDir::empty();
    for (k, v) in hm.iter() {
        let s = k.split("-").collect::<Vec<&str>>();
        if s.len() != 4 {
            panic!("Has to have 4 parts")
        }
        // add category_name
        shop_type.category_name = s.get(0).unwrap().to_string();
        // add product_path
        shop_type.product_path = s.get(2).unwrap().to_string();

        // add product_fields
        shop_type.product_fields = s
            .get(3)
            .unwrap()
            .split("-")
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();

        // add modifier
        shop_type.base_multiplier.set_vals(
            s.get(1)
                .unwrap()
                .split(",")
                .map(|x| x.parse::<f32>().unwrap())
                .collect(),
        );

        // add shop_names
        shop_type.shop_names = v.to_vec();
        type_dir.add(shop_type);
        shop_type = ShopType::default();
    }
    type_dir
}

fn assettify(path: &str) -> String {
    format!("assets/{}", path)
}

fn random_slice(vector: &Vec<String>, sample: usize) -> Vec<String> {
    let mut rng = &mut rand::thread_rng();
    vector
        .choose_multiple(&mut rng, sample)
        .cloned()
        .collect::<Vec<String>>()
}

fn random_item(name: &str, in_money: &f32, in_capacity: &u32) -> Item {
    let mut rng = rand::thread_rng();
    let cost: f32 = rng.gen_range(0.05 * in_money, 0.3 * in_money);
    let size: u32 = rng
        .gen_range(0.05 * *in_capacity as f32, 0.2 * *in_capacity as f32)
        as u32;
    let quantity: u32 = rng.gen_range(5, 30);
    Item::new(name.to_string(), cost, size, quantity)
}

//fn gen_shop_inv(
//selection: Vec<&str>,
//in_money: &f32,
//in_capacity: &u32,
//) -> Vec<Item> {
//let num_items = rand::thread_rng().gen_range(5, 10);
//let selected = random_slice(&selection, num_items);
//let mut items: Vec<Item> = vec![];
//for s in selected {
//items.push(random_item(s, in_money, in_capacity));
//}
//items
//}

pub fn gen_rd_shop_names(cat: &str, n: usize) -> Vec<String> {
    let path = assettify("shops.txt");
    let co = load_file(path);
    let ss = format_file(co);
    let hm = turn_into_hash(ss);
    let hm = hash_to_typedir(hm);
    let hh = &hm.get(cat).unwrap();
    random_slice(&hh.shop_names, n)
}

pub fn main() {
    let mut rng = rand::thread_rng();
    let path = assettify("shops.txt");
    let co = load_file(path);
    let ss = format_file(co);
    let hm = turn_into_hash(ss);
    let hm = hash_to_typedir(hm);
    let hh = &hm.get("Beauty").unwrap();
    let sel = random_slice(&hh.shop_names, 2);
    println!("{:?}", sel);
}
