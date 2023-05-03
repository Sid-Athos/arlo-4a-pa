import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:miku/api/game_manager/request/ping_request.dart';
import 'package:miku/view/friends_view.dart';
import 'dart:developer' as developer;

import 'package:miku/view/game_view.dart';
import 'package:miku/view/profile_view.dart';
import 'package:web_socket_channel/web_socket_channel.dart';

class HomeView extends StatefulWidget {
  static String routeName = "Home";

  HomeView({Key? key, required this.channel}) : super(key: key);

  WebSocketChannel channel;

  @override
  State<HomeView> createState() => _HomeState(this.channel);
}

class _HomeState extends State<HomeView> {

  int _selectedIndex = 1;
  WebSocketChannel channel;

  static final List<Widget> _widgetOptions = <Widget>[
    friendsWidget(),
    const GameScreen(),
    profileWidget(),
  ];

  _HomeState(this.channel);

  void _onItemTapped(int index) {

    channel.sink.add(PingRequest.toJson());

    setState(() {
      _selectedIndex = index;
    });
  }

  @override
  Widget build(BuildContext context) {
    SystemChrome.setPreferredOrientations([
      DeviceOrientation.portraitUp,
      DeviceOrientation.portraitDown,
    ]);
    return Scaffold(
      backgroundColor: const Color(0xFF21262B),
      body: Center(
        child: _widgetOptions.elementAt(_selectedIndex),
      ),
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
        currentIndex: _selectedIndex,
        selectedItemColor: const Color(0xFF626af7),
        onTap: _onItemTapped,
        unselectedItemColor: Colors.white,
      ),
    );
  }

}