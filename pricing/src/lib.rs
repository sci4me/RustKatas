const TAX_ALCOHOL           : f64 = 0.155;
const TAX_OTHER             : f64 = 0.075;

const DISCOUNT_100_TO_1000  : f64 = 0.1;
const DISCOUNT_1000_OR_MORE : f64 = 0.15;

/// ItemKind represents the type of Item (Food, Alcohol, or Other)
pub enum ItemKind {
    Food,
    Alcohol,
    Other
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
/// assert_eq!(total(vec![Item::new(ItemKind::Food, 1000), Item::new(ItemKind::Alcohol, 1000), Item::new(ItemKind::Other, 1000), Item::new(ItemKind::Alcohol, 10000)]), 13301);
/// ```
pub fn total(items: Vec<Item>) -> u32 {
    let mut total_food      : u32 = 0;
    let mut total_alcohol   : u32 = 0;
    let mut total_other     : u32 = 0;

    // Calculate totals for each item type
    for item in items {
        match item.kind {
            ItemKind::Food =>    total_food += item.price,
            ItemKind::Alcohol => total_alcohol += item.price,
            ItemKind::Other =>   total_other += item.price,
        }
    }

    // Apply discounts
    let raw_total = total_food + total_alcohol + total_other;

    let discount : f64;
    if raw_total >= 10000 && raw_total < 100000 {
        discount = 1. - DISCOUNT_100_TO_1000;
    } else if raw_total >= 100000 {
        discount = 1. - DISCOUNT_1000_OR_MORE;
    } else {
        discount = 1.;
    }

    total_food = ((total_food as f64) * discount) as u32;
    total_alcohol = ((total_alcohol as f64) * discount) as u32;
    total_other = ((total_other as f64) * discount) as u32; 

    // Apply taxes
    total_alcohol = ((total_alcohol as f64) * (1. + TAX_ALCOHOL)) as u32;
    total_other = ((total_other as f64) * (1. + TAX_OTHER)) as u32;

    return total_food + total_alcohol + total_other;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(total(vec![Item::new(ItemKind::Food, 3333), Item::new(ItemKind::Food, 3333), Item::new(ItemKind::Food, 3333)]), 9999);
        assert_eq!(total(vec![Item::new(ItemKind::Food, 5000), Item::new(ItemKind::Food, 5000)]), 9000);
        assert_eq!(total(vec![Item::new(ItemKind::Food, 5000), Item::new(ItemKind::Food, 5000), Item::new(ItemKind::Food, 89999)]), 89999);
        assert_eq!(total(vec![Item::new(ItemKind::Food, 100000)]), 85000);
        assert_eq!(total(vec![Item::new(ItemKind::Other, 1000)]), 1075);
        assert_eq!(total(vec![Item::new(ItemKind::Alcohol, 1000)]), 1155);
        assert_eq!(total(vec![Item::new(ItemKind::Food, 1000), Item::new(ItemKind::Alcohol, 1000), Item::new(ItemKind::Other, 1000), Item::new(ItemKind::Alcohol, 10000)]), 13301);
    }
}