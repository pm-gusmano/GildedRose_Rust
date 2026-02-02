use std::fmt::{self, Display};
pub struct Item {
    pub name: String,
    pub sell_in: i32,
    pub quality: i32,
}

impl Item {
    pub fn new(name: impl Into<String>, sell_in: i32, quality: i32) -> Item {
        Item {
            name: name.into(),
            sell_in,
            quality,
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.name, self.sell_in, self.quality)
    }
}

// impl Updatable for Item {
//     fn update(&mut self) {
//         let item_type = ItemType::from_name(&self.name);
//         match item_type {
//             ItemType::AgedBrie => update_aged_brie(self),
//             ItemType::BackstagePass => update_backstage_pass(self),
//             ItemType::Sulfuras => (), // Legendary
//             // ItemType::Conjured => update_conjured_item(self),
//             _ => update_normal_item(self),
//         }
//     }
// }

fn update_normal_item(item: &mut Item) {
    item.sell_in -= 1;

    if item.sell_in >= 0 {
        item.quality -= 1;
    } else if item.sell_in < 0 {
        item.quality -= 2;
    }

    if item.quality < 0 {
        item.quality = 0;
    }
}

pub struct GildedRose {
    pub items: Vec<Item>,
}

impl GildedRose {
    pub fn new(items: Vec<Item>) -> GildedRose {
        GildedRose { items }
    }

    pub fn update_quality(&mut self) {
        for item in &mut self.items {
            update_item_quality(item);
        }
    }
}

trait Updatable {
    fn update(&mut self);
}
enum ItemType {
    AgedBrie,
    BackstagePass,
    Sulfuras,
    Conjured,
}

fn update_item_quality(item: &mut Item) {
    // Normal item branch
    if item.name != "Aged Brie" && item.name != "Backstage passes to a TAFKAL80ETC concert" {
        if item.quality > 0 {
            if item.name != "Sulfuras, Hand of Ragnaros" {
                item.quality = item.quality - 1;
            }
        }
    } else {
        // Aged Brie and Backstage passes branch
        if item.quality < 50 {
            item.quality = item.quality + 1;

            // Just backstage passes branch
            if item.name == "Backstage passes to a TAFKAL80ETC concert" {
                if item.sell_in < 11 {
                    if item.quality < 50 {
                        item.quality = item.quality + 1;
                    }
                }

                if item.sell_in < 6 {
                    if item.quality < 50 {
                        item.quality = item.quality + 1;
                    }
                }
            }
        }
    }

    // Sulfuras branch
    if item.name != "Sulfuras, Hand of Ragnaros" {
        item.sell_in = item.sell_in - 1;
    }

    // Quality Handling for all items after their sell date
    if item.sell_in < 0 {
        if item.name != "Aged Brie" {
            if item.name != "Backstage passes to a TAFKAL80ETC concert" {
                if item.quality > 0 {
                    if item.name != "Sulfuras, Hand of Ragnaros" {
                        item.quality = item.quality - 1;
                    }
                }
            // Updating Backstage Pass quality after sell date
            // It goes to 0
            } else {
                item.quality = 0;
            }
        } else {
            // Updating Aged Brie quality after sell date
            if item.quality < 50 {
                item.quality = item.quality + 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{GildedRose, Item};

    #[test]
    pub fn foo() {
        let items = vec![Item::new("foo", 0, 0)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!("foo", rose.items[0].name);
    }
}
