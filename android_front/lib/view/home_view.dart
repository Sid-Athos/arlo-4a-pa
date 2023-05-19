import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:miku/api/user/api_user.dart';
import 'package:miku/view/friend_list_view.dart';

import 'package:miku/view/game_list_view.dart';
import 'package:miku/view/profile_view.dart';
import 'package:provider/provider.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:web_socket_channel/web_socket_channel.dart';
import 'dart:developer' as developer;

import '../api/game_manager/response/message_response_ws.dart';
import '../model/lobby_model.dart';
import '../model/user_model.dart';
import 'lobby_view.dart';

enum TabItem { friends, game, profile }

class HomeView extends StatefulWidget {
  static String routeName = "Home";

  HomeView({Key? key, required this.channel, required this.user}) : super(key: key);

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
  Lobby lobby = Lobby();

  @override
  void initState() {
    super.initState();
    channel.stream.listen((message)
      {
        switch (message) {
          case "\"BadMessage\"": developer.log("BadMessage"); return;
          case "\"Pong\"": developer.log("Pong"); return;
          case "\"LobbyJoined\"":
          case "\"LobbyCreated\"":
          navigatorKeys[1].currentState?.push(
                MaterialPageRoute(
                  builder: (BuildContext context) => ChangeNotifierProvider(
                    create: (context) => lobby,
                    builder: (context, child) => LobbyView(channel: channel, user: user),
                  ),
                ),
            ).then((value) => lobby = Lobby());
            return;
          case "\"LobbyExited\"": developer.log("LobbyExited"); return;
          case "\"Kicked\"": Navigator.pop(context); developer.log("Kicked"); return;
        }

        Map<String, dynamic> json = jsonDecode(message);
        for (var key in json.keys) {
          switch (key) {
            case "Message": MessageResponseWS.compute(json); break;
            case "Lobby":
              lobby.update(json["Lobby"]);
              break;
          }
        }
      }
    );
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
              builder: (context) => const FriendListView(),
            );
          }
      ),
    );
  }

  Widget _buildGameViewNavigator() {
    return Offstage(
      offstage: currentTab != 1,
      child: Navigator(
          key: navigatorKeys[1],
          onGenerateRoute: (settings) {
            return MaterialPageRoute(
              builder: (context) => GameScreen(channel: channel, lobby: lobby,),
            );
          }
      ),
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
          }
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    SystemChrome.setPreferredOrientations([
      DeviceOrientation.portraitUp,
      DeviceOrientation.portraitDown,
    ]);
    return WillPopScope(
      onWillPop: () async => !await navigatorKeys[currentTab].currentState!.maybePop(),
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
