//
// Created by Manuel on 27.01.2021.
//

#include "game_state.h"

#include "../exceptions/LamaException.h"
#include "../serialization/vector_utils.h"


game_state::game_state() {
    this->_players = std::vector<player>();
    this->_is_started = true;
    this->_round_number = 0;
    this->_personal_id = 0;
    this->_middle_cards = std::vector<card>();
    this->showdown = false;
    this->hand_winner = -1;
    this->my_turn = false;
}

game_state::game_state(std::vector<player>& players, bool is_started,
                                int round_number, int personal_id,
                                std::vector<card> middle_cards,
                                bool showdown,
                                int hand_winner,
                                bool my_turn)
           : _players(players),
             _is_started(is_started),
             _round_number(round_number),
             _personal_id(personal_id),
             _middle_cards(middle_cards),
             showdown(showdown),
             hand_winner(hand_winner),
             my_turn(my_turn)
{ }

game_state::~game_state() {
}


// accessors
std::vector<player>& game_state::get_players() {
    return _players;
}
int game_state::get_personal_id() {
    return this->_personal_id;
}

bool game_state::is_started() const {
    return _is_started;
}

int game_state::get_round_number() const {
    return _round_number;
}

int game_state::get_hand_winner() const {
    return hand_winner;
}

bool game_state::is_my_turn() const {
    return my_turn;
}

void game_state::set_my_turn(bool is_it) {
    my_turn = is_it;
}

bool game_state::is_showdown() const {
    return this->showdown;
}

std::vector<std::string> game_state::get_middle_cards() {
    std::vector<std::string> middle_card_assets;
    for (auto it = begin (_middle_cards); it != end (_middle_cards); ++it){
        middle_card_assets.push_back((it)->get_card_path());
    };
    return middle_card_assets;
}

game_state* game_state::from_json(const rapidjson::Value &json) {
    if (json.HasMember("personal_cards")
        && json.HasMember("personal_id")
        && json.HasMember("middle_cards")
        && json.HasMember("player_names")
        && json.HasMember("player_cards")
        && json.HasMember("player_betting_amount")
        && json.HasMember("player_money")
        && json.HasMember("player_has_folded")
        && json.HasMember("player_is_out")
        && json.HasMember("round_number")
           && json.HasMember("is_started"))
    {
        std::vector<card> personal_cards;
        int personal_id;
        std::vector<card> middle_cards;
        std::vector<std::string> player_names;
        std::vector<std::vector<card>> player_cards;
        std::vector<int> player_betting_amount;
        std::vector<int> player_money;
        std::vector<bool> player_has_folded;
        std::vector<bool> player_is_out;
        int round_number;
        bool is_started;
        int hand_winner;

        for (auto &card_ref : json["personal_cards"].GetArray()) {
            personal_cards.push_back(card(
                    card_ref.GetInt()));
        }
        personal_id = json["personal_id"].GetInt();
        for (auto &card_ref : json["middle_cards"].GetArray()) {
            middle_cards.push_back(card(
                    card_ref.GetInt()));
        }
        for (auto &name_ref : json["player_names"].GetArray()) {
            player_names.emplace_back(name_ref.GetString());
        }
        for (auto &cards_ref : json["player_cards"].GetArray()) {
            std::vector<card> tmp;
            if(!cards_ref.IsNull()) {
                for (auto &card_ref : cards_ref.GetArray()) {
                    tmp.push_back(card(card_ref.GetInt()));
                }
            }

            player_cards.emplace_back(tmp);
        }
        for (auto &money : json["player_betting_amount"].GetArray()) {
            player_betting_amount.emplace_back(money.GetInt());
        }
        for (auto &money : json["player_money"].GetArray()) {
            player_money.emplace_back(money.GetInt());
        }
        for (auto &tmp : json["player_has_folded"].GetArray()) {
            player_has_folded.emplace_back(tmp.GetBool());
        }
        for (auto &tmp : json["player_is_out"].GetArray()) {
            player_is_out.emplace_back(tmp.GetBool());
        }
        round_number = json["round_number"].GetInt();
        is_started = json["is_started"].GetBool();
        hand_winner = json["hand_winner"].GetInt();
        bool showdown = json["is_showdown"].GetBool();

        std::vector<player> deserialized_players;

        for(auto i = 0; i < player_names.size(); i++) {
            player p("");
            if(personal_id == i) {
                p = player(
                        std::to_string(i),
                        player_names[i],
                        personal_cards,
                        player_money[i],
                        player_betting_amount[i],
                        player_has_folded[i]
                );
            } else {
                p = player(
                        std::to_string(i),
                        player_names[i],
                        player_cards[i],
                        player_money[i],
                        player_betting_amount[i],
                        player_has_folded[i]
                );
            }

            deserialized_players.push_back(p);
        }

        return new game_state(
                deserialized_players,
                is_started,
                round_number,
                personal_id,
                middle_cards,
                showdown,
                hand_winner,
                false);
    } else {
        throw LamaException("Failed to deserialize game_state_response. Required entries were missing.");
    }
}

