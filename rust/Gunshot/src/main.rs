//! This is an interesting project I thought I would do. The idea is that a gun
//! with a gyroscope attachment could feed data into this program, and it would
//! determine when the gun was shot vs. when it's being reloaded or put down.
//! The application stops when the data stops being fed into this program

extern crate regex;

// Includes
use std::thread;
use std::sync::mpsc;
use std::io;
use std::ops::Add;
use std::ops::Sub;
use regex::Regex;

const SIZE: usize = 5;

/// Possible gun event types
#[derive(Debug, PartialEq)]
enum EventType {
    Aim,
    Fire,
    Reload,
    Sit,
}

/// Data type for the gun events. Holds the timestamp
#[derive(Debug)]
struct Event {
     timestamp: u64,
     location: Location,
     event_type: EventType,
}

/// A 3D point used for representing the data. x, y, z
#[derive(PartialEq, Debug, Clone)]
struct Location(f32, f32, f32);

impl Location {
    fn div(self, rhs: f32) -> Location {
        Location(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl Add for Location {
    type Output = Location;

    fn add(self, other: Location) -> Location {
        Location(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Location {
    type Output = Location;

    fn sub(self, rhs: Location) -> Location {
        Location(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2
        )
    }
}

#[derive(Debug)]
struct Range(f32, f32, f32);

impl Range {
    /// Find if point is in range of the origin
    /// The location this function is called on is a `RangeLocation`
    pub fn in_range(&self, origin: &Location, point: &Location) -> bool {
        let value_x = origin.0 + self.0;
        let value_y = origin.1 + self.1;
        let value_z = origin.2 + self.2;

        if (value_x <= point.0 || value_x * -1f32 >= point.0)
            || (value_y <= point.1 || value_y * -1f32 >= point.1)
            || (value_z <= point.2 || value_z * -1f32 >= point.2) {
            return false;
        }

        true
    }
}

/// # Main entry point
fn main() {
    let (tx, rx) = mpsc::channel::<(u64, Location)>();

    let handle = thread::spawn(move || {
        read_thread_handler(tx)
    });

    main_loop(rx);

    // Wait for the sub thread to close
    if let Err(error) = handle.join() {
        println!("Read thread exited with error: {:?}", error);
    }
}

/// Main thread, handles parsing data for detection of events
fn main_loop(rx: mpsc::Receiver<(u64, Location)>) {

    let mut buffer: Vec<(u64, Location)> = Vec::with_capacity(50);
    let mut results: Vec<Event> = Vec::new();

    // Data values
    let mut origin: Option<Location> = None;

    // This is the point within origin that is allowed
    let range: Range = Range(5f32, 5f32, 5f32);

    loop {

        let data = match rx.recv() {
            Ok(value) => value,
            Err(_) => {
                // Execute the last elements in the vec
                let items_left = buffer.len();

                if items_left % 10 == 0 {
                    break;
                } else {
                    (0u64, Location(0f32, 0f32, 0f32))
                }
            },
        };

        buffer.push(data);

        // Every 20 elements check what the gun is doing
        if buffer.len() % SIZE == 0 {
            // Check if sitting

            let start: usize = buffer.len() - SIZE;
            if let Some(event) = is_sitting(&buffer[start..]) {
                // Don't add duplicate events
                // Because this takes an immutable pointer, i couldn't solve
                // mutably pushing an event to the results array in the match

                let push = match results.last() {
                    Some(ref value) if value.event_type == EventType::Sit => false,
                    _ => true,
                };

                if push {
                    // I'm not sure how to deal with mutable + immutable borrows 100% of the time
                    results.push(event);
                }

                continue;
            }

            if origin.is_some() {
                // Check if aiming
                if let Some(event) = is_aiming(&buffer, &origin, &range) {
                    // Dont add duplicate events
                    let push = match results.last() {
                        Some(ref value) if value.event_type == EventType::Aim => false,
                        _ => true,
                    };

                    if push {
                        results.push(event);
                    }
                    continue;
                }

                // Check if reloading
                if let Some(event) = is_reloading(&buffer, &origin, &range) {
                    // Dont add duplicate events
                    let push = match results.last() {
                        Some(ref value) if value.event_type == EventType::Reload => false,
                        _ => true,
                    };

                    if push {
                        results.push(event);
                    }
                    continue;
                }

                // Check if shooting
                if let Some(event) = is_firing(&buffer, &origin, &range) {
                    results.push(event);
                    continue;
                }
            }

            // Find origin
            origin = find_origin(&buffer);
        }
    }

    display_results(&results);
}

/// Search through the data and return an origin and range
fn find_origin(data: &Vec<(u64, Location)>) -> Option<Location> {
    // Search through data to find cluster
    // (Only the last 20 because it is not any of the standard gun events)

    let (mut i, mut average) = (data.len() - 1, // Counter
        Location(0f32, 0f32, 0f32));

    for _ in 0..SIZE {
        // Create a temp copy of the element
        let element = data[i].clone();
        average = average + element.1.clone();

        // Prevent underflows
        if i != 0 {
            i -= 1;
        }
    }
    average = average.div(SIZE as f32);

    Some(average)
}

/// Determine if the gun is sitting
fn is_sitting(data: &[(u64, Location)]) -> Option<Event> {
    let mut points: Vec<&(u64, Location)> = Vec::new();

    for point in data {
        if point.0 == 0u64 {
            continue;
        }

        if points.is_empty() {
            points.push(point);
            continue;
        }
        if point.1 == points.last().unwrap().1 {
            points.push(point);
        } else {

            if points.len() >= 5 {
                return Some(Event {
                    timestamp: points[0].0,
                    location: (point.1).clone(),
                    event_type: EventType::Sit,
                })
            }

            points.clear();
            continue;
        }

    }

    if points.len() >= 5 {
        return Some(Event {
            timestamp: points[0].0,
            location: (points[0].1).clone(),
            event_type: EventType::Sit,
        })
    }

    None
}

/// Determine if the gun is firing
fn is_firing(data: &Vec<(u64, Location)>, origin: &Option<Location>, range: &Range)
                                                              -> Option<Event> {
    let mut i = data.len() - 1;
    let origin = match *origin {
        Some(ref v) => v,
        None => return None
    };

    let mut results = Vec::with_capacity(6);

    for _ in 0..SIZE {
        let item = &data[i];

        // Compare the x values
        if (item.1).0 > origin.0 + range.0 && (item.1).1 < origin.1 + range.1
                                            && (item.1).2 < origin.2 + range.2 {
            results.push(item);
        } else {
            if results.len() > 3 {
                break;
            }

            results.clear();
        }

        if i != 0 {
            i -= 1;
        }
    }

    if results.len() > 3 {
        Some(Event {
            timestamp: results[0].0.clone(),
            location: results[0].1.clone(),
            event_type: EventType::Fire
         })
    } else {
        None
    }
}

/// Determine if the gun is aiming
fn is_aiming(data: &Vec<(u64, Location)>, origin: &Option<Location>, range: &Range)
                                                              -> Option<Event> {
    let mut i = data.len() - 1;
    let mut event: (u64, Location) = (0, Location(0f32, 0f32, 0f32));

    let origin = match *origin {
        Some(ref v) => v,
        None => return None
    };

    for _ in 0..SIZE {
        event = data[i].clone();

        if event.0 == 0u64 {
            return None;
        }

        // Check if it is in range
        if !range.in_range(origin, &event.1) {
            return None;
        }

        if i != 0 {
            i -= 1;
        }
    }

    Some(Event{
        timestamp: event.0,
        location: event.1,
        event_type: EventType::Aim,
    })
}

/// Determine if the gun is reloading
fn is_reloading(data: &Vec<(u64, Location)>, origin: &Option<Location>, range: &Range)
                                                              -> Option<Event> {
    let mut i = data.len() - 1;
    let origin = match *origin {
        Some(ref v) => v,
        None => return None
    };

    let mut results = Vec::with_capacity(15);

    for _ in 0..SIZE {
        let item = &data[i];

        if !range.in_range(origin, &item.1) && (item.1).1 > origin.1 + range.1 && (item.1).2 > origin.2 + range.2 {
            results.push(item);
        } else {
            if results.len() > 3 {
                break;
            }

            results.clear();
        }

        if i != 0 {
            i -= 1;
        }
    }

    if results.len() > 3 {
        Some(Event {
            timestamp: results[0].0.clone(),
            location: results[0].1.clone(),
            event_type: EventType::Reload
         })
    } else {
        None
    }
}

/// Display events in the event vector
fn display_results(events: &Vec<Event>) {
    if events.len() == 0 {
        return;
    }

    println!("\nOutput:");

    for event in events {
        // print each event in a readable format
        println!("Event: {:?} Location: {:?} Timestamp: {}",
                             event.event_type, event.location, event.timestamp);
    }
}

/// Handles the thread for reading data
///
/// Reads data from stdin and pushes the data through the channel into the main
/// thread for it to interpret
fn read_thread_handler(tx: mpsc::Sender<(u64, Location)>) {
    loop {
        let data: (u64, Location) = match read_location_data() {
            Ok(value) => value,
            Err(err) => {
                println!("Error: {}", err);
                continue
            },
        };

        // Check for exit conditions
        // (if timestamp is 1970)
        if data.0 == 0u64 {
            return ();
        }

        // Push the data
        if let Err(err) = tx.send(data) {
            println!("Error: Could not send input over channel, {}", err);
        }
    }
}

/// Parses input from stdin into a timestamp and location
///
/// Form of stdin input:
/// `{timestamp}: {x}, {y}, {z}\n`
///
/// # Returns
/// timestamp, location
fn read_location_data() -> Result<(u64, Location), &'static str> {
    let mut input: String = String::new();
    if let Err(_) = io::stdin().read_line(&mut input) {
        return Err("Could not read input from stdin");
    };

    // Check for an exit condition
    if input.trim() == "q" {
        return Ok((0, Location(0f32, 0f32, 0f32)));
    }

    let re: Regex = match Regex::new(r"(?x)
                                        (?P<time>\d+):\s # timestamp
                                        (?P<x>[+-]?\d+[.]?\d+),\s # x
                                        (?P<y>[+-]?\d+[.]?\d+),\s # y
                                        (?P<z>[+-]?\d+[.]?\d+)\n # z") {
        Ok(value) => value,
        Err(_) => return Err("Could not create regex pattern"),
    };

    let capture = match re.captures(&input) {
        None => {
            println!("{}", input);
            return Err("Error parsing input")},
        Some(value) => value
    };

    // This code doesn't handle parsing digits that cause an overflow
    let timestamp: u64 = capture.name("time").map_or(0u64, |s|
                                            s.as_str().parse::<u64>().unwrap());

    let location: Location = Location(
        match capture.name("x").map_or(0f32, |s| s.as_str().parse::<f32>()
                                                                    .unwrap()) {
            value if value.is_infinite() => return Err("Location overflow"),
            value => value
        },
        match capture.name("y").map_or(0f32, |s| s.as_str().parse::<f32>()
                                                                    .unwrap()) {
            value if value.is_infinite() => return Err("Location overflow"),
            value => value
        },
        match capture.name("z").map_or(0f32, |s| s.as_str().parse::<f32>()
                                                                    .unwrap()) {
            value if value.is_infinite() => return Err("Location overflow"),
            value => value
        },
    );

    // Return success
    Ok((timestamp, location))
}
