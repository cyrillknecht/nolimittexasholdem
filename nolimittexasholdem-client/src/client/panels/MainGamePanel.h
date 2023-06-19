#ifndef POKER_CLIENT_MAINGAMEPANEL_H
#define POKER_CLIENT_MAINGAMEPANEL_H

#include <wx/wx.h>
#include "../uiElements/InputField.h"
#include "../../common/game_state/game_state.h"
#include "../uiElements/ImagePanel.h"


class MainGamePanel : public wxPanel {

public:
    explicit MainGamePanel(wxWindow* parent);

    // Build UI
    void buildGameState(game_state* gameState);


private:
    // Build UI elements
    void buildOtherPlayerHand(game_state* gameState, player* otherPlayer, double playerAngle);
    void buildOtherPlayerLabels(game_state* gameState, player* otherPlayer, double playerAngle, int side);
    void buildCenterPile(game_state* gameState);
    void buildTurnIndicator(game_state* gameState, const player* me);
    void buildThisPlayer(game_state* gameState, const player* me);

    // helper methods
    wxStaticText* buildStaticText(const std::string& content, wxPoint position, wxSize size, long textAlignment, bool bold = false);
    wxSize getBoundsOfRotatedSquare(double rotationAngle);
    static double getEdgeLengthOfRotatedSquare(double rotationAngle);
    static wxPoint getPointOnEllipse(double horizontalRadius, double verticalRadius, double angle);
    void makeButton(wxBoxSizer *sizer,
                    const std::string &buttonText,
                    const std::function<void()> &callback,
                    const wxSize &size);
    void buildActionButtons(wxBoxSizer *sizer, const std::vector<std::string>& available_actions);
    void displayHand(wxBoxSizer *sizer, const std::string &firstCardFile, const std::string &secondCardFile);

    // define constant layout values
    wxPoint const tableCenter = wxPoint(480, 300);
    wxSize const cardSize = wxSize(60, 93);
    double const otherPlayerHandDistanceFromCenter = 180.0;
    double const otherPlayerLabelDistanceFromCenter = 255.0;
    wxPoint const turnIndicatorOffset = wxPoint(-100, 98);
    int const borderWidth = 5;
    double const twoPi = 6.28318530718;
};


#endif //POKER_CLIENT_MAINGAMEPANEL_H
