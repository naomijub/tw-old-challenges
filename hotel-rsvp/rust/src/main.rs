use chrono::{prelude::*, Duration};
use model::HotelRsvp;

use crate::core::{best_price, parse_client_date};

mod core;
mod model;

fn main() {
    let hotels = generate_hotels();

    let client =
        parse_client_date("Regular: 16Mar2009(mon), 17Mar2009(tues), 18Mar2009(wed)").unwrap();
    let hotel = best_price(hotels.clone(), client);
    println!(
        "FOR Regular: 16Mar2009(mon), 17Mar2009(tues), 18Mar2009(wed): {:?}",
        hotel
    );

    let client =
        parse_client_date("Regular: 20Mar2009(fri), 21Mar2009(sat), 22Mar2009(sun)").unwrap();
    let hotel = best_price(hotels.clone(), client);
    println!(
        "FOR Regular: 20Mar2009(fri), 21Mar2009(sat), 22Mar2009(sun) {:?}",
        hotel
    );

    let client =
        parse_client_date("Rewards: 26Mar2009(thur), 27Mar2009(fri), 28Mar2009(sat)").unwrap();
    let hotel = best_price(hotels.clone(), client);
    println!(
        "FOR Rewards: 26Mar2009(thur), 27Mar2009(fri), 28Mar2009(sat): {:?}",
        hotel
    );
}

pub fn generate_hotels() -> Vec<HotelRsvp> {
    let today = Utc::today();
    let empty_range = today.checked_add_signed(Duration::days(3)).unwrap()..today;
    vec![
        HotelRsvp::new(
            "lakewood".to_string(),
            3,
            110,
            90,
            80,
            80,
            empty_range.clone(),
        ),
        HotelRsvp::new(
            "bridgewood".to_string(),
            4,
            160,
            60,
            110,
            50,
            empty_range.clone(),
        ),
        HotelRsvp::new(
            "ridgewood".to_string(),
            5,
            220,
            150,
            100,
            40,
            empty_range.clone(),
        ),
    ]
}

pub fn generate_hotels_with_blackouts() -> Vec<HotelRsvp> {
    let july_to_august = "2021-07-1T00:00:00Z"
        .parse::<DateTime<Utc>>()
        .unwrap()
        .date()
        .."2021-07-1T00:00:00Z"
            .parse::<DateTime<Utc>>()
            .unwrap()
            .date()
            .checked_add_signed(Duration::days(92))
            .unwrap();
    let end_year = "2020-12-23T00:00:00Z"
        .parse::<DateTime<Utc>>()
        .unwrap()
        .date()
        .."2020-12-23T00:00:00Z"
            .parse::<DateTime<Utc>>()
            .unwrap()
            .date()
            .checked_add_signed(Duration::days(9))
            .unwrap();
    let today = Utc::today();
    let empty_range = today.checked_add_signed(Duration::days(3)).unwrap()..today;
    vec![
        HotelRsvp::new(
            "lakewood".to_string(),
            3,
            110,
            90,
            80,
            80,
            empty_range.clone(),
        ),
        HotelRsvp::new(
            "bridgewood".to_string(),
            4,
            160,
            60,
            110,
            50,
            end_year.clone(),
        ),
        HotelRsvp::new(
            "ridgewood".to_string(),
            5,
            220,
            150,
            100,
            40,
            july_to_august.clone(),
        ),
    ]
}

#[cfg(test)]
mod int_tests {
    use crate::core::{best_price, parse_client_date};

    use super::*;

    #[test]
    fn no_blackouts_lakewood() {
        let hotels = generate_hotels();
        let client =
            parse_client_date("Regular: 16Mar2009(mon), 17Mar2009(tues), 18Mar2009(wed)").unwrap();

        let hotel = best_price(hotels, client);
        assert_eq!(hotel, "lakewood")
    }

    #[test]
    fn no_blackouts_bridgewood() {
        let hotels = generate_hotels();
        let client =
            parse_client_date("Regular: 20Mar2009(fri), 21Mar2009(sat), 22Mar2009(sun)").unwrap();

        let hotel = best_price(hotels, client);
        assert_eq!(hotel, "bridgewood")
    }

    #[test]
    fn no_blackouts_ridgewood() {
        let hotels = generate_hotels();
        let client =
            parse_client_date("Rewards: 26Mar2009(thur), 27Mar2009(fri), 28Mar2009(sat)").unwrap();

        let hotel = best_price(hotels, client);
        assert_eq!(hotel, "ridgewood")
    }

    #[test]
    fn blackouts_ridgewood() {
        let hotels = generate_hotels();
        let client = parse_client_date("Rewards: 25Dec2020(fri), 26Dec2020(sat)").unwrap();

        let hotel = best_price(hotels, client);
        assert_eq!(hotel, "ridgewood")
    }

    #[test]
    fn blackouts_lakewood() {
        let hotels = generate_hotels();
        let client = parse_client_date("Rewards: 18Aug2021(wed), 19Aug2021(thur)").unwrap();

        let hotel = best_price(hotels, client);
        assert_eq!(hotel, "lakewood")
    }
}
