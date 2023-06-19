//
// Created by Manuel on 15.02.2021.
//

#ifndef GAME_END_PLAYER_RESPONSE_H
#define GAME_END_PLAYER_RESPONSE_H

#include <string>
#include "server_response.h"

class game_end_response : public server_response {
public:
    int winner;
    game_end_response(int winner);

    static game_end_response* from_json(const rapidjson::Value& json);
    virtual void Process() const;
};


#endif //GAME_END_PLAYER_RESPONSE_H
