import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:web_socket_channel/web_socket_channel.dart';

import '../../model/history/game_move_history_model.dart';
import 'create_lobby_from_history_move_dialog.dart';

class GameMoveHistoryCardWidget extends StatelessWidget {
  GameMoveHistoryCardWidget(
      {super.key,
      required this.game_move_history,
      required this.channel});

  GameMoveHistory game_move_history;
  WebSocketChannel channel;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8.0, horizontal: 16.0),
      child: InkWell(
        onTap: () {
          showConfirmationDialog(context);
        },
        child: Card(
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(16),
          ),
          color: const Color(0xFF1A2025),
          child: Padding(
            padding: const EdgeInsets.only(bottom: 8.0, right: 16.0, left: 16.0, top: 8.0),
            child: ListTile(
              title: Text(
                "Player ${game_move_history.player}",
                style: const TextStyle(color: Colors.white),
              ),
              subtitle: Text(
                "N'${game_move_history.action_number}",
                style: const TextStyle(color: Colors.white38),
              ),
            ),
          ),
        ),
      ),
    );
  }

  showConfirmationDialog(BuildContext context) {
    showDialog(
      barrierDismissible: false,
      context: context,
      builder: (BuildContext context) {
        return CreateLobbyFromHistoryMoveDialog(gameMoveHistory: game_move_history, channel: channel);
      },
    );
  }
}