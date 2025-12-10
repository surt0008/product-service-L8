use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Roku TV".to_string(),
            price: 499.99,
            description: "A smart TV from Roku, offering built‑in streaming apps and an easy-to-use interface.Great for movies, shows, and simple home entertainment setups.".to_string(),
            image: "/1 roku tv.jpg".to_string()
        },
        Product {
            id: 2,
            name: "Lenovo Laptop".to_string(),
            price: 999.99,
            description: "A Lenovo laptop designed for school, office work, and light gaming depending on the model.Known for reliable performance and strong battery life.".to_string(),
            image: "/2 lenovo laptop.jpeg".to_string()
        },
        Product {
            id: 3,
            name: "Sony Headphones".to_string(),
            price: 299.99,
            description: "Over‑ear Bluetooth headphones from Sony, offering clear sound and noise cancellation.Comfortable for long listening sessions and travel.".to_string(),
            image: "/3 sony headphones.jpg".to_string()
        },
        Product {
            id: 4,
            name: "Google Pixel 9 ".to_string(),
            price: 999.99,
            description: "The Google Pixel series is known for clean Android software and excellent cameras.Fast, smooth, and great for everyday phone use.".to_string(),
            image: "/4 gp9.jpg".to_string()
        },
        Product {
            id: 5,
            name: "JBL Bluetooth Speaker".to_string(),
            price: 149.99,
            description: "A portable JBL Bluetooth speaker with strong bass and waterproof design.Perfect for outdoor use, parties, and travel.".to_string(),
            image: "/5 jbl.jpg".to_string()
        },
        Product {
            id: 6,
            name: "iPhone".to_string(),
            price: 1499.99,
            description: "A sleek Apple smartphone with fast performance and high-quality cameras.Great for photos, videos, and everyday mobile tasks.".to_string(),
            image: "/6 iphone.jpg".to_string()
        },
        Product {
            id: 7,
            name: "Apple iPad".to_string(),
            price: 699.99,
            description: "A lightweight Apple iPad ideal for schoolwork, streaming, gaming, and drawing.Smooth performance with access to many apps.".to_string(),
            image: "/7 tablet.jpg".to_string()
        },
        Product {
            id: 8,
            name: "PlayStation 5 ".to_string(),
            price: 599.99,
            description: "The Sony PS5 is a powerful gaming console with high‑end graphics and fast loading.Perfect for modern games, media, and streaming.".to_string(),
            image: "/8 ps.jpg".to_string()
        },
        Product {
            id: 9,
            name: "Smart Watch ".to_string(),
            price: 149.99,
            description: "A smartwatch with fitness tracking, notifications, and heart‑rate monitoring.Useful for workouts and everyday activity tracking.".to_string(),
            image: "/9 watch.jpg".to_string()
        },
        Product {
            id: 10,
            name: " iScooter Electric Scooter ".to_string(),
            price: 699.99,
            description: "An iScooter electric scooter for short commutes and quick travel.Lightweight, foldable, and battery-powered.".to_string(),
            image: "/10 iscooter.jpeg".to_string()
        }
    ]
}