import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:miku/view/friends_view.dart';
import 'dart:developer' as developer;

import 'package:miku/view/game_view.dart';
import 'package:miku/view/profile_view.dart';

class HomeView extends StatefulWidget {
  static String routeName = "Home";

  const HomeView({Key? key}) : super(key: key);

  @override
  State<HomeView> createState() => _HomeState();
}

class _HomeState extends State<HomeView> {

  int _selectedIndex = 1;
  static final List<Widget> _widgetOptions = <Widget>[
    friendsWidget(),
    gameWidget(),
    profileWidget(),
  ];

  void _onItemTapped(int index) {

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
      body: Center(
        child: Container(
          decoration: const BoxDecoration(
            color: Color(0xFF1A2025)
          ),
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: <Widget>[
              _widgetOptions.elementAt(_selectedIndex)
            ],
          ),
        ),
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