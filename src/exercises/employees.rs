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

fn analyze(entries: Vec<WorkEntry>) -> HashMap<String, (Duration, usize, Vec<String>)> {
    let mut stats: HashMap<String, (Duration, usize, Vec<String>)> = HashMap::new();

    for entry in entries {
        let work_duration = entry.end_time - entry.start_time;

        let emp_stat =
            stats
                .entry(entry.employee_id.clone())
                .or_insert((Duration::zero(), 0, vec![]));
        emp_stat.0 = emp_stat.0 + work_duration;
        emp_stat.1 += 1;

        if work_duration > Duration::hours(8) {
            emp_stat.2.push(entry.date.to_string());
        }
    }

    stats
}

fn write_report(
    stats: &HashMap<String, (Duration, usize, Vec<String>)>,
    path: &str,
) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(path)?;

    for (employee, (total_duration, day_count, overtime_days)) in stats {
        let avg_minutes = total_duration.num_minutes() as f64 / *day_count as f64;
        let hours = total_duration.num_minutes() / 60;
        let minutes = total_duration.num_minutes() % 60;

        writeln!(file, "Pracownik: {}", employee)?;
        writeln!(file, "  Całkowity czas pracy: {}h {}min", hours, minutes)?;
        writeln!(
            file,
            "  Średnia długość dnia pracy: {:.2}h",
            avg_minutes / 60.0
        )?;
        writeln!(file, "  Dni z nadgodzinami: {:?}", overtime_days)?;
        writeln!(file)?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let entries = read_csv("work_log.csv")?;
    let stats = analyze(entries);
    write_report(&stats, "report.txt")?;
    println!("Raport zapisany do report.txt");
    Ok(())
}
