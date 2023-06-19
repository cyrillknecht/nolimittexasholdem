//
// Created by Manuel on 28.01.2021.
//

#include "raise_request.h"

// Public constructor
raise_request::raise_request(unsigned int amount)
        : client_request(RequestType::raise_to),
          amount(amount)
{ }

void raise_request::write_into_json(rapidjson::Value &json,
                                        rapidjson::MemoryPoolAllocator<rapidjson::CrtAllocator> &allocator) const {
    client_request::write_into_json(json, allocator);
    json.AddMember("action", "raise_to",allocator);
    json.AddMember("amount", this->amount,allocator);
}
