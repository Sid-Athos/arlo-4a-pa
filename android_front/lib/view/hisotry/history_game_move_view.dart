import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:web_socket_channel/web_socket_channel.dart';

import '../../api/game_manager/api_game_manager.dart';
import '../../model/history/game_history_model.dart';
import '../../model/history/game_move_history_model.dart';
import '../review/replay_game_view.dart';
import 'game_move_history_card_widget.dart';

class HistoryGameMoveView extends StatefulWidget {
  HistoryGameMoveView({super.key,
      required this.game_history,
      required this.channel});

  GameHistory game_history;
  WebSocketChannel channel;

  @override
  _HistoryGameMoveViewState createState() =>
      _HistoryGameMoveViewState(game_history: game_history, channel: channel);
}

class _HistoryGameMoveViewState extends State<HistoryGameMoveView> {
  _HistoryGameMoveViewState({required this.game_history, required this.channel});

  late Future<List<GameMoveHistory>> game_move_history;
  GameHistory game_history;
  WebSocketChannel channel;

  @override
  void initState() {
    super.initState();
    game_move_history = ApiGameManager.getGameMoveHistory(game_history.id);
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("Moves history"),
        backgroundColor: const Color(0xFF21262B),
      ),
      backgroundColor: const Color(0xFF21262B),
      body: Column(
        children: [
          Expanded(
            child: RefreshIndicator(
               onRefresh: () async {
                 setState(() {
                   game_move_history = ApiGameManager.getGameMoveHistory(game_history.id);
                 });
               },
               child: FutureBuilder<List<GameMoveHistory>>(
                 future: game_move_history,
                 builder: (context, snapshot) {
                   if (snapshot.hasData && snapshot.data!.length > 0) {
                     return ListView.builder(
                       itemCount: snapshot.data?.length,
                       itemBuilder: (context, index) {
                         return GameMoveHistoryCardWidget(game_move_history: snapshot.data![index], channel: channel);
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
                             "No moves found.",
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
          Padding(
            padding: const EdgeInsets.only(left: 40, right: 40, top: 10),
            child: SizedBox(
              width: double.infinity,
              child: ElevatedButton(
                onPressed: () async {
                  List<GameMoveHistory> transferable = await game_move_history;
                  Navigator.push(
                    context,
                    MaterialPageRoute(builder: (context) => ReplayGameView(gameMoveHistory: transferable))
                  );
                },
                style: ElevatedButton.styleFrom(
                  backgroundColor: const Color(0xFF626af7),
                ),
                child: const Text("Review Match"),
              ),
            ),
          ),
        ],
      ),
    );
  }
}