//
// Created by Manuel on 29.01.2021.
//

#include "fold_request.h"

fold_request::fold_request() :
        client_request(RequestType::fold)
{ }

void fold_request::write_into_json(rapidjson::Value &json,
                                   rapidjson::MemoryPoolAllocator<rapidjson::CrtAllocator> &allocator) const {
    client_request::write_into_json(json, allocator);
    json.AddMember("action", "fold", allocator);
}