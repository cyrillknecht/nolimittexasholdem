//
// Created by Manuel on 15.02.2021.
//

#include "server_response.h"
#include "awaiting_player_response.h"
#include "game_state_response.h"

#include "../../exceptions/LamaException.h"
#include "game_end_response.h"
#include "../../rapidjson/include/rapidjson/writer.h"
#include "../../rapidjson/include/rapidjson/stringbuffer.h"

// for deserialization
const std::unordered_map<std::string, ResponseType> server_response::_string_to_response_type = {
        {"awaiting_player", ResponseType::awaiting_player },
        {"game_end", ResponseType::game_end},
        {"game_state", ResponseType::game_state_response_enum}
};

server_response::server_response(ResponseType params):
    _type(params)
{ }

ResponseType server_response::get_type() const {
    return this->_type;
}

ResponseType server_response::extract_response_type(const rapidjson::Value& json) {
    if (json.HasMember("type")) {
        return server_response::_string_to_response_type.at(json["type"].GetString());
    }
    else
    {
        throw LamaException("Server Response did not contain game_id");
    }
}

server_response *server_response::from_json(const rapidjson::Value& json) {
    auto type = server_response::extract_response_type(json);

    if (type == ResponseType::awaiting_player) {
        return awaiting_player_response::from_json(json);
    } else if (type == ResponseType::game_end) {
        return game_end_response::from_json(json);
    } else if (type == ResponseType::game_state_response_enum) {
        return game_state_response::from_json(json);
    } else {
        throw LamaException("Encountered unknown ServerResponse type");
    }
}

