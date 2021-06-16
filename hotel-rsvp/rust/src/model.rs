use std::ops::Range;

use chrono::prelude::*;

use crate::core::{get_client_days, get_client_type};

#[derive(PartialEq, Debug, Clone)]
pub struct HotelRsvp {
    pub name: String,
    pub rate: u8,
    fares: ClientRsvp,
    blackout_days: Range<Date<Utc>>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct ClientRsvp {
    regular: ClientFares,
    rewards: ClientFares,
}

#[derive(PartialEq, Debug, Clone)]
pub struct ClientFares {
    weekday: u64,
    weekend: u64,
}

impl HotelRsvp {
    pub fn new(
        name: String,
        rate: u8,
        regular_weekday: u64,
        regular_weekend: u64,
        rewards_weekday: u64,
        rewards_weekend: u64,
        blackout_days: Range<Date<Utc>>,
    ) -> Self {
        HotelRsvp {
            name,
            rate,
            fares: ClientRsvp {
                regular: ClientFares {
                    weekday: regular_weekday,
                    weekend: regular_weekend,
                },
                rewards: ClientFares {
                    weekday: rewards_weekday,
                    weekend: rewards_weekend,
                },
            },
            blackout_days,
        }
    }

    pub fn price(&self, client: &Client) -> u64 {
        let client_type = &client.client_type;
        let days = &client.days;

        days.iter()
            .map(|day| {
                if self.blackout_days.contains(&day) {
                    self.get_fare_by_client_type(&ClientType::Regular, day)
                } else {
                    self.get_fare_by_client_type(client_type, day)
                }
            })
            .sum()
    }

    fn get_fare_by_client_type(&self, client_type: &ClientType, day: &Date<Utc>) -> u64 {
        match (client_type, day.weekday().num_days_from_monday()) {
            (&ClientType::Regular, x) if x < 5 => self.fares.regular.weekday,
            (&ClientType::Regular, x) if x >= 5 => self.fares.regular.weekend,
            (&ClientType::Rewards, x) if x < 5 => self.fares.rewards.weekday,
            (&ClientType::Rewards, x) if x >= 5 => self.fares.rewards.weekend,
            _ => 0,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ClientType {
    Regular,
    Rewards,
}

#[derive(Debug, PartialEq)]
pub struct Client {
    pub(crate) client_type: ClientType,
    pub(crate) days: Vec<Date<Utc>>,
}

impl Client {
    pub fn new(client_type: &str, days: Vec<&str>) -> Result<Client, String> {
        let client_type = get_client_type(client_type)?;
        let days = get_client_days(days)?;

        Ok(Client { client_type, days })
    }
}

#[cfg(test)]
mod tests {
    use chrono::Duration;

    use super::*;

    #[test]
    fn new_hotel_rsvp() {
        let today = Utc::today();
        let range = today.checked_sub_signed(Duration::days(3)).unwrap()..today;
        let actual = HotelRsvp::new("test".to_string(), 3, 10, 20, 30, 40, range.clone());
        let expected = HotelRsvp {
            name: "test".to_string(),
            rate: 3,
            fares: ClientRsvp {
                regular: ClientFares {
                    weekday: 10,
                    weekend: 20,
                },
                rewards: ClientFares {
                    weekday: 30,
                    weekend: 40,
                },
            },
            blackout_days: range,
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn new_client() {
        let actual = Client::new("regular", vec!["16Mar2009(mon)", "17Mar2009(tues)"]).unwrap();
        let expected = Client {
            client_type: ClientType::Regular,
            days: vec![
                Utc.datetime_from_str("16Mar2009(mon) 00:00:00", "%e%b%Y(%a) %T")
                    .unwrap()
                    .date(),
                Utc.datetime_from_str("17Mar2009(tue) 00:00:00", "%e%b%Y(%a) %T")
                    .unwrap()
                    .date(),
            ],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn wrong_client_type() {
        let actual = Client::new("special", vec!["sun", "mon"]).err();
        let expected = Some("Client type must be rewards or regular".to_string());

        assert_eq!(actual, expected);
    }

    #[test]
    fn wrong_day() {
        let actual = Client::new("rewards", vec!["sun", "mon", "foo"]).err();
        let expected = Some("Date should be formatted dayMonthYear(weekday)".to_string());

        assert_eq!(actual, expected);
    }

    #[test]
    fn price_weekdays_regular() {
        let today = Utc::today();
        let range = today.checked_sub_signed(Duration::days(3)).unwrap()..today;
        let hotel = HotelRsvp::new("test".to_string(), 3, 11, 17, 2, 5, range.clone());
        let client = Client::new("regular", vec!["16Mar2009(mon)", "17Mar2009(tues)"]).unwrap();

        let price = hotel.price(&client);

        assert_eq!(price, 22)
    }

    #[test]
    fn price_weekends_regular() {
        let today = Utc::today();
        let range = today.checked_sub_signed(Duration::days(3)).unwrap()..today;
        let hotel = HotelRsvp::new("test".to_string(), 3, 11, 17, 2, 5, range.clone());
        let client = Client::new("regular", vec!["21Mar2009(sat)", "22Mar2009(sun)"]).unwrap();

        let price = hotel.price(&client);

        assert_eq!(price, 34)
    }

    #[test]
    fn price_week_regular() {
        let today = Utc::today();
        let range = today.checked_sub_signed(Duration::days(3)).unwrap()..today;
        let hotel = HotelRsvp::new("test".to_string(), 3, 11, 17, 2, 5, range.clone());
        let client = Client::new("regular", vec!["21Mar2009(sat)", "20Mar2009(fri)"]).unwrap();

        let price = hotel.price(&client);

        assert_eq!(price, 28)
    }

    #[test]
    fn price_weekdays_rewards() {
        let today = Utc::today();
        let range = today.checked_sub_signed(Duration::days(3)).unwrap()..today;
        let hotel = HotelRsvp::new("test".to_string(), 3, 11, 17, 2, 5, range.clone());
        let client = Client::new("rewards", vec!["16Mar2009(mon)", "17Mar2009(tues)"]).unwrap();

        let price = hotel.price(&client);

        assert_eq!(price, 4)
    }

    #[test]
    fn price_weekends_rewards() {
        let today = Utc::today();
        let range = today.checked_sub_signed(Duration::days(3)).unwrap()..today;
        let hotel = HotelRsvp::new("test".to_string(), 3, 11, 17, 2, 5, range.clone());
        let client = Client::new("rewards", vec!["21Mar2009(sat)", "22Mar2009(sun)"]).unwrap();

        let price = hotel.price(&client);

        assert_eq!(price, 10)
    }

    #[test]
    fn price_week_rewards() {
        let today = Utc::today();
        let range = today.checked_sub_signed(Duration::days(3)).unwrap()..today;
        let hotel = HotelRsvp::new("test".to_string(), 3, 11, 17, 2, 5, range.clone());
        let client = Client::new("rewards", vec!["21Mar2009(sat)", "20Mar2009(fri)"]).unwrap();

        let price = hotel.price(&client);

        assert_eq!(price, 7)
    }
}
