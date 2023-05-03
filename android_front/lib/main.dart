import 'package:flutter/material.dart';
import 'package:miku/view/login_view.dart';
import 'package:shared_preferences/shared_preferences.dart';

import 'package:miku/view/home_view.dart';
import 'package:web_socket_channel/src/channel.dart';

import 'api/game_manager/api_game_manager.dart';
import 'api/game_manager/response/response_enum_ws.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();

  SharedPreferences prefs = await SharedPreferences.getInstance();
  if (prefs.containsKey('login_token')) {
    WebSocketChannel channel = ApiGameManager.openWebSocketConnection(prefs.getString('login_token')!);
    channel.stream.listen((message)
      {
        ResponseWS.computeResponse(message);
      }
    );
    //runApp(const AppUnLogged());
    runApp(AppLogged(channel: channel));
  } else {
    runApp(const AppUnLogged());
  }
}

class AppUnLogged extends StatelessWidget {
  const AppUnLogged({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Miku',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: const LoginPage(),
      debugShowCheckedModeBanner: false,
    );
  }
}

class AppLogged extends StatelessWidget {
  AppLogged({super.key, required this.channel});

  WebSocketChannel channel;

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Miku',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: HomeView(channel: channel),
      debugShowCheckedModeBanner: false,
    );
  }
}
