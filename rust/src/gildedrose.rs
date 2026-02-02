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

const NORMAL_QUALITY_CHANGE: i32 = 1;

const MIN_ITEM_QUALITY: i32 = 0;
const MAX_ITEM_QUALITY: i32 = 50;

const BACKSTAGE_PARAMS: BackstagePassParameters = BackstagePassParameters {
    very_hype_number_of_days: 5,
    kind_of_hype_number_of_days: 10,
    very_hype_quality_increase: 3,
    kind_of_hype_quality_increase: 2,
    post_concert_pass_quality: 0,
};

fn update_item_quality(item: &mut Item) {
    let kind = classify(&item.name);

    if matches!(kind, Kind::Sulfuras) {
        return;
    }
    item.sell_in -= 1;

    match kind {
        Kind::AgedBrie => update_aged_brie(item, NORMAL_QUALITY_CHANGE),
        Kind::BackstagePasses => {
            update_backstage_pass(item, NORMAL_QUALITY_CHANGE, &BACKSTAGE_PARAMS)
        }
        Kind::Conjured => update_conjured_item(item, NORMAL_QUALITY_CHANGE),
        Kind::Sulfuras => {}
        Kind::Generic => update_generic_item(item, NORMAL_QUALITY_CHANGE),
    }

    item.quality = item.quality.clamp(MIN_ITEM_QUALITY, MAX_ITEM_QUALITY);
}

enum Kind {
    Sulfuras,
    AgedBrie,
    BackstagePasses,
    Conjured,
    Generic,
}

fn classify(name: &str) -> Kind {
    let name = name.to_ascii_lowercase();
    match name.as_str() {
        n if n.starts_with("sulfuras") => Kind::Sulfuras,
        n if n.starts_with("aged brie") => Kind::AgedBrie,
        n if n.starts_with("backstage passes") => Kind::BackstagePasses,
        n if n.starts_with("conjured") => Kind::Conjured,
        _ => Kind::Generic,
    }
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
