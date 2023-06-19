#include "ConnectionPanel.h"


#include "../uiElements/ImagePanel.h"
#include "../../common/network/default.conf"
#include "../GameController.h"
#include "panels.conf"


ConnectionPanel::ConnectionPanel(wxWindow* parent) : wxPanel(parent, wxID_ANY) {
    // Gets shown when a game client is started

    this->SetBackgroundColour(wxColour(0, 0, 0));

    auto* verticalLayout = new wxBoxSizer(wxVERTICAL);

    auto* logo = new ImagePanel(this, "assets/poker_logo.png", wxBITMAP_TYPE_ANY, wxDefaultPosition, wxSize(604, 402));
    verticalLayout->Add(logo, 0, wxALIGN_CENTER | wxTOP | wxLEFT | wxRIGHT | wxALIGN_CENTER, border);

    this->_serverAddressField = new InputField(
        this, // parent element
        "Server Address:", // label
        input_field_width, // width of label
        default_server_host, // default value (variable from "default.conf")
        input_field_width // width of field
    );

    verticalLayout->Add(this->_serverAddressField, 0, wxTOP | wxLEFT | wxRIGHT | wxALIGN_CENTER, border);

    this->_serverPortField = new InputField(
        this, // parent element
        "Server Port:", // label
        input_field_width, // width of label
        wxString::Format("%i", default_port), // default value (variable from "default.conf")
        input_field_width // width of field
    );

    verticalLayout->Add(this->_serverPortField, 0, wxTOP | wxLEFT | wxRIGHT | wxALIGN_CENTER , border);

    this->_playerNameField = new InputField(
        this, // parent element
        "Player Name:", // label
        input_field_width, // width of label
        "", // default value
        input_field_width // width of field
    );

    verticalLayout->Add(this->_playerNameField, 0, wxTOP | wxLEFT | wxRIGHT | wxALIGN_CENTER, border);

    auto* connectButton = new wxButton(this, wxID_ANY, "Connect", wxDefaultPosition, wxSize(604, 40));
    connectButton->Bind(wxEVT_BUTTON, [](wxCommandEvent& event) {
        GameController::connectToServer();
    });

    verticalLayout->Add(connectButton, 0, wxALIGN_CENTER | wxALL, border);

    this->SetSizerAndFit(verticalLayout);
}


wxString ConnectionPanel::getServerAddress() {
    return this->_serverAddressField->getValue();
}


wxString ConnectionPanel::getServerPort() {
    return this->_serverPortField->getValue();
}


wxString ConnectionPanel::getPlayerName() {
    return this->_playerNameField->getValue();
}
