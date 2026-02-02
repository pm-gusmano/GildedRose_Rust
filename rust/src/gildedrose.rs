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
    let normal_quality_change = 1;

    // Set max, min item qualities
    let min_item_quality = 0;
    let max_item_quality = 50;

    // Set backstage pass parameters
    let backstage_pass_parameters = BackstagePassParameters {
        very_hype_number_of_days: 5,
        kind_of_hype_number_of_days: 10,
        very_hype_quality_increase: 3,
        kind_of_hype_quality_increase: 2,
        post_concert_pass_quality: 0,
    };

    if item.name.to_lowercase().starts_with("sulfuras") {
        return;
    }

    item.sell_in -= 1;

    if item.name.to_lowercase().starts_with("aged brie") {
        update_aged_brie(item, normal_quality_change);
    } else if item.name.to_lowercase().starts_with("backstage passes") {
        update_backstage_pass(item, normal_quality_change, &backstage_pass_parameters);
    } else if item.name.to_lowercase().starts_with("conjured") {
        update_conjured_item(item, normal_quality_change);
    } else {
        update_generic_item(item, normal_quality_change);
    }

    item.quality = item.quality.clamp(min_item_quality, max_item_quality);
}

struct BackstagePassParameters {
    very_hype_number_of_days: i32,
    kind_of_hype_number_of_days: i32,
    very_hype_quality_increase: i32,
    kind_of_hype_quality_increase: i32,
    post_concert_pass_quality: i32,
}

fn update_backstage_pass_quality_pre_sale_date(
    item: &mut Item,
    normal_quality_change: i32,
    parameters: &BackstagePassParameters,
) {
    let quality_increase = if item.sell_in + 1 <= parameters.very_hype_number_of_days {
        parameters.very_hype_quality_increase
    } else if item.sell_in + 1 <= parameters.kind_of_hype_number_of_days {
        parameters.kind_of_hype_quality_increase
    } else {
        normal_quality_change
    };
    item.quality += quality_increase;
}

fn update_backstage_pass(
    item: &mut Item,
    normal_quality_change: i32,
    parameters: &BackstagePassParameters,
) {
    if item.sell_in >= 0 {
        update_backstage_pass_quality_pre_sale_date(item, normal_quality_change, parameters);
    } else {
        item.quality = parameters.post_concert_pass_quality;
    }
}

fn update_generic_item(item: &mut Item, normal_quality_change: i32) {
    if item.sell_in >= 0 {
        item.quality -= normal_quality_change;
    } else {
        item.quality -= normal_quality_change * 2;
    }
}

fn update_aged_brie(item: &mut Item, normal_quality_change: i32) {
    if item.sell_in >= 0 {
        item.quality += normal_quality_change;
    } else {
        item.quality += normal_quality_change * 2;
    }
}

fn update_conjured_item(item: &mut Item, normal_quality_change: i32) {
    if item.sell_in >= 0 {
        item.quality -= 2 * normal_quality_change;
    } else {
        item.quality -= 4 * normal_quality_change;
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
