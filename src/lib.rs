pub trait LucideIcon: std::fmt::Display + std::fmt::Debug {
    fn to_svg(&self) -> String {
        self.to_string()
    }
}

include!(concat!(env!("OUT_DIR"), "/icons.rs"));
