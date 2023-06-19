//
// Created by Manuel on 28.01.2021.
//

#include "client_request.h"

#include <iostream>

// for serialization
const std::unordered_map<RequestType, std::string> client_request::_request_type_to_string = {
        { RequestType::set_name, "set_display_name" },
        { RequestType::is_ready, "is_ready"},
        { RequestType::pass, "response"},
        { RequestType::raise_to, "response"},
        {RequestType::fold, "response"},
        {RequestType::heartbeat, "heartbeat"}
};

// protected constructor. only used by subclasses
client_request::client_request(RequestType _type) :
        _type(_type)
{ }

void client_request::write_into_json(rapidjson::Value &json,
                                     rapidjson::MemoryPoolAllocator<rapidjson::CrtAllocator> &allocator) const {
    rapidjson::Value type_val(_request_type_to_string.at(this->_type).c_str(), allocator);
    json.AddMember("type", type_val, allocator);
}

std::string client_request::to_string() const {
    return "client_request of type " + client_request::_request_type_to_string.at(_type);
}






