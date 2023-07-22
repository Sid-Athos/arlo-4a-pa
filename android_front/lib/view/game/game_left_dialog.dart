import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:web_socket_channel/web_socket_channel.dart';

import '../../api/game_manager/request/create_lobby_from_history_request.dart';
import '../../model/history/game_move_history_model.dart';

class GameLeftDialog extends StatefulWidget {

  @override
  _GameLeftDialogState createState() => _GameLeftDialogState();
}

class _GameLeftDialogState extends State<GameLeftDialog> {

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      backgroundColor: const Color(0xFF21262B),
      title: const Text(
        "Someone left the game, your game is stopped",
        style: TextStyle(
          color: Colors.white,
        ),
      ),
      actions: [
        TextButton(
          onPressed: () {
            Navigator.of(context).pop();
          },
          child: const Text(
            "Ok",
            style: TextStyle(
              color: Color(0xFF626af7),
            ),
          ),
        ),
      ],
    );
  }
}