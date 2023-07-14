import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:miku/api/game_manager/request/accept_invite_lobby_request.dart';
import 'package:miku/api/game_manager/request/decline_invite_lobby_request.dart';
import 'package:miku/model/ice_candidate_model.dart';
import 'package:miku/model/invite_model.dart';
import 'package:miku/model/mapper/game_started_mapper.dart';
import 'package:miku/model/mapper/game_svg_info_response_mapper.dart';
import 'package:miku/model/mapper/user_response_mapper.dart';
import 'package:miku/view/friend_list_view.dart';

import 'package:miku/view/game_list_view.dart';
import 'package:miku/view/game_view.dart';
import 'package:miku/view/profile_view.dart';
import 'package:overlay_support/overlay_support.dart';
import 'package:provider/provider.dart';
import 'package:web_socket_channel/web_socket_channel.dart';
import 'dart:developer' as developer;

import '../api/game_manager/response/emote_response_ws.dart';
import '../api/game_manager/response/message_response_ws.dart';
import '../model/game_model.dart';
import '../model/lobby_model.dart';
import '../model/mapper/invite_response_mapper.dart';
import '../model/user_model.dart';
import '../provider/game_provider.dart';
import 'lobby_view.dart';

enum TabItem { friends, game, profile }

class HomeView extends StatefulWidget {
  static String routeName = "Home";

  HomeView({Key? key, required this.channel, required this.user})
      : super(key: key);

  WebSocketChannel channel;
  User user;

  @override
  State<HomeView> createState() => _HomeState(this.channel, this.user);
}

class _HomeState extends State<HomeView> {
  _HomeState(this.channel, this.user);

  WebSocketChannel channel;
  User user;

  List<GlobalKey<NavigatorState>> navigatorKeys = [
    GlobalKey<NavigatorState>(),
    GlobalKey<NavigatorState>(),
    GlobalKey<NavigatorState>(),
  ];
  int currentTab = 1;
  Lobby lobby = Lobby(
      id: 0,
      code: "",
      gameId: 0,
      private: false,
      members: [],
      game:
      Game(id: 0,
          name: "",
          description: "",
          minPlayers: 0,
          maxPlayers: 0));
  GameProvider gameProvider = GameProvider(
      messages: [],
      isShowChat: false,
      channel: null
  );

  String tmpSvgDisplayData = '{"width": "300","height": "300","content": [{"tag": "style","content": "line{stroke:black;stroke-width:4;}"},{"tag": "line","x1": "0","y1": "100","x2": "300","y2": "100"},{"tag": "line","x1": "100","y1": "0","x2": "100","y2": "300"},{"tag": "line","x1": "0","y1": "200","x2": "300","y2": "200"},{"tag": "line","x1": "200","y1": "0","x2": "200","y2": "300"},{"tag": "circle","cx": "50","cy": "50","r": "33","fill": "blue"}],"player": 2}';

  @override
  void initState() {
    gameProvider.channel = channel;
    super.initState();

    channel.stream.listen((message) {
      developer.log("WS Received : $message");

      switch (message) {
        case "\"BadMessage\"":
          developer.log("BadMessage");
          return;
        case "\"Pong\"":
          developer.log("Pong");
          return;
        case "\"LobbyJoined\"":
        case "\"LobbyCreated\"":
          navigatorKeys[1]
              .currentState
              ?.push(
            MaterialPageRoute(
              builder: (BuildContext context) =>
                  ChangeNotifierProvider(
                    create: (context) => lobby,
                    builder: (context, child) =>
                        LobbyView(channel: channel, user: user),
                  ),
            ),
          )
              .then((value) =>
          lobby = Lobby(
              id: 0,
              code: "",
              gameId: 0,
              private: false,
              members: [],
              game: Game(
                  id: 0,
                  name: "",
                  description: "",
                  minPlayers: 0,
                  maxPlayers: 0)));
          return;
        case "\"LobbyExited\"":
          developer.log("LobbyExited");
          return;
        case "\"Kicked\"":
          navigatorKeys[1].currentState?.pop(context);
          developer.log("Kicked");
          return;
        case "\"UserInvited\"":
          developer.log("UserInvited");
          return;
        case "\"CannotStartGame\"":
          return;
      }

      Map<String, dynamic> json = jsonDecode(message);
      for (var key in json.keys) {
        switch (key) {
          case "Message":
            gameProvider.setSvgDisplay(GameSvgInfoResponseMapper.fromJson(jsonDecode(tmpSvgDisplayData)));
            gameProvider.addMessage(MessageResponseWS.fromJson(json["Message"]));
            break;
          case "Emote":
            gameProvider.addEmote(EmoteResponseWS.fromJson(json["Emote"]));
            break;
          case "Lobby":
            lobby.update(json["Lobby"]);
            break;
          case "GameStarted":
            navigatorKeys[1]
                .currentState
                ?.push(
              MaterialPageRoute(
                builder: (BuildContext context) =>
                    ChangeNotifierProvider(
                      create: (context) => gameProvider,
                      builder: (context, child) =>
                          GameView(
                            gameStarted:
                            GameStartedMapper.fromJson(json["GameStarted"]),
                            user: user,
                            channel: channel,
                          ),
                    ),
              ),
            )
                .then((value) =>
            gameProvider = GameProvider(
                messages: [],
                isShowChat: false,
                channel: channel
            ));
            break;
          case "SDPOffer":
            gameProvider.answerSdpOffer(json["SDPOffer"]["sdp"], UserResponseMapper.fromJson(json["SDPOffer"]["from_user"]));
            break;
          case "SDPAnswer":
            gameProvider.setRemoteAnswer(json["SDPAnswer"]["sdp"], UserResponseMapper.fromJson(json["SDPAnswer"]["from_user"]));
            break;
          case "ICECandidate":
            gameProvider.addIceCandidate(ICECandidate(
                candidate: json["ICECandidate"]["candidate"],
                sdp_mid: json["ICECandidate"]["sdp_mid"],
                sdp_m_line_index: json["ICECandidate"]["sdp_m_line_index"]),
                UserResponseMapper.fromJson(json["ICECandidate"]["from_user"])
            );
            break;
          case "UserLeftRtcSession":
            gameProvider.userLeftCall(json["UserLeftRtcSession"]["user_id"]);
            break;
          case "InviteReceived":
            developer.log("InviteReceived");
            showNotificationInvitedInLobby(
                InviteResponseMapper.fromJson(json["InviteReceived"]));
            break;
        }
      }
    });
  }

  void showNotificationInvitedInLobby(Invite invite) {
    showOverlayNotification((context) {
      return Card(
          color: const Color(0xFF3A4045),
          margin: const EdgeInsets.symmetric(horizontal: 4),
          child: Padding(
            padding: const EdgeInsets.only(top: 32),
            child: Row(
              children: [
                Expanded(
                  child: ListTile(
                    title: Text(
                      'Invite Received From ${invite.from.pseudo}',
                      style: const TextStyle(color: Colors.white),
                    ),
                    subtitle: Text(
                      'Game : ${invite.lobby.game.name}',
                      style: const TextStyle(color: Colors.white38),
                    ),
                  ),
                ),
                IconButton(
                    icon: const Icon(
                      Icons.check,
                      color: Colors.white,
                    ),
                    onPressed: () {
                      channel.sink
                          .add(AcceptInviteLobbyRequest.toJson(invite.from.id));
                      OverlaySupportEntry.of(context)?.dismiss();
                    }),
                IconButton(
                    icon: const Icon(
                      Icons.close,
                      color: Colors.white,
                    ),
                    onPressed: () {
                      channel.sink.add(
                          DeclineInviteLobbyRequest.toJson(invite.from.id));
                      OverlaySupportEntry.of(context)?.dismiss();
                    }),
              ],
            ),
          ));
    }, duration: const Duration(days: 1));
  }

  void _onItemTapped(int index) {
    setState(() {
      currentTab = index;
    });
  }

  Widget _buildFriendsViewNavigator() {
    return Offstage(
      offstage: currentTab != 0,
      child: Navigator(
          key: navigatorKeys[0],
          onGenerateRoute: (settings) {
            return MaterialPageRoute(
              builder: (context) => FriendListView(user: user),
            );
          }),
    );
  }

  Widget _buildGameViewNavigator() {
    return Offstage(
      offstage: currentTab != 1,
      child: Navigator(
          key: navigatorKeys[1],
          onGenerateRoute: (settings) {
            return MaterialPageRoute(
              builder: (context) =>
                  GameScreen(
                    channel: channel,
                    lobby: lobby,
                  ),
            );
          }),
    );
  }

  Widget _buildProfileViewNavigator() {
    return Offstage(
      offstage: currentTab != 2,
      child: Navigator(
          key: navigatorKeys[2],
          onGenerateRoute: (settings) {
            return MaterialPageRoute(
              builder: (context) => ProfileView(user: user),
            );
          }),
    );
  }

  @override
  Widget build(BuildContext context) {
    SystemChrome.setPreferredOrientations([
      DeviceOrientation.portraitUp,
      DeviceOrientation.portraitDown,
    ]);
    return WillPopScope(
      onWillPop: () async =>
      !await navigatorKeys[currentTab].currentState!.maybePop(),
      child: Scaffold(
        backgroundColor: const Color(0xFF21262B),
        body: Stack(children: <Widget>[
          _buildFriendsViewNavigator(),
          _buildGameViewNavigator(),
          _buildProfileViewNavigator(),
        ]),
        bottomNavigationBar: BottomNavigationBar(
          items: const <BottomNavigationBarItem>[
            BottomNavigationBarItem(
              icon: Icon(
                Icons.people,
              ),
              label: 'Friends',
            ),
            BottomNavigationBarItem(
              icon: Icon(
                Icons.videogame_asset,
              ),
              label: 'Home',
            ),
            BottomNavigationBarItem(
              icon: Icon(
                Icons.account_circle,
              ),
              label: 'Profile',
            ),
          ],
          backgroundColor: const Color(0xFF1A2025),
          currentIndex: currentTab,
          selectedItemColor: const Color(0xFF626af7),
          onTap: _onItemTapped,
          unselectedItemColor: Colors.white,
        ),
      ),
    );
  }
}