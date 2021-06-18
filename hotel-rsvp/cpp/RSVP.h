#ifndef RSVP_h
#define RSVP_h

#include <string>
#include "Model.h"
#include "Hotel.h"
#include "Client.h"

struct RSVP
{
    std::string input;
    std::vector<Hotel> hotels;

    RSVP(std::string in) : input(in)
    {
        std::string *lakename = new std::string("lakewood");
        Hotel lakewood(
            3, lakename, 110, 90, 80, 80);
        std::string *bridgename = new std::string("bridgewood");
        Hotel bridgewood(
            4, bridgename, 160, 60, 110, 50);
        std::string *ridgename = new std::string("ridgewood");
        Hotel ridgewood(
            5, ridgename, 220, 150, 100, 40);

        hotels.push_back(lakewood);
        hotels.push_back(bridgewood);
        hotels.push_back(ridgewood);
    };

public:
    std::string best_price()
    {
        Response *resp = new Response();
        auto client = Client();
        client.parse_input(input);

        auto result = resp->best_price(hotels, client).name();
        delete resp;
        return result;
    }
};

#endif