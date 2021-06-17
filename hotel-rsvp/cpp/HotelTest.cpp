#include "gtest/gtest.h"
#include "Hotel.h"
#include "Client.h"
#include "gmock/gmock.h"

using namespace ::testing;
class AHotelRsvp: public Test {
public:
    Hotel* hotel;

    void SetUp() override {
        std::string* name = new std::string("lakewood");
        hotel = new Hotel(
            3, name, 11, 17, 3, 7
        );
    }

    const Client regular_client() {
        std::vector<Days> days{ Days::Sun, Days::Mon, Days::Tues};
        Client rc(ClientType::Regular, days);
        return rc;
    }

    const Client rewards_client() {
        std::vector<Days> days{ Days::Sun, Days::Mon, Days::Tues};
        Client wc(ClientType::Rewards, days);
        return wc;
    }
};

TEST_F(AHotelRsvp, HasCorrectPriceForRegulars) {
    const Client reg_client = regular_client();
    ASSERT_EQ(hotel->price(reg_client), 39);
}

TEST_F(AHotelRsvp, HasCorrectPriceForRewards) {
    const Client rew_client = rewards_client();
    ASSERT_EQ(hotel->price(rew_client), 13);
}
