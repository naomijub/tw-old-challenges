#include "gtest/gtest.h"
#include "Client.h"
#include "gmock/gmock.h"

using namespace ::testing;

TEST(ClientInputParse, RegularWeekdays)
{
    std::vector<Days> days = {Days::Mon, Days::Tues, Days::Wed};
    auto client = Client();
    client.parse_input("Regular: 16Mar2009(mon), 17Mar2009(tues), 18Mar2009(wed)");
    Client expected(ClientType::Regular, days);

    ASSERT_EQ(client, expected);
}

TEST(ClientInputParse, RegularWeekends)
{
    std::vector<Days> days = {Days::Fri, Days::Sat, Days::Sun};
    auto client = Client();
    client.parse_input("Regular: 20Mar2009(fri), 21Mar2009(sat), 22Mar2009(sun)");
    Client expected(ClientType::Regular, days);

    ASSERT_EQ(client, expected);
}

TEST(ClientInputParse, RewardsWeekends)
{
    std::vector<Days> days = {Days::Thur, Days::Fri, Days::Sat};
    auto client = Client();
    client.parse_input("Rewards: 26Mar2009(thur), 27Mar2009(fri), 28Mar2009(sat)");
    Client expected(ClientType::Rewards, days);

    ASSERT_EQ(client, expected);
}
