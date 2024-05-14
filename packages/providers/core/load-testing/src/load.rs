use crate::types::DataCollection;

pub fn generate() -> DataCollection {
    let time = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let count = vec![100, 243, 123, 222, 312, 100, 243, 123, 222, 312];
    let duration = vec![10, 24, 52, 92, 31, 30, 44, 122, 2, 61];

    // TODO: load test the socket server

    // 1. In one thread, start the nv_provider_core::server::Server socket server
    // 2. In another thread, start an UnixStream client that connects to the same socket
    // 3. In a loop for N iterations
    //      a. Get an instant
    //      b. Request a value from the socket
    //      c. Measure the duration
    //      d. Validate a value is returned

    (time, count, duration)
}
