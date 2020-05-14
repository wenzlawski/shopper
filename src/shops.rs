use rand::seq::SliceRandom;
//use rand::Rng;
use std::collections::HashMap;
use std::fs;

use lib;

fn load_file(filename: String) -> String {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    contents
}

fn format_file(contents: &str) -> Vec<&str> {
    contents.split_terminator("\n").collect::<Vec<&str>>()
}

fn turn_into_hash(contents: Vec<&str>) -> HashMap<&str, Vec<&str>> {
    let mut ident = true;
    let mut hm: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut start: &str = "";
    let mut vals = vec![];
    for e in contents {
        if ident == true {
            start = e;
            ident = false;
        } else if e == "" {
            ident = true;
            hm.insert(start, vals);
            vals = vec![];
        } else {
            vals.push(e);
        }
    }
    hm.insert(start, vals);
    hm
}

fn assettify(path: &str) -> String {
    format!("assets/{}", path)
}

fn random_slice<'a>(vector: &'a Vec<&str>, sample: usize) -> Vec<&'a str> {
    let mut rng = &mut rand::thread_rng();
    vector
        .choose_multiple(&mut rng, sample)
        .cloned()
        .collect::<Vec<&str>>()
}

fn gen_shop_inv(shop_type: &str) -> Vec<lib::Item> {}

pub fn main() {
    //let mut rng = &mut rand::thread_rng();
    ////let path1 = assettify("shoes.txt");
    //let path2 = assettify("shops.txt");
    ////let co = load_file(path1);
    ////let ss = format_file(&co);
    ////let hm1 = turn_into_hash(ss);
    //let co = load_file(path2);
    //let ss = format_file(&co);
    //let hm2 = turn_into_hash(ss);
    //let hh = hm2.get("Shoes").unwrap();
    //let sel = random_slice(hh, 2);
    //let shops = vec![];
    //for s in sel {
    //let num_items: u32 = rng.gen_range(1, 10);
    //let inv = populate
    ////let shop = Shop::new(s)
    //}
    let ii = lib::Item::new("Shop");
}
