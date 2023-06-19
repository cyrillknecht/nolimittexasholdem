//
// Created by Manuel on 09.03.2021.
//

#ifndef POKER_POKEREXCEPTION_H
#define POKER_POKEREXCEPTION_H

#include <string>
#include <utility>

class LamaException : public std::exception {
private:
    std::string _msg;
public:
    explicit LamaException(std::string  message) : _msg(std::move(message)) { };

    [[nodiscard]] const char* what() const noexcept override {
        return _msg.c_str();
    }
};

#endif //POKER_POKEREXCEPTION_H
