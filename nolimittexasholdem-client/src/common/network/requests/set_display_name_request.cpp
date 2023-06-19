//
// Created by Manuel on 29.01.2021.
//

#include "set_display_name_request.h"

#include <utility>

#ifdef LAMA_SERVER
#include <string>
#include "../../../server/game_instance_manager.h"
#include "../../../server/player_manager.h"
#include "../../../server/game_instance.h"
#endif

// Public constructor
set_display_name_request::set_display_name_request(std::string name)
        : client_request( RequestType::set_name ),
          _player_name(name)
{ }

void set_display_name_request::write_into_json(rapidjson::Value &json,
                                               rapidjson::MemoryPoolAllocator<rapidjson::CrtAllocator> &allocator) const {
    client_request::write_into_json(json, allocator);
    rapidjson::Value name_val(_player_name.c_str(), allocator);
    json.AddMember("player_name", name_val, allocator);
}


