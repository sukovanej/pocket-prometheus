use crate::parser::Measurement;
use textplots::{Chart, Plot, Shape};

pub fn render_graph(measurements: &Vec<Measurement>, now_timestamp_ns: u128) -> String {
    let (name, points) = measurements_to_points(measurements, now_timestamp_ns);
    let (xmin, _, ymin, ymax) = calculate_dimensions(&points);

    let result = Chart::new_with_y_range(120, 60, xmin, 60.0, ymin, ymax)
        .lineplot(&Shape::Points(&points))
        .to_string();

    format!("Metric: {}\n{}", name, result)
}

fn measurements_to_points(
    measurements: &Vec<Measurement>,
    now_timestamp_ns: u128,
) -> (String, Vec<(f32, f32)>) {
    let mut points = vec![];
    let mut name = "".to_string();

    for measurement in measurements {
        let timestamp_s = (measurement.timestamp_ns - now_timestamp_ns) / 1_000_000_000;
        let metrics = &measurement.metrics;
        name = metrics[0].name.clone();
        //let metrics_len = metrics.len();

        //if metrics_len != 1 {
        //    panic!("expected 1 metric, got {}", metrics_len);
        //}

        points.push((timestamp_s as f32, metrics[0].value as f32))
    }

    (name, points)
}

fn calculate_dimensions(points: &Vec<(f32, f32)>) -> (f32, f32, f32, f32) {
    let (xfirst, yfirst) = points[0];
    let (mut xmin, mut xmax, mut ymin, mut ymax) = (xfirst, xfirst, yfirst, yfirst);

    for (x, y) in points {
        if *x < xmin {
            xmin = *x;
        } else if *x > xmax {
            xmax = *x;
        }

        if *y < ymin {
            ymin = *y;
        } else if *y > ymax {
            ymax = *y;
        }
    }

    return (xmin, xmax, ymin, ymax);
}
