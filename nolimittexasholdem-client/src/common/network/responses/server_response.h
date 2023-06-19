//
// Created by Manuel on 15.02.2021.
//
// Base class for all messages sent from the server to the client.
// It offers a function to deserialize a server_response subclass from a valid json.

#ifndef LAMA_SERVER_RESPONSE_H
#define LAMA_SERVER_RESPONSE_H

#include <string>
#include <unordered_map>

#include "../../serialization/serializable.h"

// Identifier for the different response types.
// The ResponseType is sent with every server_response to identify the type of server_response
// during deserialization on the client side.
enum ResponseType {
    awaiting_player,
    game_state_response_enum,
    game_end
};

class server_response {
private:

    // for deserialization
    static const std::unordered_map<std::string, ResponseType> _string_to_response_type;

protected:
    ResponseType _type;

    explicit server_response(ResponseType); // base constructor

    static ResponseType extract_response_type(const rapidjson::Value& json);

public:
    ResponseType get_type() const;

    // Tries to create the specific server_response from the provided json.
    // Throws exception if parsing fails -> Use only inside "try{ }catch()" block
    static server_response* from_json(const rapidjson::Value& json);

    virtual void Process() const = 0;
};


#endif //LAMA_SERVER_RESPONSE_H
