# Simple Moving Average (SMA) in Rust

A simple implementation of the **Simple Moving Average (SMA)** algorithm in Rust for forecasting and data smoothing.

## What is a Simple Moving Average?

A **Simple Moving Average (SMA)** calculates the average of the most recent **N** values in a dataset. As new data becomes available, the oldest value is removed from the calculation and the newest value is added, causing the average to "move" over time.

The formula is:

```text
SMA = (x₁ + x₂ + ... + xₙ) / n
```

Where:

- `x₁ ... xₙ` are the values in the moving window.
- `n` is the window size.

---

## Example

Daily sales:

| Day | Sales |
|-----|------:|
| 1 | 12 |
| 2 | 15 |
| 3 | 14 |
| 4 | 10 |
| 5 | 18 |
| 6 | 20 |
| 7 | 22 |

Using a window size of **3**, the SMA is:

```text
(18 + 20 + 22) / 3

= 60 / 3

= 20
```

The predicted demand for the next day is **20 units**.

---

## Implementation

```rust
fn moving_average(data: &[f64], window: usize) -> Option<f64> {
    if data.len() < window || window == 0 {
        return None;
    }

    let sum: f64 = data[data.len() - window..].iter().sum();

    Some(sum / window as f64)
}
```

---

## Example Usage

```rust
fn main() {
    let sales = vec![12.0, 15.0, 14.0, 10.0, 18.0, 20.0, 22.0];

    let prediction = moving_average(&sales, 3);

    match prediction {
        Some(value) => println!("Predicted demand: {:.2}", value),
        None => println!("Not enough data"),
    }
}
```

Output:

```text
Predicted demand: 20.00
```

---

## How It Works

1. Checks whether there are enough data points.
2. Takes the last `window` values.
3. Calculates their sum.
4. Divides the sum by the window size.
5. Returns the average as `Some(f64)`.
6. Returns `None` if the calculation cannot be performed.

---

## Applications

This implementation can be used in:

- Retail sales forecasting
- Inventory management
- Smart vending machines
- Financial market analysis
- Sensor data smoothing
- Manufacturing production planning
- Energy consumption forecasting
- Traffic monitoring

---

## Limitations

- Uses only the most recent values.
- Every value has equal importance.
- Does not account for trends or seasonality.
- May react slowly to sudden changes.

---

## Future Improvements

Some possible enhancements include:

- Weighted Moving Average (WMA)
- Exponential Moving Average (EMA)
- Rolling averages over an entire dataset
- Configurable forecasting strategies
- CSV data import and export
- Database integration (PostgreSQL with SQLx)
- Real-time data streaming
- Interactive charts and visualizations
- Time-series forecasting models (ARIMA, Prophet)
- Machine Learning models for demand prediction

---

## Project Structure

```text
.
├── src
│   └── main.rs
├── Cargo.toml
└── README.md
```

---

## Running the Project

Clone the repository:

```bash
git clone https://github.com/newton-f150/sma_snippet.git
```

Navigate into the project:

```bash
cd sma_snippet
```

Run the application:

```bash
cargo run
```

---


