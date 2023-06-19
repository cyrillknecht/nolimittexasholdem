//
// Created by Manuel on 15.02.2021.
//

#include "awaiting_player_response.h"
#include "../../serialization/json_utils.h"
#include "../../exceptions/LamaException.h"
#include "../../game_state/game_state.h"

#ifdef LAMA_CLIENT
#include "../../../client/GameController.h"
#endif

awaiting_player_response::awaiting_player_response() :
    server_response(ResponseType::awaiting_player) {
}

void awaiting_player_response::Process() const  {
    //Simulate new game_state
    class game_state* state = GameController::_currentGameState;

    state->set_my_turn(true);

    GameController::updateGameState(state);
}

awaiting_player_response *awaiting_player_response::from_json(const rapidjson::Value& json) {
    return new awaiting_player_response();
}

