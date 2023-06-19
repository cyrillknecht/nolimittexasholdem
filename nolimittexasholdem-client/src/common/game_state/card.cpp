#include "card.h"
#include "../exceptions/LamaException.h"

card::card(int value)
    : _value(value)
{ }


card::card() : unique_serializable() {
    this->_value = 0;
}

// accessors
int card::get_suite() const {
    return _value / 13;
}

int card::get_rank() const {
    return _value % 13 + 2;
}

std::string card::rank_to_string() const {
    switch (this->get_rank()) {
        case 11: return "jack";
        case 12: return "queen";
        case 13: return "king";
        case 14: return "ace";
    }
    return std::to_string(card::get_rank());
}

std::string card::get_card_path() const {
    switch(this->get_suite()){
        case 0:
            //clubs
            return "assets/" + rank_to_string() + "_of_clubs.png";
        case 1:
            //diamonds
            return "assets/" + rank_to_string() + "_of_diamonds.png";
        case 2:
            //hearts
            return "assets/" + rank_to_string() + "_of_hearts.png";
        case 3:
            //spades
            return "assets/" + rank_to_string() + "_of_spades.png";
    }
    throw "END OF SWITCHEROO STATMENETINO";
}












