import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:web_socket_channel/web_socket_channel.dart';

import '../../api/game_manager/request/create_lobby_request.dart';
import '../../model/game/game_model.dart';

class CreateLobbyDialog extends StatefulWidget {
  CreateLobbyDialog({required this.game, required this.channel});

  Game game;
  WebSocketChannel channel;

  @override
  _CreateLobbyDialogState createState() => _CreateLobbyDialogState(game: game, channel: channel);
}

class _CreateLobbyDialogState extends State<CreateLobbyDialog> {
  _CreateLobbyDialogState({required this.game, required this.channel});

  Game game;
  WebSocketChannel channel;
  bool _isChecked = false;

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      backgroundColor: const Color(0xFF21262B),
      title: const Text(
        "Create Lobby",
        style: TextStyle(
          color: Colors.white,
        ),
      ),
      content: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          Text(
            "Number of Players  : ${game.minPlayers} - ${game.maxPlayers}",
            style: const TextStyle(
                color: Colors.white
            ),
          ),
          Theme(
              data: Theme.of(context).copyWith(
                unselectedWidgetColor: Colors.white,
              ),
              child: Padding(
                padding: const EdgeInsets.only(top: 32.0),
                child: CheckboxListTile(
                  title: const Text(
                    "Private",
                    style: TextStyle(
                      color: Colors.white,
                    ),
                  ),
                  value: _isChecked,
                  onChanged: (bool? val) {
                    setState(() {
                      _isChecked = val!;
                    });
                  },
                ),
              )
          ),
        ],
      ),
      actions: [
        TextButton(
          onPressed: () {
            Navigator.of(context).pop();
          },
          child: const Text(
            "Cancel",
            style: TextStyle(
              color: Color(0xFF626af7),
            ),
          ),
        ),
        TextButton(
          onPressed: () {
            channel.sink.add(CreateLobbyRequest.toJson(game.id, _isChecked));
            Navigator.of(context).pop();
          },
          child: const Text(
            "Create",
            style: TextStyle(
              color: Color(0xFF626af7),
            ),
          ),
        ),
      ],
    );
  }
}