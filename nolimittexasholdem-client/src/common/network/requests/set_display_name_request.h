//
// Created by Manuel on 29.01.2021.
//

#ifndef START_GAME_REQUEST_H
#define START_GAME_REQUEST_H


#include <string>
#include "client_request.h"
#include "../../../../rapidjson/include/rapidjson/document.h"

class set_display_name_request : public client_request{

private:
    std::string _player_name;
public:
    [[nodiscard]] std::string get_player_name() const { return this->_player_name; }
    /*
     * Constructor to join any game
     */
    set_display_name_request(std::string name);

    virtual void write_into_json(rapidjson::Value& json, rapidjson::Document::AllocatorType& allocator) const override;
};


#endif //LAMA_SET_NAME_REQUEST_H
