import re
import requests
from bs4 import BeautifulSoup
from geopy.geocoders import Nominatim
from geopy.distance import geodesic
from datetime import datetime
import time

# this is the Python based this projekt is based on.
# it isn't ready you need to install the libarys abouth.
# prs to improve this file are welcome

def extract_locations_and_dates_from_url(url):
    start_time = time.time()  # Startzeit erfassen
    
    response = requests.get(url)
    
    if response.status_code == 200:
        html_content = response.text
    else:
        print(f"Error retrieving URL: {response.status_code}")
        return
    
    soup = BeautifulSoup(html_content, 'html.parser')
    rows = soup.find_all('tr')
    
    bamberg_coordinates = (49.8917, 10.8928)

    cities = []

    for row in rows:
        a_tag = row.find('a')
        date_tag = row.find('span', class_='date')
        if a_tag and date_tag:
            location_match = re.search(r'CSD (.+?) \d{4}', a_tag.text.strip())
            if location_match:
                city_name = location_match.group(1)
                date_str = date_tag.text.strip()
                date = datetime.strptime(date_str, '%d.%m.%y')
                cities.append({"name": city_name, "date": date})

    city_distances = {}
    total_cities = len(cities)
    current_date = datetime.now()
    
    for index, city in enumerate(cities, 1):
        city_name = city["name"]
        city_coordinates = get_coordinates(city_name)
        if city_coordinates:
            distance = calculate_distance(city_coordinates, bamberg_coordinates)
            if city["date"] >= current_date:  # Check if the date has passed
                city_distances[city_name] = {"distance": distance, "date": city["date"]}
            print(f"Progress: {index}/{total_cities}")

    sorted_cities = sorted(city_distances.items(), key=lambda x: x[1]["distance"], reverse=True)

    for city_name, data in sorted_cities:
        print(f"Am {data['date'].strftime('%d.%m.%y')}, in der Stadt {city_name} ist {data['distance']:.2f} km von Bamberg entfernt.")
    
    end_time = time.time()  # Endzeit erfassen
    execution_time = end_time - start_time  # Gesamtausführungszeit berechnen
    print(f"Gesamtausführungszeit: {execution_time:.2f} Sekunden")

def get_coordinates(city_name):
    geolocator = Nominatim(user_agent="city_locator")
    location = geolocator.geocode(city_name)
    if location:
        return location.latitude, location.longitude
    else:
        print(f"Error retrieving coordinates for {city_name}")
        return None

def calculate_distance(city_coordinates, bamberg_coordinates):
    return geodesic(city_coordinates, bamberg_coordinates).kilometers

url = 'https://www.csd-termine.de/2024'
extract_locations_and_dates_from_url(url)
