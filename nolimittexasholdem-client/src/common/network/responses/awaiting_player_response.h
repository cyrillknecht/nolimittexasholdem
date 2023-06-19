//
// Created by Manuel on 15.02.2021.
//

#ifndef LAMA_AWAITING_PLAYER_RESPONSE_H
#define LAMA_AWAITING_PLAYER_RESPONSE_H

#include <string>
#include "server_response.h"

class awaiting_player_response : public server_response {
public:

    awaiting_player_response();

    static awaiting_player_response* from_json(const rapidjson::Value& json);

    virtual void Process() const;
};


#endif //LAMA_AWAITING_PLAYER_RESPONSE_H
