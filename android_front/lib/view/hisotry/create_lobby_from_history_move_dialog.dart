import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:web_socket_channel/web_socket_channel.dart';

import '../../api/game_manager/request/create_lobby_from_history_request.dart';
import '../../model/history/game_move_history_model.dart';

class CreateLobbyFromHistoryMoveDialog extends StatefulWidget {
  CreateLobbyFromHistoryMoveDialog({required this.gameMoveHistory, required this.channel});

  GameMoveHistory gameMoveHistory;
  WebSocketChannel channel;

  @override
  _CreateLobbyFromHistoryMoveDialogState createState() => _CreateLobbyFromHistoryMoveDialogState(gameMoveHistory: gameMoveHistory, channel: channel);
}

class _CreateLobbyFromHistoryMoveDialogState extends State<CreateLobbyFromHistoryMoveDialog> {
  _CreateLobbyFromHistoryMoveDialogState({required this.gameMoveHistory, required this.channel});

  GameMoveHistory gameMoveHistory;
  WebSocketChannel channel;

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      backgroundColor: const Color(0xFF21262B),
      title: Text(
        "Do you want to create a lobby from move N'${gameMoveHistory.action_number} ?",
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
            "Cancel",
            style: TextStyle(
              color: Color(0xFF626af7),
            ),
          ),
        ),
        TextButton(
          onPressed: () {
            channel.sink.add(CreateLobbyFromHistoryRequest.toJson(gameMoveHistory.id));
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