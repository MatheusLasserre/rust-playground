
struct Custom {
    age: usize,
    name: String
}

enum Item {
    Number(usize),
    String(String),
    MyCustom(Custom)
}

impl Item {
    pub fn log(&self) {
        match &self {
            Item::Number(number) => println!("{}", number),
            Item::String(string) => println!("{}", string),
            Item::MyCustom(custom) => println!("Name :{}, Age:{}", custom.name, custom.age),
        }
    }
}

fn append(items: &mut Vec<Item>) {
    items.push(Item::String(("Hellowww").into()))
}

fn main() {
    
   let mut my_enum: Vec<Item> = vec![Item::Number(12)];
   append(&mut my_enum);
    my_enum
    .iter()
    .for_each(|f| f.log());

}


