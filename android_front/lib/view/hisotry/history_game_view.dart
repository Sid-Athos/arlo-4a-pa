import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:miku/view/hisotry/game_history_card_widget.dart';
import 'package:web_socket_channel/web_socket_channel.dart';

import '../../api/game_manager/api_game_manager.dart';
import '../../model/game/game_model.dart';
import '../../model/history/game_history_model.dart';
import 'game_move_history_card_widget.dart';

class HistoryGameView extends StatefulWidget {
  HistoryGameView({super.key,
      required this.game,
      required this.channel});

  Game game;
  WebSocketChannel channel;

  @override
  _HistoryGameViewState createState() =>
      _HistoryGameViewState(game: game, channel: channel);
}

class _HistoryGameViewState extends State<HistoryGameView> {
  _HistoryGameViewState({required this.game, required this.channel});

  late Future<List<GameHistory>> gameHistory;
  Game game;
  WebSocketChannel channel;

  @override
  void initState() {
    super.initState();
    gameHistory = ApiGameManager.getGameHistory(game.id);
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("History"),
        backgroundColor: const Color(0xFF21262B),
      ),
      backgroundColor: const Color(0xFF21262B),
      body: Center(
        child: RefreshIndicator(
          onRefresh: () async {
            setState(() {
              gameHistory = ApiGameManager.getGameHistory(game.id);
            });
          },
          child: FutureBuilder<List<GameHistory>>(
            future: gameHistory,
            builder: (context, snapshot) {
              if (snapshot.hasData && snapshot.data!.length > 0) {
                return ListView.builder(
                  itemCount: snapshot.data?.length,
                  itemBuilder: (context, index) {
                    return GameHistoryCardWidget(game_history: snapshot.data![index], channel: channel,);
                  },
                );
              } else if (snapshot.hasError) {
                return Text("${snapshot.error}");
              } else if (snapshot.hasData && snapshot.data!.length == 0) {
                return SingleChildScrollView(
                  physics: const AlwaysScrollableScrollPhysics(),
                  child: Column(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: const <Widget>[
                      Padding(
                        padding: EdgeInsets.all(16.0),
                        child: Icon(
                          Icons.search_off,
                          color: Colors.white,
                          size: 48,
                        ),
                      ),
                      Text(
                        "You never played this game",
                        style: TextStyle(
                          color: Colors.white,
                          fontSize: 16,
                        ),
                      ),
                    ],
                  ),
                );
              }
              return CircularProgressIndicator();
            },
          ),
        ),
      ),
    );
  }
}