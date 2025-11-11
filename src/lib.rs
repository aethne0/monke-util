pub fn fmt_size(size: u64) -> String {
    if size == 0 {
        return "0B".to_string();
    }

    let prefixes = ["PiB", "TiB", "GiB", "MiB", "KiB", "Bytes"];
    let mut base = 1;
    let counts: Vec<u64> = (0..prefixes.len())
        .map(|_| {
            let mut size = size;
            size %= base * 1024;
            size /= base;
            base *= 1024;
            size
        })
        .collect();

    let pairs = counts
        .iter()
        .rev()
        .enumerate()
        //.skip_while(|p| *p.1 == 0)
        .filter(|p| *p.1 > 0)
        .map(|(i, v)| format!("{}{}", v, prefixes[i]));

    /*
    format!(
        "{} ({} Bytes)",
        pairs.into_iter().collect::<Vec<_>>().join("."),
        size
    )
    */
    format!("{}", pairs.into_iter().collect::<Vec<_>>().join(" + "),)
}

#[macro_export]
macro_rules! Bytes {
    ($x:expr) => {
        $x
    };
}

#[macro_export]
macro_rules! KiB {
    ($x:expr) => {
        $x * 1024
    };
}

#[macro_export]
macro_rules! MiB {
    ($x:expr) => {
        $x * 1024 * 1024
    };
}

#[macro_export]
macro_rules! GiB {
    ($x:expr) => {
        $x * 1024 * 1024 * 1024
    };
}

#[macro_export]
macro_rules! TiB {
    ($x:expr) => {
        $x * 1024 * 1024 * 1024 * 1024
    };
}
