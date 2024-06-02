use reqwest::blocking::get;
use select::document::Document;
use select::predicate::{Class, Name};
use chrono::NaiveDate;
use geo::algorithm::haversine_distance::HaversineDistance;
use geo::point;
use std::collections::HashMap;
use std::time::Instant;
use geocoding::{Forward, Opencage};
use std::io;
use std::io::Write;

fn main() {
    let url = "https://www.csd-termine.de/tabelle";
    let user_city_name = get_city_name_from_user();
    let user_city_coordinates = match get_coordinates(&user_city_name) {
        Some(coords) => coords,
        None => {
            eprintln!("Failed to retrieve coordinates for {}", user_city_name);
            return;
        }
    };
    extract_locations_and_dates_from_url(url, &user_city_name, user_city_coordinates);
}

fn get_city_name_from_user() -> String {
    print!("Enter the name of your city: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    input.trim().to_string()
}

fn extract_locations_and_dates_from_url(url: &str, user_city_name: &str, user_city_coordinates: geo::Point<f64>) {
    let start_time = Instant::now();  // Startzeit erfassen
    
    let response = get(url);
    let html_content = match response {
        Ok(res) => res.text().unwrap_or_else(|_| String::new()),
        Err(err) => {
            eprintln!("Error retrieving URL: {}", err);
            return;
        }
    };
    
    let document = Document::from(html_content.as_str());
    let rows = document.find(Name("tr"));

    let mut cities = Vec::new();
    let current_date = chrono::Utc::now().naive_utc().date();

    for row in rows {
        if let (Some(a_tag), Some(date_tag)) = (row.find(Name("a")).next(), row.find(Class("date")).next()) {
            if let Some(location_match) = a_tag.text().split("CSD ").nth(1) {
                if let Some((city_name, date)) = parse_location_and_date(location_match.trim(), date_tag.text().trim()) {
                    if date >= current_date {
                        cities.push((city_name.to_string(), date));
                    }
                }
            }
        }
    }

    let mut city_distances = HashMap::new();
    let total_cities = cities.len();

    for (index, (city_name, date)) in cities.iter().enumerate() {
        if let Some(city_coordinates) = get_coordinates(city_name) {
            let distance = calculate_distance(&city_coordinates, &user_city_coordinates);
            city_distances.insert(city_name.clone(), (distance, *date));
            println!("Progress: {}/{}", index + 1, total_cities);
        }
    }

    let mut sorted_cities: Vec<_> = city_distances.iter().collect();
    sorted_cities.sort_by(|a, b| b.1 .0.partial_cmp(&a.1 .0).unwrap());

    for (city_name, (distance, date)) in sorted_cities {
        println!(
            "Am {}, in der Stadt {} ist {:.2} km von {} entfernt.",
            date.format("%d.%m.%y"),
            city_name,
            distance,
            user_city_name
        );
    }
    
    let end_time = Instant::now();  // Endzeit erfassen
    let execution_time = end_time.duration_since(start_time).as_secs_f64();  // Gesamtausführungszeit berechnen
    println!("Gesamtausführungszeit: {:.2} Sekunden", execution_time);
}

fn parse_location_and_date(location: &str, date: &str) -> Option<(String, NaiveDate)> {
    let city_name = location.split_whitespace().next()?.to_string();
    let date = NaiveDate::parse_from_str(date, "%d.%m.%y").ok()?;
    Some((city_name, date))
}

fn get_coordinates(city_name: &str) -> Option<geo::Point<f64>> {
    let api_key = "a8b455b3e8944081b20f0db5755d2df1";
    let geocoder = Opencage::new(api_key.to_string());
    match geocoder.forward(&city_name) {
        Ok(geocoded) => geocoded.first().map(|location| {
            point!(x: location.x(), y: location.y())
        }),
        Err(err) => {
            eprintln!("Error retrieving coordinates for {}: {}", city_name, err);
            None
        }
    }
}

fn calculate_distance(city_coordinates: &geo::Point<f64>, user_city_coordinates: &geo::Point<f64>) -> f64 {
    city_coordinates.haversine_distance(user_city_coordinates) / 1000.0  // Convert to kilometers
}
