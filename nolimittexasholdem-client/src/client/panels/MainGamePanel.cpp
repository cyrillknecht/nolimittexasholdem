#include "MainGamePanel.h"
#include "../GameController.h"
#include <map>


MainGamePanel::MainGamePanel(wxWindow* parent) : wxPanel(parent, wxID_ANY, wxDefaultPosition, wxSize(960, 680)) {

}


// Building current Display
void MainGamePanel::buildGameState(game_state* gameState) {
    const player* me = &gameState->get_players()[gameState->get_personal_id()];

    // remove any existing UI
    this->DestroyChildren();

    // Set the background image to poker_table.png
    new ImagePanel(this, "assets/poker_table.png", wxBITMAP_TYPE_PNG, wxPoint(0, 0), wxSize(960, 680));

    std::vector<player> players = gameState->get_players();
    int numberOfPlayers = players.size();

    double anglePerPlayer = MainGamePanel::twoPi / (double) numberOfPlayers;

    // show all other players
    for(int i = 1; i < numberOfPlayers; i++) {

        // get player at i-th position after myself
        player* otherPlayer = &players.at((gameState->get_personal_id() + i) % numberOfPlayers);

        double playerAngle = (double) i * anglePerPlayer;
        int side = (2 * i) - numberOfPlayers; // side < 0 => right, side == 0 => center, side > 0 => left

        this->buildOtherPlayerHand(gameState, otherPlayer, playerAngle);
        this->buildOtherPlayerLabels(gameState, otherPlayer, playerAngle, side);
    }

    // show all board cards that have already been revealed
    this->buildCenterPile(gameState);

    // show turn indicator on the board
    this->buildTurnIndicator(gameState, me);

    // show our own player
    this->buildThisPlayer(gameState, me);

    // update layout
    this->Layout();
    if (gameState->is_showdown()) {
        GameController::showNewRoundMessage(gameState->get_hand_winner());
    }
}


// Build parts of the UI
void MainGamePanel::buildOtherPlayerHand(game_state* gameState, player* otherPlayer, double playerAngle) {

    //No need to build hands if game has not started yet
    if(!gameState->is_started()) {
        return;
    }

    // Define the ellipse which represents the virtual player circle
    double horizontalRadius = MainGamePanel::otherPlayerHandDistanceFromCenter * 1.4; // 1.4 to horizontally elongate players' circle
    double verticalRadius = MainGamePanel::otherPlayerHandDistanceFromCenter;

    //Get this player's position on that ellipse
    wxPoint handPosition = MainGamePanel::tableCenter;
    handPosition += MainGamePanel::getPointOnEllipse(horizontalRadius, verticalRadius, playerAngle);

    //Get new bounds of image, as they increase when image is rotated
    wxSize boundsOfRotatedHand = this->getBoundsOfRotatedSquare(playerAngle);
    handPosition -= boundsOfRotatedHand / 2;

    if(!gameState->is_showdown() || otherPlayer->get_cards().size() < 2) {
        std::string handImage = "assets/lama_hand_2.png";

        new ImagePanel(this, handImage, wxBITMAP_TYPE_ANY, handPosition, boundsOfRotatedHand, playerAngle);
    } else {
        std::string firstCardFile = otherPlayer->get_cards()[0];
        std::string secondCardFile = otherPlayer->get_cards()[1];

         wxPoint firstCardPosition = handPosition + wxSize(MainGamePanel::cardSize.GetWidth()*0.75/2+30,0);
         wxPoint secondCardPosition = handPosition - wxSize(MainGamePanel::cardSize.GetWidth()*0.75/2-26,0);

        new ImagePanel(this, firstCardFile, wxBITMAP_TYPE_ANY, firstCardPosition, MainGamePanel::cardSize*0.75, 0);
        new ImagePanel(this, secondCardFile, wxBITMAP_TYPE_ANY, secondCardPosition, MainGamePanel::cardSize*0.75, 0);
    }
}

void MainGamePanel::buildOtherPlayerLabels(game_state* gameState, player* otherPlayer, double playerAngle, int side) {

    long textAlignment = wxALIGN_CENTER;
    int labelOffsetX = 0;

    if(side < 0) { // right side
        textAlignment = wxALIGN_LEFT;
        labelOffsetX = 85;

    } else if(side > 0) { // left side
        textAlignment = wxALIGN_RIGHT;
        labelOffsetX = -85;
    }

    // Define the ellipse which represents the virtual player circle
    double horizontalRadius = MainGamePanel::otherPlayerLabelDistanceFromCenter * 1.4; // 1.25 to horizontally elongate players' circle
    double verticalRadius = MainGamePanel::otherPlayerLabelDistanceFromCenter;

    // Get this player's position on that ellipse
    wxPoint labelPosition = MainGamePanel::tableCenter;
    labelPosition += MainGamePanel::getPointOnEllipse(horizontalRadius, verticalRadius, playerAngle);
    labelPosition += wxSize(labelOffsetX, 0);

    // If game has not yet started, we only have two lines
    if(!gameState->is_started()) {
        this->buildStaticText(
                otherPlayer->get_player_name(),
                labelPosition + wxSize(-100, -18),
                wxSize(200, 18),
                textAlignment,
                true
        );
        this->buildStaticText(
                "Ready to play...",
                labelPosition + wxSize(-100, 0),
                wxSize(200, 18),
                textAlignment
        );
        return;
    }

    this->buildStaticText(
            otherPlayer->get_player_name(),
            labelPosition + wxSize(-100, -27),
            wxSize(200, 18),
            textAlignment,
            true
    );
    this->buildStaticText(
            "Remaining Credits: " + std::to_string(otherPlayer->get_balance()),
            labelPosition + wxSize(-100, -9),
            wxSize(200, 18),
            textAlignment
    );

    // Show other player's status label
    std::string statusText = "Current Bet: " + std::to_string(otherPlayer->get_current_bet());
    bool bold = false;
    if(otherPlayer->has_folded()) {
        statusText = "Has folded.";
    } else if(!gameState->is_my_turn()) {
        statusText = "Still playing...";
        bold = true;
    }
    this->buildStaticText(
            statusText,
            labelPosition + wxSize(-100, 9),
            wxSize(200, 18),
            textAlignment,
            bold
    );

}

void MainGamePanel::buildCenterPile(game_state* gameState) {

    if(!gameState->is_started()){
        // if the game did not start yet, show a card in the center (only for the mood)
        wxPoint cardPosition = MainGamePanel::tableCenter - (MainGamePanel::cardSize / 2);
        new ImagePanel(this, "assets/ace_of_spades.png", wxBITMAP_TYPE_ANY, cardPosition, MainGamePanel::cardSize);
        return;
    }

    // Make vector containing all current middle cards in form of asset paths
    std::vector<std::string> placeholderCardImages = gameState->get_middle_cards();


    // Get the number of board cards
    int numBoardCards = placeholderCardImages.size();

    // Create a horizontal sizer for the board card panels
    auto* boardCardsSizer = new wxBoxSizer(wxHORIZONTAL);

    // Calculate the position of the board cards
    wxPoint boardCardsPosition = MainGamePanel::tableCenter - wxPoint(MainGamePanel::cardSize.GetWidth()*numBoardCards/2+(numBoardCards-1)*MainGamePanel::borderWidth/2,
                                                                      MainGamePanel::cardSize.GetHeight()/2);

    // Create image panels and add them to the sizer, with a border between them
    for (int i = 0; i < numBoardCards; i++) {
        auto* cardPanel = new ImagePanel(this, placeholderCardImages[i], wxBITMAP_TYPE_ANY, boardCardsPosition, MainGamePanel::cardSize);
        cardPanel->SetToolTip("Board Cards: In total, 5 cards will be dealt on the board. These cards are visible to all players and can be used by all players to make their hand.");
        boardCardsSizer->Add(cardPanel, 0, wxLEFT | wxRIGHT, 8);
        boardCardsPosition += wxPoint(MainGamePanel::cardSize.GetWidth()+MainGamePanel::borderWidth, 0); // There could be a more elegant solution
    }

// Add the sizer to the parent panel
    this->SetSizer(boardCardsSizer);

}

void MainGamePanel::buildTurnIndicator(game_state *gameState, const player *me) {
    // If game yet started or no current player, do not show turn indicator
    if (!gameState->is_started()) return;

    std::string turnIndicatorText;
    if(gameState->is_my_turn()) {
        turnIndicatorText = "It's your turn! Choose an action.";
    } else {
        turnIndicatorText = "Waiting for your turn";
    }

    wxPoint turnIndicatorPosition = MainGamePanel::tableCenter + MainGamePanel::turnIndicatorOffset;

    this->buildStaticText(
            turnIndicatorText,
            turnIndicatorPosition,
            wxSize(200, 18),
            wxALIGN_CENTER,
            true
    );
}

void MainGamePanel::buildThisPlayer(game_state* gameState, const player* me) {
    // Setup two nested box sizers, in order to align our player's UI to the bottom center
    auto* outerLayout = new wxBoxSizer(wxHORIZONTAL);
    this->SetSizer(outerLayout);
    auto* innerLayout = new wxBoxSizer(wxVERTICAL);
    outerLayout->Add(innerLayout, 1, wxALIGN_BOTTOM);

    // Show the label with our player name
    wxStaticText* playerName = buildStaticText(
            me->get_player_name(),
            wxDefaultPosition,
            wxSize(200, 18),
            wxALIGN_CENTER,
            true
    );
    innerLayout->Add(playerName, 0, wxALIGN_CENTER, 4);

    // If the game has not yet started we say so and offer to start the game
    if(!gameState->is_started()) {

        wxStaticText* waitForStart = buildStaticText(
                "Waiting for poker game to start...",
                wxDefaultPosition,
                wxSize(200, 18),
                wxALIGN_CENTER
        );
        innerLayout->Add(waitForStart, 0, wxALIGN_CENTER | wxBOTTOM, 4);

        // show button that allows our player to start the game
        MainGamePanel::makeButton(innerLayout, "Start Game!", GameController::startGame, wxSize(160, 64));
        return;
    }

    //Game has already started
    // Show our player balance
    wxStaticText *playerBalance = buildStaticText(
            "Remaining Credits: " + std::to_string(me->get_balance()),
            wxDefaultPosition,
            wxSize(200, 18),
            wxALIGN_CENTER
    );

    innerLayout->Add(playerBalance, 0, wxALIGN_CENTER | wxBOTTOM, 0);

    // If our player folded, we display that as status and do not have to display any actions
    if (me->has_folded()) {
        wxStaticText *playerStatus = buildStaticText(
                "You folded.",
                wxDefaultPosition,
                wxSize(200, 18),
                wxALIGN_CENTER
        );
        innerLayout->Add(playerStatus, 0, wxALIGN_CENTER | wxBOTTOM, 8);

        return;
    }

    //Case: Game has started and our player has not folded
    // Show our players current bet
    wxStaticText *playerBet = buildStaticText(
            "Current Bet: " + std::to_string(me->get_current_bet()),
            wxDefaultPosition,
            wxSize(200, 18),
            wxALIGN_CENTER
    );

    innerLayout->Add(playerBet, 0, wxALIGN_CENTER | wxBOTTOM, 8);

    auto player_cards = me->get_cards();
    std::string firstCardFile = player_cards[0];
    std::string secondCardFile = player_cards[1];

    MainGamePanel::displayHand(innerLayout, firstCardFile, secondCardFile);

    // If it's not our turn, we display that as status and do not have to display any actions
    if(!gameState->is_my_turn()) {
        wxStaticText *waitingForOthers = buildStaticText(
                "Waiting for other players...",
                wxDefaultPosition,
                wxSize(200, 18),
                wxALIGN_CENTER
        );
        innerLayout->Add(waitingForOthers, 0, wxALIGN_CENTER | wxBOTTOM, 8);

        return;
    }
    std::vector<std::string> actions = {"fold"};

    int highest_other_bet = 0;
    for(auto p: gameState->get_players()) {
        highest_other_bet = std::max(p.get_current_bet() , highest_other_bet);
    }

    if(highest_other_bet > me->get_current_bet()) {
        //Need to add some to the game to continue
        actions.emplace_back("call");
    } else {
        //Nothing to do here
        actions.emplace_back("check");
    }

    switch (gameState->get_round_number()) {
        case 0:
        case 2:
        case 4:
        case 6:
            //Here we can also raise
            actions.emplace_back("raise");
            break;
        case 1:
        case 3:
        case 5:
        case 7:
            //Nothing to do here
            break;
        default:
            //Unknown round, just display everything
            //Should not happen
            actions = {"fold", "call", "raise", "check"};
    }

    MainGamePanel::buildActionButtons(innerLayout, actions);
}


// Helper methods for building the UI
void MainGamePanel::displayHand(wxBoxSizer *sizer, const std::string& firstCardFile, const std::string& secondCardFile){
    // create horizontal layout for the individual hand cards of our player
    auto *handLayout = new wxBoxSizer(wxHORIZONTAL);
    sizer->Add(handLayout, 0, wxALIGN_CENTER);

    //Display cards
    auto *firstCard = new ImagePanel(this, firstCardFile, wxBITMAP_TYPE_ANY, wxDefaultPosition,
                                     MainGamePanel::cardSize);
    firstCard->SetToolTip("Those are your cards!");
    handLayout->Add(firstCard, 0, wxLEFT | wxRIGHT, 4);

    auto *secondCard = new ImagePanel(this, secondCardFile, wxBITMAP_TYPE_ANY, wxDefaultPosition,
                                      MainGamePanel::cardSize);
    secondCard->SetToolTip("Those are your cards!");
    handLayout->Add(secondCard, 0, wxLEFT | wxRIGHT, 4);

    sizer->AddSpacer(10);

}

void MainGamePanel::makeButton(wxBoxSizer *sizer, const std::string& buttonText, const std::function<void()>& callback, const wxSize& size = wxSize(80, 32)) {
    auto *button = new wxButton(this, wxID_ANY, buttonText, wxDefaultPosition, size);
    button->Bind(wxEVT_BUTTON, [callback](wxCommandEvent &event) {
        callback();
    });
    sizer->Add(button, 0, wxALIGN_CENTER | wxBOTTOM, 8);
    sizer->AddSpacer(10); // add space of 10 pixels
}

void MainGamePanel::buildActionButtons(wxBoxSizer *sizer, const std::vector<std::string>& available_actions) {
    auto *buttonSizer = new wxBoxSizer(wxHORIZONTAL);

    // define map to store function pointers
    std::map<std::string, std::function<void()>> action_functions {
            {"fold", GameController::fold},
            {"call", GameController::call},
            {"raise", GameController::raise},
            {"check", GameController::check},
    };

    // make Button for every available action
    for (auto &action : available_actions) {
        auto it = action_functions.find(action);
        if (it != action_functions.end()) {
            makeButton(buttonSizer, action, it->second);
        }
    }

    sizer->Add(buttonSizer, 0, wxALIGN_CENTER_HORIZONTAL);
}

wxStaticText* MainGamePanel::buildStaticText(const std::string& content, wxPoint position, wxSize size, long textAlignment, bool bold) {
    auto* staticText = new wxStaticText(this, wxID_ANY, content, position, size, textAlignment);
    if(bold) {
        wxFont font = staticText->GetFont();
        font.SetWeight(wxFONTWEIGHT_BOLD);
        staticText->SetFont(font);
    }
    return staticText;
}

wxSize MainGamePanel::getBoundsOfRotatedSquare(double rotationAngle) {
    double newEdgeLength = this->getEdgeLengthOfRotatedSquare(rotationAngle);
    return {static_cast<int>(newEdgeLength), static_cast<int>(newEdgeLength)};
}

double MainGamePanel::getEdgeLengthOfRotatedSquare(double rotationAngle) {
    return 80.0 * (abs(sin(rotationAngle)) + abs(cos(rotationAngle)));
}

wxPoint MainGamePanel::getPointOnEllipse(double horizontalRadius, double verticalRadius, double angle) {
    return {(int) (sin(angle) * horizontalRadius), (int) (cos(angle) * verticalRadius)};
}
