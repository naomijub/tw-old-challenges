#ifndef Client_h
#define Client_h

#include <vector>

enum ClientType {
    Regular,
    Rewards,
};

enum Days {
    Mon = 0,
    Tues = 1,
    Wed = 2,
    Thur = 3,
    Fri = 4,
    Sat = 5,
    Sun = 6,
};

struct Client {
    std::vector<Days> days_;
    ClientType type_;
    
    public: 
        Client(ClientType client_type, std::vector<Days> days) : days_(days), type_(client_type) {};

};

#endif