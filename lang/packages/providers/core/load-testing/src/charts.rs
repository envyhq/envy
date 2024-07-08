use crate::{
    load::{X_DURATION_UNIT, Y_DURATION_UNIT},
    types::Value,
};
use charming::{
    component::{Axis, Title},
    element::{AxisType, Tooltip, Trigger},
    series::Line,
    Chart, ImageRenderer,
};
use std::fs::write;

pub fn generate(time: &[Value], req_count: &[Value], client_count: &[Value], duration: &[Value]) {
    let req_count_chart = Chart::new()
        .title(Title::new().text("Load test results - Request count"))
        .tooltip(Tooltip::new().trigger(Trigger::Axis))
        .x_axis(
            Axis::new().type_(AxisType::Category).data(
                time.iter()
                    .map(|t| format!("{}{}", t, X_DURATION_UNIT))
                    .collect(),
            ),
        )
        .y_axis(Axis::new().name("Request count over time"))
        .series(Line::new().data(req_count.to_owned()));

    let client_count_chart = Chart::new()
        .title(Title::new().text("Load test results - Client count"))
        .tooltip(Tooltip::new().trigger(Trigger::Axis))
        .x_axis(
            Axis::new().type_(AxisType::Category).data(
                time.iter()
                    .map(|t| format!("{}{}", t, X_DURATION_UNIT))
                    .collect(),
            ),
        )
        .y_axis(Axis::new().name("Request count over time"))
        .series(Line::new().data(client_count.to_owned()));

    let duration_chart = Chart::new()
        .title(Title::new().text("Load test results - Request duration"))
        .tooltip(Tooltip::new().trigger(Trigger::Axis))
        .x_axis(
            Axis::new().type_(AxisType::Category).data(
                time.iter()
                    .map(|t| format!("{}{}", t, X_DURATION_UNIT))
                    .collect(),
            ),
        )
        .y_axis(Axis::new().name(format!("Request duration ({})", Y_DURATION_UNIT)))
        .series(Line::new().data(duration.to_owned()));

    let both_chart = Chart::new()
        .title(
            Title::new().text("Load test results - Request duration, req count and client count"),
        )
        .tooltip(Tooltip::new().trigger(Trigger::Axis))
        .x_axis(
            Axis::new().type_(AxisType::Category).data(
                time.iter()
                    .map(|t| format!("{}{}", t, X_DURATION_UNIT))
                    .collect(),
            ),
        )
        .y_axis(Axis::new().name(format!("Request duration ({}) and count", Y_DURATION_UNIT)))
        .series(Line::new().data(duration.to_owned()))
        .series(Line::new().data(req_count.to_owned()))
        .series(Line::new().data(client_count.to_owned()));

    let mut renderer = ImageRenderer::new(1000, 800);

    let req_count_image = renderer.render(&req_count_chart).unwrap();
    let client_count_image = renderer.render(&client_count_chart).unwrap();
    let duration_image = renderer.render(&duration_chart).unwrap();
    let both_image = renderer.render(&both_chart).unwrap();

    let html = format!(
        "
<!DOCTYPE html>
<html>
    <head>
        <title>Load Test Results</title>
    </head>
    <body>
        <div style=\"display:flex;justify-content:center;align-items:center;flex-direction:column;\">
            {}
            {}
            {}
            {}
        </div>
    </body>
</html>
",
        both_image, req_count_image, client_count_image, duration_image
    );

    let _ = write("./load-test-results.html", html);
}
