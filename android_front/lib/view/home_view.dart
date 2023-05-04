import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:miku/api/game_manager/request/ping_request.dart';
import 'package:miku/view/friends_view.dart';
import 'dart:developer' as developer;

import 'package:miku/view/game_list_view.dart';
import 'package:miku/view/lobby_list_view.dart';
import 'package:miku/view/profile_view.dart';
import 'package:miku/view/tab_navigatior.dart';
import 'package:web_socket_channel/web_socket_channel.dart';

enum TabItem { friends, game, profile }

class HomeView extends StatefulWidget {
  static String routeName = "Home";

  HomeView({Key? key, required this.channel}) : super(key: key);

  WebSocketChannel channel;

  @override
  State<HomeView> createState() => _HomeState(this.channel);
}

class _HomeState extends State<HomeView> {
  List<GlobalKey<NavigatorState>> navigatorKeys = [
    GlobalKey<NavigatorState>(),
    GlobalKey<NavigatorState>(),
    GlobalKey<NavigatorState>(),
  ];

  int currentTab = 1;
  WebSocketChannel channel;

  _HomeState(this.channel);

  void _onItemTapped(int index) {
    channel.sink.add(PingRequest.toJson());

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
              builder: (context) => const FriendsScreen(),
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
              builder: (context) => const GameScreen(),
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
              builder: (context) => const ProfileScreen(),
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
