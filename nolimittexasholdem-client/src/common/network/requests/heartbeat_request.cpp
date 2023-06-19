//
// Created by Manuel on 29.01.2021.
//

#include "heartbeat_request.h"

#ifdef LAMA_SERVER
#include "../../../server/game_instance_manager.h"
#include "../../../server/game_instance.h"
#endif

// Public constructor
heartbeat_request::heartbeat_request()
        : client_request( RequestType::heartbeat )
{ }

void heartbeat_request::write_into_json(rapidjson::Value &json,
                                         rapidjson::MemoryPoolAllocator<rapidjson::CrtAllocator> &allocator) const {
    client_request::write_into_json(json, allocator);
}
