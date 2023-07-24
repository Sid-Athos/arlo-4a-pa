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
  int player_index = 0;

  @override
  Widget build(BuildContext context) {
    return WillPopScope(
      onWillPop: () {
        return showDialog(
          context: context,
          builder: (context) => ConfirmCloseReplayDialog(),
        ).then((leave) {
          if (leave) {
            return true;
          }
          return false;
        });
      },
      child: Scaffold(
        appBar: AppBar(
          title: const Text("Game replay"),
          backgroundColor: const Color(0xFF21262B),
        ),
        backgroundColor: const Color(0xFF21262B),
        body: Column(
          children: [
            Expanded(
              child: Center(
                child: SvgPicture.string(gameMoveHistory[move_index]
                    .game_state
                    .displays[player_index]
                    .createSVG()),
              ),
            ),
            Padding(
              padding: const EdgeInsets.only(bottom: 25),
              child: Row(
                mainAxisAlignment: MainAxisAlignment.center,
                mainAxisSize: MainAxisSize.max,
                children: [
                  Padding(
                    padding: const EdgeInsets.only(right: 25),
                    child: IconButton(
                      onPressed: () {
                        if (move_index > 0) {
                          setState(() {
                            move_index -= 1;
                          });
                        }
                      },
                      color: (move_index <= 0)
                          ? const Color(0xFF222ab7)
                          : const Color(0xFF626af7),
                      icon: const Icon(Icons.navigate_before),
                    ),
                  ),
                  ElevatedButton(
                      onPressed: () {
                        setState(() {
                          player_index += 1;
                          if (player_index >= gameMoveHistory[move_index]
                              .game_state
                              .displays.length) {
                            player_index = 0;
                          }
                        });
                      },
                      style: ElevatedButton.styleFrom(
                          backgroundColor: const Color(0xFF626af7)
                      ),
                      child: Text("P${player_index + 1}")),
                  Padding(
                    padding: const EdgeInsets.only(left: 25),
                    child: IconButton(
                      onPressed: () {
                        if (move_index < gameMoveHistory.length - 1) {
                          setState(() {
                            move_index += 1;
                          });
                        }
                      },
                      color: (move_index >= gameMoveHistory.length - 1)
                          ? const Color(0xFF222ab7)
                          : const Color(0xFF626af7),
                      icon: const Icon(Icons.navigate_next),
                    ),
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
