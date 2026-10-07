use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Write;

use chrono::{Duration, NaiveDate, NaiveTime};
use csv::ReaderBuilder;

#[derive(Debug)]
struct WorkEntry {
    employee_id: String,
    date: NaiveDate,
    start_time: NaiveTime,
    end_time: NaiveTime,
}

fn read_csv(path: &str) -> Result<Vec<WorkEntry>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().from_path(path)?;
    let mut entries = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let entry = WorkEntry {
            employee_id: record[0].to_string(),
            date: NaiveDate::parse_from_str(&record[1], "%Y-%m-%d")?,
            start_time: NaiveTime::parse_from_str(&record[2], "%H:%M")?,
            end_time: NaiveTime::parse_from_str(&record[3], "%H:%M")?,
        };
        entries.push(entry);
    }

    Ok(entries)
}

// A named struct instead of a (Duration, usize, Vec<String>) tuple keeps
// the aggregation and reporting code self-documenting.
struct EmployeeStats {
    total: Duration,
    days: usize,
    overtime_days: Vec<String>,
}

fn analyze(entries: Vec<WorkEntry>) -> HashMap<String, EmployeeStats> {
    let mut stats: HashMap<String, EmployeeStats> = HashMap::new();

    for entry in entries {
        let work_duration = entry.end_time - entry.start_time;

        let emp_stat = stats
            .entry(entry.employee_id.clone())
            .or_insert_with(|| EmployeeStats {
                total: Duration::zero(),
                days: 0,
                overtime_days: vec![],
            });
        emp_stat.total += work_duration;
        emp_stat.days += 1;

        if work_duration > Duration::hours(8) {
            emp_stat.overtime_days.push(entry.date.to_string());
        }
    }

    stats
}

fn write_report(
    stats: &HashMap<String, EmployeeStats>,
    path: &str,
) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(path)?;

    for (employee, stat) in stats {
        let avg_minutes = stat.total.num_minutes() as f64 / stat.days as f64;
        let hours = stat.total.num_minutes() / 60;
        let minutes = stat.total.num_minutes() % 60;

        writeln!(file, "Employee: {}", employee)?;
        writeln!(file, "  Total work time: {}h {}min", hours, minutes)?;
        writeln!(file, "  Average workday length: {:.2}h", avg_minutes / 60.0)?;
        writeln!(file, "  Days with overtime: {:?}", stat.overtime_days)?;
        writeln!(file)?;
    }

    Ok(())
}

// Every other exercise module exposes `pub fn run()`. A private `fn main()`
// inside a module is NOT a program entry point — it would never be called.
pub fn run() {
    if let Err(err) = generate_report() {
        eprintln!("Failed to generate the report: {err}");
    }
}

fn generate_report() -> Result<(), Box<dyn Error>> {
    let entries = read_csv("work_log.csv")?;
    let stats = analyze(entries);
    write_report(&stats, "report.txt")?;
    println!("Report written to report.txt");
    Ok(())
}
