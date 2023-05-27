import 'package:flutter/material.dart';
import 'package:miku/view/login_view.dart';
import 'package:shared_preferences/shared_preferences.dart';

import 'package:miku/view/home_view.dart';
import 'package:web_socket_channel/src/channel.dart';

import 'api/game_manager/api_game_manager.dart';
import 'api/user/api_user.dart';
import 'model/user_model.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();

  SharedPreferences prefs = await SharedPreferences.getInstance();
  if (prefs.containsKey('login_token')) {
    WebSocketChannel? channel = ApiGameManager.openWebSocketConnection(prefs.getString('login_token')!);
    if (channel == null) {
      prefs.remove('login_token');
      runApp(const AppUnLogged());
    } else {
      User? user = await ApiUser.me(prefs.getString('login_token')!);
      if (user == null) {
        prefs.remove('login_token');
        runApp(const AppUnLogged());
      } else {
        runApp(AppLogged(channel: channel, user: user));
      }
    }
  } else {
    runApp(const AppUnLogged());
  }
}

class AppUnLogged extends StatelessWidget {
  const AppUnLogged({super.key});

  @override
  Widget build(BuildContext context) {
    return const MaterialApp(
      title: 'Miku',
      home: LoginPage(),
      debugShowCheckedModeBanner: false,
    );
  }
}

class AppLogged extends StatelessWidget {
  AppLogged({super.key, required this.channel, required this.user});

  WebSocketChannel channel;
  User user;

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Miku',
      home: HomeView(channel: channel, user: user),
      debugShowCheckedModeBanner: false,
    );
  }
}
