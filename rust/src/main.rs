mod gildedrose;

use gildedrose::{GildedRose, Item};
use std::io::{self, Write};

fn main() {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    store_front(&mut handle);
}

fn store_front<W: Write>(w: &mut W) {
        let items = vec![
        Item::new("+5 Dexterity Vest", 10, 20),
        Item::new("Aged Brie", 2, 0),
        Item::new("Elixir of the Mongoose", 5, 7),
        Item::new("Sulfuras, Hand of Ragnaros", 0, 80),
        Item::new("Sulfuras, Hand of Ragnaros", -1, 80),
        Item::new("Backstage passes to a TAFKAL80ETC concert", 15, 20),
        Item::new("Backstage passes to a TAFKAL80ETC concert", 10, 49),
        Item::new("Backstage passes to a TAFKAL80ETC concert", 5, 49),
        // this conjured item does not work properly yet
        Item::new("Conjured Mana Cake", 3, 6),
    ];
    let mut rose = GildedRose::new(items);

    writeln!(w, "OMGHAI!").unwrap();
    for i in 0..=30 {
        writeln!(w, "-------- day {} --------", i).unwrap();
        writeln!(w, "name, sellIn, quality").unwrap();
        for item in &rose.items {
            writeln!(w, "{}", item).unwrap();
        }
        writeln!(w, "").unwrap();
        rose.update_quality();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta;

    #[test]
    fn test_store_front() {
        let mut buffer = Vec::new();
        store_front(&mut buffer);
        let output = String::from_utf8(buffer).unwrap();
        insta::assert_snapshot!(output);
    }
}