use crate::types::DataCollection;

pub fn generate() -> DataCollection {
    let time = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let count = vec![100, 243, 123, 222, 312, 100, 243, 123, 222, 312];
    let duration = vec![10, 24, 52, 92, 31, 30, 44, 122, 2, 61];

    (time, count, duration)
}
