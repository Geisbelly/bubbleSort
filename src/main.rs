use rand::random;

fn main() {
    let n: u32 = 100_000;
    let mut list = Vec::with_capacity(n as usize);

    for _ in 0..list.capacity() {
        list.push(random::<u16>())
    }

    let start = std::time::Instant::now();
    bubble_sort(list);

    println!("Sorted {} in {}", n, format_time(start.elapsed()))
}

fn bubble_sort(mut list: Vec<u16>) {
    let n = list.len();

    for _ in 1..(n + 1) {
        for i in 0..((n + 1) - 2) {
            if list[i] > list[i + 1] {
                let t = list[i];
                list[i] = list[i + 1];
                list[i + 1] = t
            }
        }
    }
}

fn format_time(duration: std::time::Duration) -> String {
    let total_ms = duration.as_millis();

    match total_ms {
        ms if ms < 1000 => format!("{}ms", ms),
        ms if ms >= 1000 && ms < 60000 => format!("{}s", total_ms / 1000),
        _ => format!("{}m", total_ms / 60000),
    }
}
