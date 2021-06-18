#include "Client.h"
#include "Exceptions.h"

ClientType parse_client_type(const std::string &client_type_input);
std::vector<Days> parse_client_days(const std::string &client_days_input);

void Client::parse_input(const std::string input)
{
    std::string client_type_input = input.substr(0, input.find(":"));
    std::string client_days_input = input.substr(input.find(":"));

    ClientType client_type = parse_client_type(client_type_input);
    std::vector<Days> days = parse_client_days(client_days_input);
    type_ = client_type;
    days_ = days;
}

ClientType parse_client_type(const std::string &client_type_input)
{
    if (client_type_input == "Regular")
    {
        return ClientType::Regular;
    }
    else if (client_type_input == "Rewards")
    {
        return ClientType::Rewards;
    }
    else
    {
        throw UnknownClient();
    }
}

std::vector<Days> parse_client_days(const std::string &client_days_input)
{
    std::vector<Days> days;
    std::vector<std::string> days_tokens;
    std::stringstream ss(client_days_input);

    std::string intermediate;

    while (getline(ss, intermediate, ','))
    {
        days_tokens.push_back(intermediate);
    }

    std::transform(days_tokens.begin(), days_tokens.end(), std::back_inserter(days), [](std::string token)
                   {
                       if (token.find("mon") != std::string::npos)
                       {
                           return Days::Mon;
                       }
                       else if (token.find("tues") != std::string::npos)
                       {
                           return Days::Tues;
                       }
                       else if (token.find("wed") != std::string::npos)
                       {
                           return Days::Wed;
                       }
                       else if (token.find("thur") != std::string::npos)
                       {
                           return Days::Thur;
                       }
                       else if (token.find("fri") != std::string::npos)
                       {
                           return Days::Fri;
                       }
                       else if (token.find("sat") != std::string::npos)
                       {
                           return Days::Sat;
                       }
                       else if (token.find("sun") != std::string::npos)
                       {
                           return Days::Sun;
                       }

                       throw UnknownDayOfTheWeek();
                   });

    return days;
}

bool operator==(const Client &lhs, const Client &rhs)
{
    return lhs.type_ == rhs.type_ && (lhs.days_.size() == rhs.days_.size() &&
                                      std::equal(lhs.days_.begin(), lhs.days_.end(), rhs.days_.begin()));
}