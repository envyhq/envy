use charming::{
    component::{Axis, Title},
    element::{AxisType, Tooltip, Trigger},
    series::Line,
    Chart, ImageRenderer,
};
use std::fs::write;

fn generate_chart(time: &[i64], count: &[i64], duration: &[i64]) {
    let count_chart = Chart::new()
        .title(Title::new().text("Load test results - Request count"))
        .tooltip(Tooltip::new().trigger(Trigger::Axis))
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .data(time.iter().map(|t| format!("{}s", t)).collect()),
        )
        .y_axis(Axis::new().name("Request #"))
        .series(Line::new().data(count.to_owned()));

    let duration_chart = Chart::new()
        .title(Title::new().text("Load test results - Request duration"))
        .tooltip(Tooltip::new().trigger(Trigger::Axis))
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .data(time.iter().map(|t| format!("{}s", t)).collect()),
        )
        .y_axis(Axis::new().name("Request ms"))
        .series(Line::new().data(duration.to_owned()));

    let both_chart = Chart::new()
        .title(Title::new().text("Load test results - Request duration & count"))
        .tooltip(Tooltip::new().trigger(Trigger::Axis))
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .data(time.iter().map(|t| format!("{}s", t)).collect()),
        )
        .y_axis(Axis::new().name("Request ms & #"))
        .series(Line::new().data(duration.to_owned()))
        .series(Line::new().data(count.to_owned()));

    let mut renderer = ImageRenderer::new(1000, 800);

    let count = renderer.render(&count_chart).unwrap();
    let duration = renderer.render(&duration_chart).unwrap();
    let both = renderer.render(&both_chart).unwrap();

    let html = format!(
        "
<!DOCTYPE html>
<html>
    <head>
        <title>Load Test Results</title>
    </head>
    <body>
        <div style=\"display:flex;justify-content:center;align-items:center;flex-direction:row;\">
            {}
            {}
            {}
        </div>
    </body>
</html>
",
        both, count, duration
    );

    let _ = write("./load-test-results.html", html);
}

fn main() {
    let time = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let count = vec![100, 243, 123, 222, 312, 100, 243, 123, 222, 312];

    let duration = vec![10, 24, 52, 92, 31, 30, 44, 122, 2, 61];

    generate_chart(&time, &count, &duration);
}
