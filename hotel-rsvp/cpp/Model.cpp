#include "Model.h"
#include "Hotel.h"
#include "Client.h"

Response Response::best_price(std::vector<Hotel_h::Hotel> hotels, Client client) {
    Response result = Response();

    for (Hotel hotel: hotels) {
        auto price = hotel.price(client);
        result.update(hotel.name(), hotel.rate(), price);
    }

    if (result.name_.empty()) {
        throw ErrorFindingBestHotel();
    }

    return result;
}