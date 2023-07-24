import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:web_socket_channel/web_socket_channel.dart';

import '../../api/game_manager/request/create_lobby_from_history_request.dart';
import '../../model/history/game_move_history_model.dart';

class GameLoseDialog extends StatefulWidget {

  @override
  _GameLoseDialogState createState() => _GameLoseDialogState();
}

class _GameLoseDialogState extends State<GameLoseDialog> {

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      backgroundColor: const Color(0xFF21262B),
      title: const Text(
        "You are a loser !",
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