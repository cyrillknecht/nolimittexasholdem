//
// Created by Manuel on 28.01.2021.
//

#ifndef LAMA_RAISE_REQUEST_H
#define LAMA_RAISE_REQUEST_H


#include "client_request.h"
#include <string>
#include "../../../../rapidjson/include/rapidjson/document.h"

class raise_request : public client_request {

private:
    unsigned int amount;
public:
    //Private constructor for deserialization
    raise_request(unsigned int amount);

    [[nodiscard]] std::string get_amount() const { return std::to_string(this->amount); }

    virtual void write_into_json(rapidjson::Value& json, rapidjson::Document::AllocatorType& allocator) const override;
};


#endif //LAMA_RAISE_REQUEST_H
