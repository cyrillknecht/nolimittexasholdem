//
// Created by Manuel on 27.01.2021.
//

#ifndef LAMA_GAME_STATE_H
#define LAMA_GAME_STATE_H

#include <vector>
#include <string>
#include "../../rapidjson/include/rapidjson/document.h"
#include "../serialization/serializable.h"
#include "../serialization/serializable_value.h"
#include "../serialization/unique_serializable.h"
#include "player.h"

class game_state {
private:
    std::vector<player> _players;
    bool _is_started;
    int _round_number;
    int _personal_id;
    std::vector<card> _middle_cards;
    bool showdown;
    int hand_winner;
    bool my_turn;
    // deserialization constructor
    game_state(
            std::vector<player>& players,
            bool is_started,
            int round_number,
            int personal_id,
            std::vector<card> middle_cards,
            bool showdown,
            int hand_winner,
            bool my_turn);
public:
    game_state();
    ~game_state();

// accessors
    std::vector<player>& get_players();
    int get_personal_id();
    bool is_started() const;
    std::vector<std::string> get_middle_cards();
    int get_round_number() const;
    bool is_showdown() const;
    int get_hand_winner() const;
    bool is_my_turn() const;
    void set_my_turn(bool is_it);
    // serializable interface
    static game_state* from_json(const rapidjson::Value& json);
};

#endif //LAMA_GAME_STATE_H
