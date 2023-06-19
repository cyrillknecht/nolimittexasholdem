//
// Created by Manuel on 29.01.2021.
//

#ifndef LAMA_PASS_REQUEST_H
#define LAMA_PASS_REQUEST_H

#include <string>
#include "client_request.h"
#include "../../../../rapidjson/include/rapidjson/document.h"

class pass_request : public client_request{
public:
    pass_request();
    virtual void write_into_json(rapidjson::Value& json, rapidjson::Document::AllocatorType& allocator) const override;
};


#endif //LAMA_FOLD_REQUEST_H
