#include "gtest/gtest.h"
#include "RSVP.h"
#include "gmock/gmock.h"

using namespace ::testing;

TEST(HotelRSVP, Lakewood)
{
    RSVP rsvp("Regular: 16Mar2009(mon), 17Mar2009(tues), 18Mar2009(wed)");
    std::string expected("lakewood");

    ASSERT_EQ(rsvp.best_price(), expected);
}

TEST(HotelRSVP, Bridgewood)
{
    RSVP rsvp("Regular: 20Mar2009(fri), 21Mar2009(sat), 22Mar2009(sun)");

    ASSERT_EQ(rsvp.best_price(), "bridgewood");
}

TEST(HotelRSVP, Ridgewood)
{
    RSVP rsvp("Rewards: 26Mar2009(thur), 27Mar2009(fri), 28Mar2009(sat)");

    ASSERT_EQ(rsvp.best_price(), "ridgewood");
}
