#ifndef Hotel_h
#define Hotel_h

#include <vector>
#include <string>
#include "Client.h"

class Hotel
{
private:
    unsigned int rate_;
    const std::string *name_;
    unsigned long regular_weekday_fare_;
    unsigned long regular_weekend_fare_;
    unsigned long rewards_weekday_fare_;
    unsigned long rewards_weekend_fare_;

public:
    Hotel(
        unsigned int rate,
        const std::string *name,
        unsigned long regular_weekday_fare,
        unsigned long regular_weekend_fare,
        unsigned long rewards_weekday_fare,
        unsigned long rewards_weekdend_fare) : rate_(rate), name_(name), regular_weekday_fare_(regular_weekday_fare),
                                               regular_weekend_fare_(regular_weekend_fare), rewards_weekday_fare_(rewards_weekday_fare),
                                               rewards_weekend_fare_(rewards_weekdend_fare){};

    unsigned long price(const Client &client);
    std::string name();
    unsigned int rate();
};

#endif