#include "GameController.h"
//#include "../common/network/requests/connection_ended_request.h"
#include "../common/network/requests/fold_request.h"
#include "../common/network/requests/is_ready_request.h"
#include "../common/network/requests/heartbeat_request.h"
#include "../common/network/requests/pass_request.h"
#include "../common/network/requests/raise_request.h"
#include "../common/network/requests/set_display_name_request.h"
#include "network/ClientNetworkManager.h"



// Initialize static member variables
GameWindow* GameController::_gameWindow = nullptr;
ConnectionPanel* GameController::_connectionPanel = nullptr;
MainGamePanel* GameController::_mainGamePanel = nullptr;
volatile bool GameController::game_ended = false;

game_state* GameController::_currentGameState = nullptr;


player* GameController::_me() {
    if (_currentGameState == nullptr) {
        return nullptr;
    }
    return &_currentGameState->get_players()[_currentGameState->get_personal_id()];
}

// Game UI Control
void GameController::init(GameWindow* gameWindow) {

    GameController::_gameWindow = gameWindow;
    gameWindow->SetBackgroundColour(wxColour(0, 0, 0));

    // Set up main panels
    GameController::_connectionPanel = new ConnectionPanel(gameWindow);
    GameController::_mainGamePanel = new MainGamePanel(gameWindow);

    // Hide all panels
    GameController::_connectionPanel->Show(false);
    GameController::_mainGamePanel->Show(false);

    // Only show connection panel at the start of the game
    GameController::_gameWindow->showPanel(GameController::_connectionPanel);

    // Set status bar
    GameController::showStatus("Not yet connected to a Poker game server");
}

void GameController::updateGameState(game_state* newGameState) {
    // the existing game state is now old
    game_state* oldGameState = GameController::_currentGameState;

    // save the new game state as our current game state
    GameController::_currentGameState = newGameState;

    if(oldGameState != nullptr) {

        // check if a new round started, and display message accordingly
        if(oldGameState->get_round_number() > 0 && oldGameState->get_round_number() < newGameState->get_round_number()) {
            //GameController::showNewRoundMessage(oldGameState, newGameState);
        }

        // delete the old game state, we don't need it anymore
        if(oldGameState != _currentGameState) {
            delete oldGameState;
        }
    }

    // make sure we are showing the main game panel in the window (if we are already showing it, nothing will happen)
    GameController::_gameWindow->showPanel(GameController::_mainGamePanel);

    // command the main game panel to rebuild itself, based on the new game state
    GameController::_mainGamePanel->buildGameState(GameController::_currentGameState);
}

// Game setup
// Only used by ConnectionPanel !!!
void GameController::connectToServer() {
    // get values form UI input fields
    wxString inputServerAddress = GameController::_connectionPanel->getServerAddress().Trim();
    wxString inputServerPort = GameController::_connectionPanel->getServerPort().Trim();
    wxString inputPlayerName = GameController::_connectionPanel->getPlayerName().Trim();

    // check that all values were provided
    if(inputServerAddress.IsEmpty()) {
        GameController::showError("Input error", "Please provide the server's address");
        return;
    }
    if(inputServerPort.IsEmpty()) {
        GameController::showError("Input error", "Please provide the server's port number");
        return;
    }
    if(inputPlayerName.IsEmpty()) {
        GameController::showError("Input error", "Please enter your desired player name");
        return;
    }

    // convert host from wxString to std::string
    std::string host = inputServerAddress.ToStdString();

    // convert port from wxString to uint16_t
    unsigned long portAsLong;
    if(!inputServerPort.ToULong(&portAsLong) || portAsLong > 65535) {
        GameController::showError("Connection error", "Invalid port");
        return;
    }
    auto port = (uint16_t) portAsLong;

    // convert player name from wxString to std::string
    std::string playerName = inputPlayerName.ToStdString();

    // connect to network
    ClientNetworkManager::init(host, port);

    // send request to join game
    set_display_name_request request = set_display_name_request(playerName);
    ClientNetworkManager::sendRequest(request);
}

void GameController::startGame() {
    is_ready_request request = is_ready_request();
    ClientNetworkManager::sendRequest(request);
}

// Player actions
void GameController::fold() {
    fold_request request = fold_request();
    ClientNetworkManager::sendRequest(request);
}

void GameController::check() {
    pass_request request = pass_request();
    ClientNetworkManager::sendRequest(request);
}

void GameController::bet() {
    int amount = GameController::enterBet();
    if (amount == 0) {
        return;
    }
    raise_request request = raise_request(amount);
    ClientNetworkManager::sendRequest(request);
}

void GameController::call() {
    int highest = 0;
    for(auto p: GameController::_currentGameState->get_players()) {
        highest = std::max(highest, p.get_current_bet());
    }
    raise_request request = raise_request(highest);
    ClientNetworkManager::sendRequest(request);
}

void GameController::raise() {
    //no difference to bet, so call bet
    GameController::bet();
}

int GameController::enterBet()
{
    std::string title = "Enter your bet:";
    std::string message = "Enter the TOTAL amount you want to bet in this game."
                          "\nMust be higher than current amount";

    wxString betString;
    wxTextEntryDialog dialog(nullptr, message, title, wxEmptyString, wxOK | wxCANCEL);
    if (dialog.ShowModal() == wxID_OK)
    {
        betString = dialog.GetValue();
        // Convert the betString to an integer or perform further processing
        long bet = 0;

        if (betString.ToLong(&bet))
        {
            // Return the entered bet value
            return (int)bet;
        }
    }

    // Return a default value or handle invalid input as needed
    return 0;
}

// Messages
void GameController::showError(const std::string& title, const std::string& message) {
    wxMessageBox(message, title, wxICON_ERROR);
}

void GameController::showGameOverMessage(int winner) {
    std::string title = "Game Over!";
    std::string buttonLabel = "Close Game";
    std::string message = "Winner is: ";
    if(winner == -1) {
        message += "Nobody :(";
    } else {
        message += GameController::_currentGameState->get_players()[winner].get_player_name();
    }
    wxMessageDialog dialogBox = wxMessageDialog(nullptr, message, title, wxICON_NONE);
    dialogBox.SetOKLabel(wxMessageDialog::ButtonLabel(buttonLabel));
    int buttonClicked = dialogBox.ShowModal();
    if(buttonClicked == wxID_OK) {
        GameController::_gameWindow->Close();
    }
}

void GameController::showNewRoundMessage(int winner) {
    std::string title = "Round Over!";
    std::string buttonLabel = "Next round";
    std::string message = "Winner of the round: ";

    if(winner == -1) {
        message += "Nobody :(";
    } else {
        message += GameController::_currentGameState->get_players()[winner].get_player_name();
    }

    wxMessageDialog dialogBox = wxMessageDialog(nullptr, message, title, wxICON_NONE);
    dialogBox.SetOKLabel(wxMessageDialog::ButtonLabel(buttonLabel));
    dialogBox.ShowModal();
}

void GameController::showStatus(const std::string& message) {
    GameController::_gameWindow->setStatus(message);
}

wxEvtHandler* GameController::getMainThreadEventHandler() {
    return GameController::_gameWindow->GetEventHandler();
}
