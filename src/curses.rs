use cursive::direction::Direction;
use cursive::event::{Event, EventResult, EventTrigger, Key, MouseEvent};
use cursive::theme::BaseColor::*;
use cursive::theme::Color::*;
use cursive::theme::PaletteColor::*;
use cursive::theme::{load_default, BorderStyle, ColorStyle, Theme};
use cursive::views::{Button, LinearLayout, OnEventView};
//use cursive::Cursive;
//use crate::{listview, listview_top};
use cursive::{Printer, Vec2};
use std::{fs, mem};

//struct MyList(LinearLayout);

pub fn gen_board(n: Vec<String>) {
    let mut siv = cursive::default();
    siv.set_theme(generate_theme());
    let shop_display = ShopInfo::new(&n);
    let mut street = generate_street(n, 3);
    street.add_child(shop_display);
    let mut top_layer = LinearLayout::vertical();
    top_layer.add_child(street);
    top_layer.add_child(Button::new("quit", |s| s.quit()));

    siv.add_layer(top_layer);
    siv.run();
}

fn load_file(filename: &str) -> String {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    contents
}

fn format_file(contents: String) -> Vec<String> {
    contents
        .split("\n")
        .map(|x| x.to_owned())
        .collect::<Vec<String>>()
}

fn generate_street(n: Vec<String>, _houses: usize) -> LinearLayout {
    let mut ni = n.iter();
    let mut vert_layout = LinearLayout::vertical();
    let mut linear_layout1 = LinearLayout::horizontal();
    let mut linear_layout2 = LinearLayout::horizontal();
    let street =
        LinearLayout::horizontal().child(Thing::new(false, Elements::Street));
    //for _ in 0..(n.len()/2) {
    for _ in 0..(3 - 1) {
        linear_layout1.add_child(Thing::new(
            false,
            Elements::House(ni.next().unwrap().to_owned()),
        ));
        linear_layout1.add_child(Thing::new(false, Elements::Buffer));
    }

    linear_layout1.add_child(Thing::new(
        false,
        Elements::House(ni.next().unwrap().to_owned()),
    ));

    for _ in 0..(3 - 1) {
        linear_layout2.add_child(Thing::new(
            true,
            Elements::House(ni.next().unwrap().to_owned()),
        ));
        linear_layout2.add_child(Thing::new(true, Elements::Buffer));
    }

    linear_layout2.add_child(Thing::new(
        true,
        Elements::House(ni.next().unwrap().to_owned()),
    ));
    vert_layout.add_child(linear_layout1);
    vert_layout.add_child(street);
    vert_layout.add_child(linear_layout2);
    vert_layout
}

fn generate_theme() -> Theme {
    let mut th = load_default();
    th.borders = BorderStyle::None;
    th.shadow = false;
    // https://docs.rs/cursive/0.14.1/cursive/theme/index.html#palette
    th.palette[Background] = Dark(Red);
    th
}

enum Elements {
    House(String),
    Street,
    Buffer,
}

impl Elements {
    fn value(&self) -> (&'static str, bool) {
        match *self {
            Elements::House(_) => ("house", true),
            Elements::Street => ("street", false),
            Elements::Buffer => ("buffer", false),
        }
    }
}

struct ShopInfo {
    cs: Vec<String>,
    current: usize,
}

impl ShopInfo {
    fn new(cs: &Vec<String>) -> Self {
        Self {
            cs: cs.to_vec(),
            current: 0,
        }
    }
}

impl cursive::view::View for ShopInfo {
    fn draw(&self, printer: &Printer) {
        let style = ColorStyle::secondary();

        printer.with_color(style, |printer| {
            printer.print((0, 0), self.cs.get(self.current).unwrap())
        });
    }

    fn take_focus(&mut self, _: Direction) -> bool {
        false
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Key(e) => match e {
                Key::F9 => {
                    /*self.cs = format!("{:?}", e);*/
                    if self.current > 2 {
                        self.current -= 3;
                        return EventResult::Consumed(None);
                    }
                }

                Key::F8 => {
                    /*self.cs = format!("{:?}", e);*/
                    if self.current < 3 {
                        self.current += 3;
                        return EventResult::Consumed(None);
                    }
                }

                Key::F6 => {
                    /*self.cs = format!("{:?}", e);*/
                    if ![0, 3].contains(&self.current) {
                        self.current -= 1;
                        return EventResult::Consumed(None);
                    }
                }

                Key::F7 => {
                    /*self.cs = format!("{:?}", e);*/
                    if ![2, 5].contains(&self.current) {
                        self.current += 1;
                        return EventResult::Consumed(None);
                    }
                }
                _ => {}
            },

            /*Event::Mouse {*/
            /*offset,*/
            /*position,*/
            /*event: MouseEvent::Press(btn),*/
            /*} => {*/
            /*self.cs = format!("{:?}", btn);*/
            /*}*/
            _ => {}
        }
        EventResult::Ignored
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        Vec2::from((20, 1))
    }
}

struct Thing {
    texts: Vec<String>,
    selectable: bool,
    size: Vec2,
}

impl Thing {
    fn new(bottom: bool, element: Elements) -> Self {
        let options = element.value();
        let mut texts =
            format_file(load_file(&format!("assets/graphics/{}", options.0)));
        if bottom {
            texts.remove(0);
            texts.remove(0);
        }
        texts.pop();
        if let Elements::House(l) = element {
            let ll = texts.len();
            let label = if l.len() <= 6 {
                l.to_owned()
            } else {
                format!("{}.", &l[0..6])
            };
            mem::replace(&mut texts[ll - 4], format!(" |{:7}| ", label));
        }
        let size = Vec2::new(texts.first().unwrap().len(), texts.len());
        Thing {
            texts,
            selectable: options.1,
            size,
        }
    }
}

impl cursive::view::View for Thing {
    fn draw(&self, printer: &Printer) {
        let x = 0;

        let style = if printer.focused {
            //ColorStyle::primary()
            ColorStyle::highlight()
        } else {
            ColorStyle::secondary()
        };

        for (i, text) in self.texts.iter().enumerate() {
            printer.with_color(style, |printer| printer.print((x, i), text));
            //printer.print((x, i), text);
        }
    }

    fn take_focus(&mut self, _: Direction) -> bool {
        self.selectable
    }

    //fn on_event(&mut self, event: Event) -> EventResult {
    //match event {
    //Event::Mouse {
    //offset,
    //position,
    //event: MouseEvent::Press(btn),
    //} => {
    //// Get cell for position
    //self.texts.pop();
    //self.texts.pop();
    //return EventResult::Consumed(None);
    //}
    //_ => {}
    //}
    //EventResult::Ignored
    //}

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        self.size
    }
}
