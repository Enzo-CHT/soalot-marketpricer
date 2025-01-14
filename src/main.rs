use marketpricer::{MarketPricer, Token, TokenRecord};

pub mod marketpricer;

fn main() {
    println!("Hello, world!");

    let t = Token {
        id: 1,
        slug: String::from("bitcoin"),
        symbol: String::from("BTC"),
    };

    let r = TokenRecord::new(t, 0.1);
    println!("{:#?}", r.get_token());
    println!("{r:#?}");
}
