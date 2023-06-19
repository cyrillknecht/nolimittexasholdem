//
// Created by Manuel on 29.01.2021.
//

#include "is_ready_request.h"

#ifdef LAMA_SERVER
#include "../../../server/game_instance_manager.h"
#include "../../../server/game_instance.h"
#endif

// Public constructor
is_ready_request::is_ready_request()
        : client_request( RequestType::is_ready )
{ }

void is_ready_request::write_into_json(rapidjson::Value &json,
                                       rapidjson::MemoryPoolAllocator<rapidjson::CrtAllocator> &allocator) const {
    client_request::write_into_json(json, allocator);
}
