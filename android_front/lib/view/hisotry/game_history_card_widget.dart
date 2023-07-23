import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:web_socket_channel/web_socket_channel.dart';

import '../../model/history/game_history_model.dart';
import 'history_game_move_view.dart';

class GameHistoryCardWidget extends StatelessWidget {
  GameHistoryCardWidget(
      {super.key,
      required this.game_history,
      required this.channel});

  GameHistory game_history;
  WebSocketChannel channel;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8.0, horizontal: 16.0),
      child: InkWell(
        onTap: () {
          Navigator.push(
            context,
            MaterialPageRoute(builder: (context) => HistoryGameMoveView(game_history: game_history, channel: channel,))
          );
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
                game_history.game.name,
                style: const TextStyle(color: Colors.white),
              ),
              subtitle: Text(
                "${game_history.dateTime.month}/${game_history.dateTime.day}/${game_history.dateTime.year} ${game_history.dateTime.hour}:${game_history.dateTime.minute}",
                style: const TextStyle(color: Colors.white38),
              ),
            ),
          ),
        ),
      ),
    );
  }
}