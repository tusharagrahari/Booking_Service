use std::i32::MIN;

fn main() {
    let mut vec = Vec::new();
    vec.push([900, 1030]);
    vec.push([1000, 1100]);
    vec.push([1030, 1130]);
    vec.push([1100, 1200]);

    let ans = scheduler(vec);
    println!("{ans}");
    // [[900, 1030], [1000, 1100], [1030, 1130], [1100, 1200]]
}


pub fn scheduler(mut intervals: Vec<[i32; 2]>) -> usize {
    let mut count = 0;
    if intervals.is_empty(){
        return count;
    }

    intervals.sort_unstable_by(|a, b| a[1].cmp(&b[1]).then(a[0].cmp(&b[0])));

    let mut last = MIN;

    for [start, end] in intervals {
        if start >= last {
            count+=1;
            last = end;
        }
    }
    return count;
}