//
// Created by Manuel on 29.01.2021.
//

#ifndef LAMA_IS_READY_REQUEST_H
#define LAMA_IS_READY_REQUEST_H


#include <string>
#include "client_request.h"
#include "../../../../rapidjson/include/rapidjson/document.h"

class is_ready_request : public client_request{
public:
    is_ready_request();
    virtual void write_into_json(rapidjson::Value& json, rapidjson::Document::AllocatorType& allocator) const override;
};

#endif //LAMA_IS_READY_REQUEST_H