use std::collections::HashMap;
use std::hash::{Hash, Hasher};

const DISCOUNT_100_TO_1000  : f64 = 0.1;
const DISCOUNT_1000_OR_MORE : f64 = 0.15;

pub const FOOD : ItemKind = ItemKind { id: 0, tax: 0. };
pub const ALCOHOL : ItemKind = ItemKind { id: 1, tax: 0.155 };
pub const OTHER : ItemKind = ItemKind { id: 2, tax: 0.075 };

/// ItemKind represents the type of Item (Food, Alcohol, or Other)
/// The id field must be a unique identifier for the ItemKind; it is used to compare ItemKind instances
#[derive(Clone)]
pub struct ItemKind {
    id: u32,
    tax: f64,
}

impl PartialEq for ItemKind {
    fn eq(&self, other: &ItemKind) -> bool {
        return self.id == other.id;
    }
}

impl Eq for ItemKind { }

impl Hash for ItemKind {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

/// The Item struct represents an item being purchased. It has a kind (ItemKind) and a price represented in cents.
pub struct Item {
    kind: ItemKind,
    price: u32,
}

impl Item {
    pub fn new(kind: ItemKind, price: u32) -> Item {
        return Item {
            kind: kind,
            price: price,
        };
    }
}

/// This functions takes a list of Items and calculates the total price to buy them.
///
/// If less than $100 of merchandise is bought, the full price is paid.
/// If $100 or more, but less than $1000 of merchandise is bought, a 10% discount is applied.
/// If $1000 or more of merchandise is bought, a 15% discount is applied.
///
/// Food items are not taxed.
/// Alcohol is subject to a 7.5% sales tax and an 8% "sin" tax
/// All other items are subject to 7.5% sales tax
///
/// # Examples
/// ```
/// use pricing::*;
/// assert_eq!(total(&vec![Item::new(FOOD, 1000), Item::new(ALCOHOL, 1000), Item::new(OTHER, 1000), Item::new(ALCOHOL, 10000)]), 13301);
/// ```
pub fn total(items: &Vec<Item>) -> u32 {
    let mut totals = HashMap::new();

    // Calculate totals for each item type
    for item in items {
        let new_total = match totals.get(&item.kind) {
            Some(total) => total + item.price,
            None => item.price,
        };
        totals.insert(item.kind.clone(), new_total);
    }

    // Apply discounts
    let raw_total : u32 = totals.values().sum();

    let discount : f64;
    if raw_total >= 10000 && raw_total < 100000 {
        discount = 1. - DISCOUNT_100_TO_1000;
    } else if raw_total >= 100000 {
        discount = 1. - DISCOUNT_1000_OR_MORE;
    } else {
        discount = 1.;
    }

    for (_, total) in totals.iter_mut() {
        *total = ((*total as f64) * discount) as u32;
    }

    // Apply taxes
    for (kind, total) in totals.iter_mut() {
        *total = ((*total as f64) * (1. + kind.tax)) as u32;
    }

    return totals.values().sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_food_pricing() {
        assert_eq!(total(&vec![Item::new(FOOD, 3333), Item::new(FOOD, 3333), Item::new(FOOD, 3333)]), 9999);
        assert_eq!(total(&vec![Item::new(FOOD, 5000), Item::new(FOOD, 5000)]), 9000);
        assert_eq!(total(&vec![Item::new(FOOD, 5000), Item::new(FOOD, 5000), Item::new(FOOD, 89999)]), 89999);
        assert_eq!(total(&vec![Item::new(FOOD, 100000)]), 85000);
    }

    #[test]
    fn test_other_pricing() {
        assert_eq!(total(&vec![Item::new(OTHER, 1000)]), 1075);
    }

    #[test]
    fn test_alcohol_pricing() {
        assert_eq!(total(&vec![Item::new(ALCOHOL, 1000)]), 1155);
    }

    #[test]
    fn test_pricing() {
        assert_eq!(total(&vec![Item::new(FOOD, 1000), Item::new(ALCOHOL, 1000), Item::new(OTHER, 1000), Item::new(ALCOHOL, 10000)]), 13301);
    }
}