use polars::prelude::*;

fn main() {
    let df = df! [
        "my_int" => [Some(1), Some(2), Some(3), None, Some(5)],
    ]
    .unwrap();

    println!("{}", &df);

    let out = df
        .clone()
        .lazy()
        .select([
            col("my_int"),
            col("my_int").pow(2).alias("pow_2_my_int"),
            col("my_int")
                .quantile(0.3, QuantileInterpolOptions::Nearest)
                .alias("quantile_my_int"),
        ])
        .collect()
        .unwrap();

    println!("{}", out);
}
