mod app;
mod devices;
mod platform;

#[cfg(test)]
mod ui_tests;

slint::include_modules!();

fn main() -> Result<(), Box<dyn std::error::Error>> {
    app::run()
}
