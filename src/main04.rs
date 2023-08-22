use aoc2018::input_lines;
use sscanf::sscanf;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Timestamp {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum LogEntryData {
    BeginShift(usize),
    Sleeps,
    Wakes,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct LogEntry {
    date: Timestamp,
    event: LogEntryData,
}

impl FromStr for LogEntry {
    type Err = sscanf::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (stamp, event) = s.split_at(19);
        let timedata = sscanf!(stamp, "[{u32}-{u32}-{u32} {u32}:{u32}] ")?;

        use LogEntryData::*;
        let event = if event.starts_with("Guard") {
            BeginShift(sscanf!(event, "Guard #{usize} begins shift")?)
        } else if event.starts_with("falls asleep") {
            Sleeps
        } else if event.starts_with("wakes up") {
            Wakes
        } else {
            panic!("Bad input again.")
        };

        let date = Timestamp {
            year: timedata.0,
            month: timedata.1,
            day: timedata.2,
            hour: timedata.3,
            minute: timedata.4,
        };

        Ok(LogEntry { date, event })
    }
}

fn sleepiest_minute(naps: &[(u32, u32)]) -> (u32, u32) {
    // Find sleepiest minute
    let mut hour = [0; 60].to_vec();
    for (start, end) in naps {
        for minute in *start..*end {
            hour[minute as usize] += 1;
        }
    }
    let result = hour
        .iter()
        .enumerate()
        .max_by_key(|(_, count)| *count)
        .expect("No winner?");
    (result.0 as u32, *result.1 as u32)
}

fn main() {
    let mut log = input_lines::<LogEntry>();
    log.sort();

    let mut sleeps: HashMap<usize, Vec<(u32, u32)>> = HashMap::new();
    let mut guard: usize = 0;
    let mut sleep_time = 0;
    // Read log, store sleep times.
    for entry in log {
        match entry.event {
            LogEntryData::BeginShift(g) => guard = g,
            LogEntryData::Sleeps => sleep_time = entry.date.minute,
            LogEntryData::Wakes => sleeps
                .entry(guard)
                .or_default()
                .push((sleep_time, entry.date.minute)),
        }
    }

    // * Strategy 1: the minute the sleepiest guard is most often asleep.

    // Find sleepiest guard.
    let sleepy_guard = sleeps
        .iter()
        .map(|(k, vec)| (k, vec.iter().map(|(a, b)| b - a).sum::<u32>()))
        .max_by_key(|(_, v)| *v)
        .expect("No solution?");
    let guard_id = sleepy_guard.0;
    println!("Strategy 1:");
    println!(" - Guard {} slept for {} minutes", guard_id, sleepy_guard.1);

    let minute = sleepiest_minute(&sleeps[guard_id]);

    println!(" - Most slept minute: {}", minute.0);
    println!(" - Challenge response: {}", minute.0 * *guard_id as u32);

    // * Strategy 2: which guard is most often asleep at any given minute.
    
    println!("Strategy 2:");

    let (guard, (minute, count)) = sleeps
        .iter()
        .map(|(guard, sleeps)| (guard, sleepiest_minute(sleeps)))
        .max_by_key(|(_, (_, c))| *c)
        .expect("No result for strategy 2");
    println!(
        " - Guard {} slept a totat of {} minutes on minute {}.",
        guard, count, minute
    );
    println!(" - Challenge response: {}", minute * *guard as u32);
}
