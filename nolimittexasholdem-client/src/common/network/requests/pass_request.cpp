//
// Created by Manuel on 29.01.2021.
//

#include "pass_request.h"

pass_request::pass_request() :
        client_request(RequestType::pass)
{ }

void pass_request::write_into_json(rapidjson::Value &json,
                                   rapidjson::MemoryPoolAllocator<rapidjson::CrtAllocator> &allocator) const {
    client_request::write_into_json(json, allocator);
    json.AddMember("action", "pass", allocator);
}