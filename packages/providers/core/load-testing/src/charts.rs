use crate::types::Value;
use charming::{
    component::{Axis, Title},
    element::{AxisType, Tooltip, Trigger},
    series::Line,
    Chart, ImageRenderer,
};
use std::fs::write;

pub fn generate(time: &[Value], count: &[Value], duration: &[Value]) {
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
