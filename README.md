# Lucide SVG

![Crates.io Version](https://img.shields.io/crates/v/lucide-svg)
![Crates.io License](https://img.shields.io/crates/l/lucide-svg)

[Lucide](https://lucide.dev) static icons in Rust.

## How it works
When you build your project with `lucide-svg`, a build script runs. It downloads all SVG icons, from the latest [lucid-static unpkg CDN](https://app.unpkg.com/lucide-static@latest), and generates a struct for each icon.

Furthermore, each struct implements `Debug`, `Display`, `Clone`, `Copy`, and, the crate defined trait, `LucideIcon`.

## Usage
```rust
use lucide_svg::LucideIcon;
use lucide_svg::House;

// all icons implement the `LucideIcon` trait
struct Icon(Box<dyn LucideIcon>);

fn foo(icon: impl LucideIcon) -> Icon {
    Icon(Box::new(icon))
}

fn make_house() -> Icon {
    // all icons implement `std::fmt::Display` (and Debug too)
    println!("house icon: {}", House);

    foo(House)
}
```
