use reqwest::blocking::get;
use select::document::Document;
use select::predicate::{Class, Name};
use chrono::NaiveDate;
use geo::algorithm::haversine_distance::HaversineDistance;
use geo::point;
use cached::proc_macro::cached;
use std::sync::Arc;
use std::time::Instant;
use geocoding::{Forward, Opencage};
use std::io;
use std::io::Write;
use rayon::prelude::*;

fn main() {
    let url = "https://www.csd-termine.de/tabelle";
    let api_key = Arc::from("a8b455b3e8944081b20f0db5755d2df1");
    let user_city_name = get_city_name_from_user();
    let user_city_coordinates = match get_coordinates(Arc::clone(&api_key), Arc::from(user_city_name.clone())) {
        Some(coords) => coords,
        None => {
            eprintln!("Failed to retrieve coordinates for {}", user_city_name);
            return;
        }
    };
    extract_locations_and_dates_from_url(Arc::clone(&api_key), url, &user_city_name, user_city_coordinates);
}

fn get_city_name_from_user() -> String {
    print!("Enter the name of your city: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    input.trim().to_string()
}

fn extract_locations_and_dates_from_url(api_key: Arc<str>, url: &str, user_city_name: &str, user_city_coordinates: geo::Point<f64>) {
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
            let location_text = a_tag.text();
            let date_text = date_tag.text();

            if let Some(location_match) = location_text.split_once("CSD ") {
                let location_part = location_match.1.trim();
                let date_part = date_text.trim();

                if let Some((city_name, date)) = parse_location_and_date(location_part, date_part) {
                    if date >= current_date {
                        cities.push((city_name.to_string(), date));
                    }
                }
            }
        }
    }

    let mut city_distances = cities
        .par_iter()
        .filter_map(|(city_name, date)| {
            get_coordinates(Arc::clone(&api_key), Arc::from(city_name.clone())).map(|city_coordinates| {
                let distance = calculate_distance(&city_coordinates, &user_city_coordinates);
                (city_name.clone(), (distance, *date))
            })
        })
        .collect::<Vec<_>>();

    city_distances.sort_by(|a, b| a.1 .0.partial_cmp(&b.1 .0).unwrap());

    for (city_name, (distance, date)) in city_distances {
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
    let parsed_date = NaiveDate::parse_from_str(date, "%d.%m.%y").ok()?;

    let city_name = if location.ends_with(" 2024") {
        location.trim_end_matches(" 2024").to_string()
    } else {
        location.to_string()
    };

    Some((city_name, parsed_date))
}

#[cached(time = 1800)]
fn get_coordinates(api_key: Arc<str>, city_name: Arc<str>) -> Option<geo::Point<f64>> {
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
