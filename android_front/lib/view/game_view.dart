import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:miku/model/game_started.dart';
import 'package:web_socket_channel/web_socket_channel.dart';

import '../model/user_model.dart';
import 'chat_view.dart';

class GameView extends StatefulWidget {
  GameView(
      {super.key,
      required this.gameStarted,
      required this.channel,
      required this.user});

  WebSocketChannel channel;
  GameStarted gameStarted;
  User user;

  @override
  _GameViewState createState() =>
      _GameViewState(gameStarted: gameStarted, channel: channel, user: user);
}

class _GameViewState extends State<GameView> {
  _GameViewState(
      {required this.gameStarted, required this.channel, required this.user});

  WebSocketChannel channel;
  GameStarted gameStarted;
  User user;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        automaticallyImplyLeading: false,
        backgroundColor: const Color(0xFF21262B),
        leading: IconButton(
          icon: const Icon(Icons.chat_bubble),
          onPressed: () {
            Navigator.push(
                context,
                MaterialPageRoute(
                    builder: (context) =>
                        ChatView(channel: channel, user: user)));
          },
        ),
      ),
    );
  }
}
