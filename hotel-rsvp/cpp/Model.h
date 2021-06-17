#ifndef Model_h
#define Model_h

#include <string>
#include <vector>
#include "Hotel.h"
#include "Client.h"

struct Response {
    std::string name_;
    unsigned int rate_;
    unsigned long price_;

    public: 

    Response() {
        name_ = "";
        rate_ = 0;
        price_ = ULONG_MAX;
    }

    Response(std::string name, unsigned int rate, unsigned long price) {
        name_ = name;
        rate_ = rate;
        price_ = price;
    }

    void update(
        const std::string& name,
        unsigned int rate,
        unsigned long price) 
    {
        if (price < price_) {
            name_ = name;
            rate_ = rate;
            price_ = price;
        } else if (price == price_ && rate > rate_) {
            name_ = name;
            rate_ = rate;
            price_ = price;
        }
    }

    Response best_price(std::vector<Hotel>& hotel, const Client& client);

    std::string& name() {
        return name_;
    }

};

class ErrorFindingBestHotel {
    public: 
    ErrorFindingBestHotel() {}
};

#endif