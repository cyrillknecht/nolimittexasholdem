//
// Created by Manuel on 28.01.2021.
//
// client_request is the base-class for all requests that are being sent from the client to the server.
// It offers a function to deserialize a client_request subclass from a valid json.

#ifndef LAMA_CLIENT_REQUEST_H
#define LAMA_CLIENT_REQUEST_H

#include <string>
#include <unordered_map>
#include "../../../../rapidjson/include/rapidjson/document.h"
#include "../../serialization/serializable.h"
#include "../../exceptions/LamaException.h"
#include "../../serialization/uuid_generator.h"
#include "../../serialization/json_utils.h"

// Identifier for the different request types.
// The RequestType is sent with every client_request to identify the type of client_request
// during deserialization on the server side.
enum RequestType {
    set_name,
    is_ready,
    pass,
    fold,
    raise_to,
    heartbeat
};

class client_request : public serializable {
protected:
    RequestType _type;

    explicit client_request(RequestType); // base constructor

    static RequestType extract_type(const rapidjson::Value& json);
private:
    // for serialization
    static const std::unordered_map<RequestType, std::string> _request_type_to_string;

public:
    virtual ~client_request() {}

    [[nodiscard]] RequestType get_type() const { return this->_type; }

    // Serializes the client_request into a json object that can be sent over the network
    void write_into_json(rapidjson::Value& json, rapidjson::Document::AllocatorType& allocator) const override;

    [[nodiscard]] virtual std::string to_string() const;
};


#endif //LAMA_CLIENT_REQUEST_H
