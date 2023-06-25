import 'package:flutter/material.dart';
import 'package:miku/view/login_view.dart';
import 'package:miku/view/test_webrtc_view.dart';
import 'package:overlay_support/overlay_support.dart';
import 'package:shared_preferences/shared_preferences.dart';

import 'package:miku/view/home_view.dart';
import 'package:web_socket_channel/src/channel.dart';

import 'api/game_manager/api_game_manager.dart';
import 'api/user/api_user.dart';
import 'model/user_model.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();

  SharedPreferences prefs = await SharedPreferences.getInstance();
  //prefs.setString("login_token", "m09TO9xebFj0VoH30CQMBCdJ1tIhRV7jBWpN0jFMrMTOoA4kVP3E8PddcDUrPrgtitR7fdWjT7UfnIYf9mhpgDUhDsvrHj4noKyYA29TZQz74CFEJ9Zhhzo9MD0ju2KRfHl8zG8d44tExhLS8aBd8Y6znRab7QuYNFmPWdghoXpi5pl7J8ITqCzrg2fyw0qWnkEQi9FeSHexeDI8fsdWOsoI4gpDsnfcQEuMZE6RgB7t2W3TGeDSILJkoicJCxim");
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
  //runApp(AppTest());
}

class AppUnLogged extends StatelessWidget {
  const AppUnLogged({super.key});

  @override
  Widget build(BuildContext context) {
    return const OverlaySupport(
      child: MaterialApp(
        title: 'Miku',
        home: LoginPage(),
        debugShowCheckedModeBanner: false,
      ),
    );
  }
}

class AppLogged extends StatelessWidget {
  AppLogged({super.key, required this.channel, required this.user});

  WebSocketChannel channel;
  User user;

  @override
  Widget build(BuildContext context) {
    return OverlaySupport(
      child: MaterialApp(
        title: 'Miku',
        home: HomeView(channel: channel, user: user),
        debugShowCheckedModeBanner: false,
      ),
    );
  }
}
/*
class AppTest extends StatelessWidget {
  AppTest({super.key});

  @override
  Widget build(BuildContext context) {
    return OverlaySupport(
      child: MaterialApp(
        title: 'Miku',
        home: WebRTCSession(),
        debugShowCheckedModeBanner: false,
      ),
    );
  }
}
*/