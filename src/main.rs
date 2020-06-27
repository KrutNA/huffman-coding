mod queue;
mod types;

fn main() {
    let data = "Lorem ipsum dolor sit amet";
    let heap = queue::generate_from_data(&Vec::from(data.as_bytes()));
    println!("{:#?}", heap);
}
