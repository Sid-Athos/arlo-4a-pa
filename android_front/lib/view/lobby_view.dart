import 'dart:convert';

import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:miku/api/game_manager/request/exit_lobby_request.dart';
import 'package:miku/model/lobby_member_model.dart';
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
      appBar: AppBar(
        title: const Text(
            "Lobby"
        ),
        backgroundColor: const Color(0xFF21262B),
        leading: IconButton(
          icon: const Icon(Icons.close),
          onPressed: () {
            channel.sink.add(ExitLobbyRequest.toJson());
            Navigator.pop(context);
          },
        )
      ),
      backgroundColor: const Color(0xFF21262B),
      body: Center(
        child: Column(
          children: <Widget>[
            Expanded(
              child: ListView.builder(
                itemCount: lobby.members.length,
                itemBuilder: (context, index) {
                  return LobbyMemberCardWidget(
                    lobbyMember: lobby.members[index],
                    channel: channel,
                  );
                },
              ),
            ),
          ],
        ),
      ),
    );
  }

  @override
  void dispose() {
    channel.sink.add(ExitLobbyRequest.toJson());
    super.dispose();
  }
}

class LobbyMemberCardWidget extends StatelessWidget {
  LobbyMemberCardWidget({super.key, required this.lobbyMember, required this.channel});

  LobbyMember lobbyMember;
  WebSocketChannel channel;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8.0, horizontal: 16.0),
      child: Card(
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(16),
        ),
        color: const Color(0xFF1A2025),
        child: Padding(
          padding:
          const EdgeInsets.only(bottom: 16.0, right: 32.0, left: 16.0),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: <Widget>[
              ListTile(
                title: Text(
                  lobbyMember.pseudo,
                  style: const TextStyle(color: Colors.white),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
