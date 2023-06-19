//
// Created by Manuel on 25.01.2021.
//

#ifndef LAMA_PLAYER_H
#define LAMA_PLAYER_H

#include <vector>
#include <string>
#include "card.h"
#include "../serialization/uuid_generator.h"
#include "../../../rapidjson/include/rapidjson/document.h"
#include "../serialization/unique_serializable.h"
#include "../serialization/serializable_value.h"
#include "../serialization/vector_utils.h"

class player {
private:
    std::string _player_id;
    std::string _player_name;
    std::vector<card> _player_cards;
    int _balance;
    int _current_bet;
    bool _has_folded;
public:
    // constructors
    explicit player(std::string name);   // for client
    ~player();

    // accessors
    std::string get_id() const;
    std::string get_player_name() const;
    std::vector<std::string> get_cards() const;
    int get_balance() const;
    int get_current_bet() const;
    bool has_folded() const;

player(std::string player_id,
       std::string player_name,
       std::vector<card> player_cards,
       int balance,
       int current_bet,
       bool has_folded);
};


#endif //LAMA_PLAYER_H
