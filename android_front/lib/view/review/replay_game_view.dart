import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter_svg/svg.dart';

import '../../model/history/game_move_history_model.dart';
import 'confirm_close_replay_dialog.dart';

class ReplayGameView extends StatefulWidget {
  ReplayGameView({super.key, required this.gameMoveHistory});

  List<GameMoveHistory> gameMoveHistory;

  @override
  _ReplayGameViewState createState() => _ReplayGameViewState(
    gameMoveHistory: gameMoveHistory,
  );
}

class _ReplayGameViewState extends State<ReplayGameView> {
  _ReplayGameViewState({required this.gameMoveHistory});

  List<GameMoveHistory> gameMoveHistory;
  int move_index = 0;

  @override
  Widget build(BuildContext context) {
    return WillPopScope(
      onWillPop: () {
        return showDialog(
          context: context,
          builder: (context) => ConfirmCloseReplayDialog(),
        ).then( (leave) {
          if (leave) {
            return true;
          }
          return false;
        });
      },
      child: Scaffold(
        backgroundColor: const Color(0xFF21262B),
        body: Column(
          children: [
            Expanded(
              child: Center(
                 child: SvgPicture.string(gameMoveHistory[move_index].game_state.createSVG()),
               ),
            ),
            Center(
              child: Row(
                children: [
                  IconButton(
                    onPressed: () {
                      if (move_index > 0) {
                        setState(() {
                          move_index -= 1;
                        });
                      }
                    },
                    color: (move_index <= 0) ? const Color(0xFF222ab7) : const Color(0xFF626af7),
                    icon: const Icon(Icons.navigate_before),
                    tooltip: 'Before',
                  ),
                  IconButton(
                    onPressed: () {
                      if (move_index < gameMoveHistory.length - 1) {
                        setState(() {
                          move_index += 1;
                        });
                      }
                    },
                    color: (move_index >= gameMoveHistory.length - 1) ? const Color(0xFF222ab7) : const Color(0xFF626af7),
                    icon: const Icon(Icons.navigate_next),
                    tooltip: 'Next',
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}