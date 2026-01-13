use tinybook::TinyBook;

fn main() {
    let book = TinyBook::new("BTCUSDT");
    println!("Created TinyBook for symbol: {}", book.symbol());
}
