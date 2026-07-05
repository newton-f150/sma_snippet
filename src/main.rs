// Simple Moving Average (SMA)
//  A statistical technique that calculates the average of the most recent N data points.
// As new data arrives, the oldest value is dropped and the average is recalculated.
// This is why it's called a moving average—the window "moves" through the data over time.

fn moving_average(data:&[f64],window:usize)->Option<f64>{
    if data.len() < window || window == 0 {
        return None;
    }

    let sum :f64 = data[data.len() - window..].iter().sum();
    Some(sum / window as f64)
}

fn main(){
    let sales = vec![10.0,12.3,12.0,13.9,12.1,11.2,14.5];

    let prediction = moving_average(&sales, 5);
    println!("\nValue without Error Handling: {:?}",prediction);

    match prediction{
        Some(val)=>println!("Value With Error Handling: {:?}",val),
        None => eprint!("Failed to Print the value")
    }
}