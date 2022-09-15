use std::io;

fn main() -> io::Result<()> {
    println!("Enter your weight (kg):");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    let weight = buf.trim().parse::<f32>().unwrap();
    let weight_on_mars = calculate_weight_on_mars(weight);
    println!("Weight on Mars: {} kg", weight_on_mars);
    Ok(())
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}
