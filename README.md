# Lucide SVG

![Crates.io Downloads (latest version)](https://img.shields.io/crates/dv/lucide-svg)
![Crates.io License](https://img.shields.io/crates/l/lucide-svg)

[Lucide](https://lucide.dev) static icons in Rust.


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
