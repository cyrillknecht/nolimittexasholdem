#ifndef LAMA_CARD_H
#define LAMA_CARD_H

#include <vector>
#include <algorithm>
#include <string>
#include <utility>
#include <functional>
#include "../serialization/vector_utils.h"
#include "../serialization/unique_serializable.h"
#include "../serialization/serializable_value.h"


class card : public unique_serializable{
private:
    int _value;

    /*
     * Deserialization constructor
     */

public:
    explicit card(int value);
    card();

// accessors
    int get_suite() const;
    int get_rank() const;
    std::string rank_to_string() const;
    std::string get_card_path() const;
};

#endif //LAMA_CARD_H