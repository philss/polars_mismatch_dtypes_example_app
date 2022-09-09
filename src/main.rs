use polars::prelude::*;

fn main() {
    let df = df! [
        "nrs" => [Some(1), Some(2), Some(3), None, Some(5)],
        "names" => [Some("foo"), Some("ham"), Some("spam"), Some("eggs"), None],
        "random" => [1.4, 1.2, 1.5, 0.3, 0.1],
        "groups" => ["A", "A", "B", "C", "B"],
    ]
    .expect("should be a df");

    println!("{}", &df);

    let out = df
        .clone()
        .lazy()
        .select([
            col("names"),
            col("nrs"),
            col("nrs").pow(2).alias("pow_2_nrs"),
            col("nrs").quantile(0.3, QuantileInterpolOptions::Nearest).alias("quantile_nrs"),
        ])
        .collect()
        .expect("a df");
    println!("{}", out);
}
