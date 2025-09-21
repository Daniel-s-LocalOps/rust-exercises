use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;

struct ImportantEvent {
    name: String,
    date: DateTime<Utc>,
}

trait Deadline {
    fn is_passed(&self) -> bool;
}

impl Deadline for ImportantEvent {
    fn is_passed(&self) -> bool {
        let current_time = Utc::now();

        current_time > self.date
    }
}

fn main() {
    let event = ImportantEvent {
        name: "Daniel's test".to_string(),
        date: Utc::now() + Duration::days(2),
    };

    println!("event :{} has passed: {}", event.name, event.is_passed());
}
