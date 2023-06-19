cmake_minimum_required(VERSION 3.15)
project(Pokerers) # your project name

set(CMAKE_CXX_STANDARD 20)

include_directories(sockpp/include)

find_package(wxWidgets COMPONENTS core base net REQUIRED)
include(${wxWidgets_USE_FILE})

# define a variable CLIENT_SOURCE_FILES that contains the paths to all source files required to compile the client executable
set(CLIENT_SOURCE_FILES
        src/client/main.cpp
        src/client/app/Poker.cpp src/client/app/Poker.h
        src/client/GameController.cpp src/client/GameController.h
        # UI
        src/client/windows/GameWindow.cpp src/client/windows/GameWindow.h
        src/client/uiElements/ImagePanel.cpp src/client/uiElements/ImagePanel.h
        src/client/panels/ConnectionPanel.cpp src/client/panels/ConnectionPanel.h
        src/client/panels/MainGamePanel.cpp src/client/panels/MainGamePanel.h
        src/client/uiElements/InputField.cpp src/client/uiElements/InputField.h
        src/client/uiElements/ImagePanel.cpp src/client/uiElements/ImagePanel.h
        # network
        src/client/network/ClientNetworkManager.cpp src/client/network/ClientNetworkManager.h
        src/client/network/ResponseListenerThread.cpp src/client/network/ResponseListenerThread.h
        # game state
        src/common/game_state/card.cpp src/common/game_state/card.h
        src/common/game_state/game_state.cpp src/common/game_state/game_state.h
        src/common/game_state/player.cpp src/common/game_state/player.h
        # client requests
        src/common/network/requests/client_request.cpp src/common/network/requests/client_request.h
        src/common/network/requests/raise_request.cpp src/common/network/requests/raise_request.h
        src/common/network/requests/fold_request.cpp src/common/network/requests/fold_request.h
        src/common/network/requests/set_display_name_request.cpp src/common/network/requests/set_display_name_request.h
        src/common/network/requests/is_ready_request.cpp src/common/network/requests/is_ready_request.h
        src/common/network/requests/heartbeat_request.cpp src/common/network/requests/heartbeat_request.h
        src/common/network/requests/pass_request.cpp src/common/network/requests/pass_request.h

        # server responses
        src/common/network/responses/server_response.cpp src/common/network/responses/server_response.h
        src/common/network/responses/awaiting_player_response.cpp src/common/network/responses/awaiting_player_response.h
        src/common/network/responses/game_state_response.cpp src/common/network/responses/game_state_response.h
        src/common/network/responses/game_end_response.cpp src/common/network/responses/game_end_response.h

        # serialization
        src/common/serialization/serializable.h
        src/common/serialization/value_type_helpers.h
        src/common/serialization/vector_utils.h
        src/common/serialization/serializable_value.h
        src/common/serialization/json_utils.h
        src/common/serialization/uuid_generator.h
        src/common/serialization/unique_serializable.cpp src/common/serialization/unique_serializable.h)

# set source files for client-executable
add_executable(Pokerers-client ${CLIENT_SOURCE_FILES})
# set compile directives for client-executable
target_compile_definitions(Pokerers-client PRIVATE LAMA_CLIENT=1 RAPIDJSON_HAS_STDSTRING=1)
# link with wxWidgets
target_link_libraries(Pokerers-client ${wxWidgets_LIBRARIES})
# Comment out if you don't want to print network-related messages into the console
target_compile_definitions(Pokerers-client PRIVATE PRINT_NETWORK_MESSAGES=1)

# linking to sockpp
if(WIN32)
    message("Detected Win32")

    target_link_libraries(Pokerers-client ${CMAKE_SOURCE_DIR}/sockpp/cmake-build-debug/sockpp-static.lib)

    # Necessary to get sockets working under Windows (with MingW)
    target_link_libraries(Pokerers-client wsock32 ws2_32)
elseif (APPLE)
    message("Detected Apple")

    set(THREADS_PREFER_PTHREAD_FLAG ON)
    find_package(Threads REQUIRED)

    target_link_libraries(Pokerers-client ${CMAKE_SOURCE_DIR}/sockpp/cmake-build-debug/libsockpp.dylib Threads::Threads)

else()
    message("Not Win32 or APPLE, so probably a Linux") # We assume it's Linux in this case

    set(THREADS_PREFER_PTHREAD_FLAG ON)
    find_package(Threads REQUIRED)

    target_link_libraries(Pokerers-client ${CMAKE_SOURCE_DIR}/sockpp/cmake-build-debug/libsockpp.so Threads::Threads)
endif()

# copy assets (images) to binary directory
file(INSTALL assets DESTINATION ${CMAKE_BINARY_DIR})

set(CMAKE_CXX_FLAGS "--coverage")

#add_subdirectory(googletest)
#add_subdirectory(unit-tests)