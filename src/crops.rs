use crate::models::CropType;
use std::collections::HashMap;

pub fn crop_registry() -> HashMap<String, CropType> {
    HashMap::from([
        (
            "wheat".to_string(),
            CropType {
                icon: "🌾",
                id: "wheat",
                grow_time: 60 * 5,
                base_buy_price: 5,
                base_sell_price: 8,
            },
        ),
        (
            "carrot".to_string(),
            CropType {
                icon: "🥕",
                id: "carrot",
                grow_time: 60 * 8,
                base_buy_price: 8,
                base_sell_price: 13,
            },
        ),
        (
            "corn".to_string(),
            CropType {
                icon: "🌽",
                id: "corn",
                grow_time: 60 * 7,
                base_buy_price: 9,
                base_sell_price: 11,
            },
        ),
        (
            "potato".to_string(),
            CropType {
                icon: "🥔",
                id: "potato",
                grow_time: 60 * 9,
                base_buy_price: 3,
                base_sell_price: 4,
            },
        ),
        (
            "tomato".to_string(),
            CropType {
                icon: "🍅",
                id: "tomato",
                grow_time: 60 * 7,
                base_buy_price: 11,
                base_sell_price: 15,
            },
        ),
        (
            "strawberry".to_string(),
            CropType {
                icon: "🍓",
                id: "strawberry",
                grow_time: 60 * 12,
                base_buy_price: 20,
                base_sell_price: 30,
            },
        ),
        (
            "grape".to_string(),
            CropType {
                icon: "🍇",
                id: "grape",
                grow_time: 60 * 15,
                base_buy_price: 23,
                base_sell_price: 25,
            },
        ),
        (
            "watermelon".to_string(),
            CropType {
                icon: "🍉",
                id: "watermelon",
                grow_time: 60 * 20,
                base_buy_price: 30,
                base_sell_price: 35,
            },
        ),
        (
            "broccoli".to_string(),
            CropType {
                icon: "🥦",
                id: "broccoli",
                grow_time: 60 * 8,
                base_buy_price: 4,
                base_sell_price: 7,
            },
        ),
        (
            "avocado".to_string(),
            CropType {
                icon: "🥑",
                id: "avocado",
                grow_time: 60 * 5,
                base_buy_price: 6,
                base_sell_price: 9,
            },
        ),
        (
            "mango".to_string(),
            CropType {
                icon: "🥭",
                id: "mango",
                grow_time: 60 * 15,
                base_buy_price: 14,
                base_sell_price: 20,
            },
        ),
        (
            "cantaloupe".to_string(),
            CropType {
                icon: "🍈",
                id: "cantaloupe",
                grow_time: 60 * 20,
                base_buy_price: 15,
                base_sell_price: 25,
            },
        ),
        (
            "lemon".to_string(),
            CropType {
                icon: "🍋",
                id: "lemon",
                grow_time: 60 * 8,
                base_buy_price: 10,
                base_sell_price: 12,
            },
        ),
    ])
}
