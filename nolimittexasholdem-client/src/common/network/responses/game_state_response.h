//
// Created by Manuel on 15.02.2021.
//

#ifndef LAMA_GAME_STATE_RESPONSE_H
#define LAMA_GAME_STATE_RESPONSE_H

#include "server_response.h"
#include "../../game_state/game_state.h"

class game_state_response : public server_response {
private:
    rapidjson::Value* _state_json;

    /*
     * Private constructor for deserialization
     */
    game_state_response(rapidjson::Value* state_json);
public:
    ~game_state_response();

    rapidjson::Value* get_state_json() const;

    static game_state_response* from_json(const rapidjson::Value& json);

    virtual void Process() const;
};


#endif //LAMA_GAME_STATE_RESPONSE_H
