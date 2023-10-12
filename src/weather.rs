use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetCurrent {
    pub location: Location,
    pub current:  Current,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetForecast {
    pub location: Location,
    pub current:  Current,
    pub forecast: Forecast,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub name:            String,
    pub region:          String,
    pub country:         String,
    pub lat:             f32,
    pub lon:             f32,
    pub tz_id:           String,
    pub localtime_epoch: u32, 
    pub localtime:       String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Current {
    pub last_updated_epoch: u32,
    pub last_updated:       String,
    pub temp_c:             f32,
    pub temp_f:             f32,
    pub is_day:             u32,
    pub condition:          Condition,
    pub wind_mph:           f32,
    pub wind_kph:           f32,
    pub wind_degree:        u32,
    pub wind_dir:           String,
    pub pressure_mb:        f32,
    pub pressure_in:        f32,
    pub precip_mm:          f32,
    pub precip_in:          f32,
    pub humidity:           u32,
    pub cloud:              u32,
    pub feelslike_c:        f32,
    pub feelslike_f:        f32,
    pub vis_km:             f32,
    pub vis_miles:          f32,
    pub uv:                 f32,
    pub gust_mph:           f32,
    pub gust_kph:           f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Forecast {
    pub forecastday: Vec<Forecastday>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Forecastday {
    date:       String,
    date_epoch: u32,
    day:        Day,
    astro:      Astro,
    hour:       Vec<Hour>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Day {
    maxtemp_c:      f32,
    maxtemp_f:      f32,
    mintemp_c:      f32,
    mintemp_f:      f32,
    avgtemp_c:      f32,
    avgtemp_f:      f32,
    maxwind_mph:    f32,
    maxwind_kph:    f32,
    totalprecip_mm: f32,
    totalprecip_in: f32,
    avgvis_km:      f32,
    avgvis_miles:   f32,
    avghumidity:    f32,
    condition:      Condition,
    uv:             f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Astro {
    sunrise:           String,
    sunset:            String,
    moonrise:          String,
    moonset:           String,
    moon_phase:        String, 
    moon_illumination: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Hour {
    time_epoch:     u32,
    time:           String,
    temp_c:         f32,
    temp_f:         f32,
    condition:      Condition,
    wind_mph:       f32,
    wind_kph:       f32,
    wind_degree:    u32,
    wind_dir:       String,
    pressure_mb:    f32,
    pressure_in:    f32,
    precip_mm:      f32,
    precip_in:      f32,
    humidity:       u32,
    cloud:          u32,
    feelslike_c:    f32,
    feelslike_f:    f32,
    windchill_c:    f32,
    windchill_f:    f32,
    heatindex_c:    f32,
    heatindex_f:    f32,
    dewpoint_c:     f32,
    dewpoint_f:     f32,
    will_it_rain:   u32,
    will_it_snow:   u32,
    is_day:         u32,
    vis_km:         f32,
    vis_miles:      f32,
    chance_of_rain: u32,
    chance_of_snow: u32,
    gust_mph:       f32,
    gust_kph:       f32,
    uv:             f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Condition {
    pub text: String,
    pub icon: String,
    pub code: u32,
}
