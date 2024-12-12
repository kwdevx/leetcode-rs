#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn min_meeting_rooms(intervals: Vec<Vec<i32>>) -> i32 {
        // sort the intervals by starting time
        // group overlapping intervals
        // return the group with max length

        let mut start_times = Vec::with_capacity(intervals.len());
        let mut end_times = Vec::with_capacity(intervals.len());
        intervals.iter().for_each(|x| {
            start_times.push(x[0]);
            end_times.push(x[1]);
        });
        start_times.sort();
        end_times.sort();

        let mut start_ptr = 0;
        let mut end_ptr = 0;

        let mut res = 0;
        let mut count = 0;
        while start_ptr < intervals.len() {
            if start_times[start_ptr] < end_times[end_ptr] {
                count += 1;
                // move start_ptr
                start_ptr += 1;
            } else if start_times[start_ptr] >= end_times[end_ptr] {
                count -= 1;
                // move end_ptr
                end_ptr += 1;
            }
            res = std::cmp::max(res, count);
        }

        res
    }
}
