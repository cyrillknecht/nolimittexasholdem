//
// Created by Manuel on 15.02.2021.
//

#include "game_state_response.h"

#include "../../exceptions/LamaException.h"
#include "../../serialization/json_utils.h"

#ifdef LAMA_CLIENT
#include "../../../client/GameController.h"
#endif

game_state_response::game_state_response(rapidjson::Value* state_json) :
        server_response(ResponseType::game_state_response_enum),
        _state_json(state_json)
{ }

game_state_response *game_state_response::from_json(const rapidjson::Value& json) {
    if (server_response::extract_response_type(json) == ResponseType::game_state_response_enum) {
        return new game_state_response(json_utils::clone_value(json));
    } else {
        throw LamaException("Could not parse full_state_response from json. state is missing.");
    }
}

game_state_response::~game_state_response() {
    if (_state_json != nullptr) {
        delete _state_json;
        _state_json = nullptr;
    }
}

rapidjson::Value* game_state_response::get_state_json() const {
    return _state_json;
}

void game_state_response::Process() const {
    try {
        std::cout << "oompa game_state_response" << std::endl;
        class game_state* state = game_state::from_json(*_state_json);
        GameController::updateGameState(state);
    } catch(std::exception& e) {
        std::cerr << "Failed to extract game_state_response from full_state_response" << std::endl
                  << e.what() << std::endl;
    }
}
