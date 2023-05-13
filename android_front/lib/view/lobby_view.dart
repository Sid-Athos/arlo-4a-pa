import 'dart:convert';

import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:miku/api/game_manager/request/exit_lobby_request.dart';
import 'package:provider/provider.dart';
import 'package:web_socket_channel/web_socket_channel.dart';

import '../model/lobby_model.dart';

class LobbyView extends StatefulWidget {
  LobbyView({super.key, required this.channel});
  WebSocketChannel channel;

  @override
  _LobbyViewState createState() => _LobbyViewState(channel: channel);
}

class _LobbyViewState extends State<LobbyView> {
  _LobbyViewState({required this.channel});

  WebSocketChannel channel;

  @override
  Widget build(BuildContext context) {

    final lobby = Provider.of<Lobby>(context);

    return Scaffold(
      body: Center(
        child: Text(lobby.code),
      ),
    );
  }

  @override
  void dispose() {
    channel.sink.add(ExitLobbyRequest.toJson());
    super.dispose();
  }
}