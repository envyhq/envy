mod charts;
mod load;
mod types;

fn main() {
    let (time, count, duration) = load::generate();
    charts::generate(&time, &count, &duration);
}
