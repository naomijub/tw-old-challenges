use crate::model::{Client, ClientType, HotelRsvp};
use chrono::prelude::*;

pub fn get_client_type(client_type: &str) -> Result<ClientType, String> {
    match client_type.to_lowercase().as_str() {
        "regular" => Ok(ClientType::Regular),
        "rewards" => Ok(ClientType::Rewards),
        _ => Err(String::from("Client type must be rewards or regular")),
    }
}

pub fn get_client_days(days: Vec<&str>) -> Result<Vec<Date<Utc>>, String> {
    days.into_iter()
        .map(|day| {
            format!("{} 00:00:00", day)
                .replace("tues", "tue")
                .replace("thur", "thu")
        })
        .map(|day| {
            Utc.datetime_from_str(&day, "%d%b%Y(%a) %T")
                .map_err(|e| format!("{:?}", e))
        })
        .map(|d| {
            if let Ok(day) = d {
                Ok(day.date())
            } else {
                Err(String::from(
                    "Date should be formatted dayMonthYear(weekday)",
                ))
            }
        })
        .collect()
}

pub fn best_price(hotels: Vec<HotelRsvp>, client: Client) -> String {
    hotels
        .into_iter()
        .map(|hotel| (hotel.name.clone(), hotel.price(&client), hotel.rate))
        .min_by(|x, y| x.1.cmp(&y.1).then(y.2.cmp(&x.2)))
        .unwrap()
        .0
}

pub fn parse_client_date(client_date: &str) -> Result<Client, String> {
    let pattern: Vec<&str> = client_date.split(":").collect();
    let client_type = pattern[0];
    let days: Vec<&str> = pattern[1].split(",").map(|d| d.trim()).collect();

    Client::new(client_type, days)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_type() {
        assert_eq!(ClientType::Regular, get_client_type("RegulAR").unwrap());
        assert_eq!(ClientType::Rewards, get_client_type("reWArDs").unwrap());
    }

    #[test]
    fn wrong_client_type() {
        let err = get_client_type("Type").err();
        assert_eq!(
            err,
            Some("Client type must be rewards or regular".to_string())
        );
    }

    #[test]
    fn all_valid_days() {
        let days = vec!["16Mar2009(mon)", "17Mar2009(tues)", "18Mar2009(wed)"];
        let actual = get_client_days(days).unwrap();
        let expected = vec![
            Utc.datetime_from_str("16Mar2009(mon) 00:00:00", "%e%b%Y(%a) %T")
                .unwrap()
                .date(),
            Utc.datetime_from_str("17Mar2009(tue) 00:00:00", "%e%b%Y(%a) %T")
                .unwrap()
                .date(),
            Utc.datetime_from_str("18Mar2009(wed) 00:00:00", "%e%b%Y(%a) %T")
                .unwrap()
                .date(),
        ];

        assert_eq!(actual, expected)
    }

    #[test]
    fn invalid_day() {
        let days = vec!["notADay"];
        let actual = get_client_days(days).err();
        let expected = Some("Date should be formatted dayMonthYear(weekday)".to_string());

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_parse_client() {
        let s = "Regular: 16Mar2009(mon), 17Mar2009(tues), 18Mar2009(wed)";
        let client = parse_client_date(s).unwrap();
        let expected = Client {
            client_type: ClientType::Regular,
            days: vec![
                Utc.datetime_from_str("16Mar2009(mon) 00:00:00", "%e%b%Y(%a) %T")
                    .unwrap()
                    .date(),
                Utc.datetime_from_str("17Mar2009(tue) 00:00:00", "%e%b%Y(%a) %T")
                    .unwrap()
                    .date(),
                Utc.datetime_from_str("18Mar2009(wed) 00:00:00", "%e%b%Y(%a) %T")
                    .unwrap()
                    .date(),
            ],
        };

        assert_eq!(client, expected);
    }
}
