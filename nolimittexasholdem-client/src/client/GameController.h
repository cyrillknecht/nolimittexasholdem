#ifndef TEXAS_GAMECONTROLLER_H
#define TEXAS_GAMECONTROLLER_H

#include "windows/GameWindow.h"
#include "panels/ConnectionPanel.h"
#include "panels/MainGamePanel.h"
#include "network/ResponseListenerThread.h"
#include "../common/game_state/game_state.h"


class GameController {

public:
    // Game UI Init -> Used to start the client in poker.cpp
    static void init(GameWindow* gameWindow);

    // Game UI update -> Used by the request_response
    // and the full_state_response to trigger UI update
    static void updateGameState(game_state* newGameState);

    // Game setup -> Functions triggerd by the UI to request change at the server
    static void connectToServer();
    static void startGame();

    // Player actions -> Functions triggerd by the UI to request change at the server
    static void fold();
    static void check();
    static void bet();
    static void call();
    static void raise();
    static int enterBet();

    // Messages -> Trigger by the UI or the Client Network Controller
    // to show messages to the user.
    static void showError(const std::string& title, const std::string& message);
    static void showGameOverMessage(int winner);
    static void showNewRoundMessage(int winner);
    static void showStatus(const std::string& message);

    // Used in the Response Listener Thread
    static wxEvtHandler* getMainThreadEventHandler();
    static game_state* _currentGameState;

    static player* _me();
    static volatile bool game_ended;
private:
    static GameWindow* _gameWindow;
    static ConnectionPanel* _connectionPanel;
    static MainGamePanel* _mainGamePanel;
};


#endif //TEXAS_GAMECONTROLLER_H
