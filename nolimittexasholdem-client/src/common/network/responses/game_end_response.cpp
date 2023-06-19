//
// Created by Manuel on 15.02.2021.
//

#include "game_end_response.h"
#include "../../serialization/json_utils.h"
#include "../../exceptions/LamaException.h"
#include "../../game_state/game_state.h"

#ifdef LAMA_CLIENT
#include "../../../client/GameController.h"
#endif

game_end_response::game_end_response(int winner) :
    server_response(ResponseType::game_end), winner(winner) {
}
void game_end_response::Process() const {
    GameController::game_ended = true;
    GameController::showGameOverMessage(winner);
}

game_end_response *game_end_response::from_json(const rapidjson::Value& json) {
    if(json.HasMember("winner")) {
        if(json["winner"].IsNull()) {
            return new game_end_response(-1);
        }
        return new game_end_response(json["winner"].GetInt());
    }

    throw LamaException("No winner");
}
