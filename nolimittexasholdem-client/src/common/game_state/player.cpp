//
// Created by Manuel on 25.01.2021.
//

#include "player.h"

#include "../exceptions/LamaException.h"
// Decerialization constructor
player::player(std::string player_id, std::string player_name,
               std::vector<card> player_cards, int balance, int current_bet,
               bool has_folded):
        _player_id(player_id),
        _player_name(player_name),
        _player_cards(player_cards),
        _balance(balance),
        _current_bet(current_bet),
        _has_folded(has_folded)
{ }

// Other constructors
player::player(std::string name) {
    this->_player_id = "";
    this->_player_name = name;
    this->_player_cards = std::vector<card>();
    this->_balance = 0;
    this->_current_bet = 0;
    this->_has_folded = false;
}

player::~player() {
}

// Accessors
std::string player::get_id() const {
    return this->_player_id;
}

std::string player::get_player_name() const {
    return this->_player_name;
}

std::vector<std::string> player::get_cards() const {
    std::vector<std::string> cards;
    for (auto it = begin (_player_cards); it != end (_player_cards); ++it){
        cards.push_back((it)->get_card_path());
    };
    return cards;
}

int player::get_balance() const {
    return this->_balance;
}

int player::get_current_bet() const {
    return this->_current_bet;
}

bool player::has_folded() const {
    return this->_has_folded;
}
