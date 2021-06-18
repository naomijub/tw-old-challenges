#include "Hotel.h"

unsigned long Hotel::price(const Client &client)
{
    long price = 0;
    auto type = client.type_;
    auto days = client.days_;

    for (Days day : days)
    {
        if (day >= 5 && type == ClientType::Regular)
        {
            price += regular_weekend_fare_;
        }
        else if (day < 5 && type == ClientType::Regular)
        {
            price += regular_weekday_fare_;
        }
        else if (day >= 5 && type == ClientType::Rewards)
        {
            price += rewards_weekend_fare_;
        }
        else if (day < 5 && type == ClientType::Rewards)
        {
            price += rewards_weekday_fare_;
        }
    }

    return price;
}

std::string Hotel::name() { return *name_; }
unsigned int Hotel::rate() { return rate_; };