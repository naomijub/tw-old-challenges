#include "gtest/gtest.h"
#include "Model.h"
#include "gmock/gmock.h"

using namespace ::testing;

TEST(ResponseUpdate, UpdateForLesserPrice) {
    auto response = Response("teste", 3, 100);
    response.update("atualizado", 5, 95);

    ASSERT_EQ(response.name(), "atualizado");
}

TEST(ResponseUpdate, NoUpdateForSamePriceSameRate) {
    auto response = Response("teste", 3, 100);
    response.update("atualizado", 3, 100);

    ASSERT_EQ(response.name(), "teste");
}

TEST(ResponseUpdate, UpdateForSamePriceGreaterRate) {
    auto response = Response("teste", 3, 100);
    response.update("atualizado", 5, 100);

    ASSERT_EQ(response.name(), "atualizado");
}

TEST(ResponseUpdate, NoUpdateForGreaterPrice) {
    auto response = Response("teste", 3, 100);
    response.update("atualizado", 5, 105);

    ASSERT_EQ(response.name(), "teste");
}

#include <vector>

class ABestPriceRsvp: public Test {
public:
    std::vector<Hotel> hotels;
    Response resp;

    void SetUp() override {
        std::string* lakename = new std::string("lakewood");
        Hotel lakewood(
            3, lakename, 110, 90, 80, 80
        );
        hotels.push_back(lakewood);

        std::string* bridgename = new std::string("bridgewood");
        Hotel bridgewood(
            4, bridgename, 160, 60, 110, 50
        );
        hotels.push_back(bridgewood);

        std::string* ridgename = new std::string("ridgewood");
        Hotel ridgewood(
            5, ridgename, 220, 150, 100, 40
        );
        hotels.push_back(ridgewood);
        resp = Response();
    }

    const Client weekday_regular_client() {
        std::vector<Days> days{ Days::Wed, Days::Mon, Days::Tues};
        Client rc(ClientType::Regular, days);
        return rc;
    }

    const Client weekend_regular_client() {
        std::vector<Days> days{ Days::Fri, Days::Sat, Days::Sun};
        Client rc(ClientType::Regular, days);
        return rc;
    }

    const Client rewards_client() {
        std::vector<Days> days{ Days::Thur, Days::Fri, Days::Sat};
        Client wc(ClientType::Rewards, days);
        return wc;
    }
};

#include <iostream>

TEST_F(ABestPriceRsvp, WeekDayRegularIsLakewood) {
    const Client reg_client = weekday_regular_client();
    Response result = resp.best_price(hotels, reg_client);
    auto expected = "lakewood";

    ASSERT_EQ(result.name(), expected);
}

TEST_F(ABestPriceRsvp, WeekDayRegularIsBridgewood) {
    const Client reg_client = weekend_regular_client();
    Response result = resp.best_price(hotels, reg_client);
    auto expected = "bridgewood";

    ASSERT_EQ(result.name(), expected);
}

TEST_F(ABestPriceRsvp, WeekDayRegularIsRidgewood) {
    const Client reg_client = rewards_client();
    Response result = resp.best_price(hotels, reg_client);
    auto expected = "ridgewood";

    ASSERT_EQ(result.name(), expected);
}