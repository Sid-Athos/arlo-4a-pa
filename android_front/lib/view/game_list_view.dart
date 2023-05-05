import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:miku/api/game_manager/api_game_manager.dart';
import 'package:miku/model/game_model.dart';

import 'package:miku/view/lobby_list_view.dart';
import 'package:web_socket_channel/web_socket_channel.dart';

class GameScreen extends StatefulWidget {
  GameScreen({super.key, required this.channel});
  WebSocketChannel channel;

  @override
  _GameScreenState createState() => _GameScreenState(channel: channel);
}

class _GameScreenState extends State<GameScreen> {
  _GameScreenState({required this.channel});

  late Future<List<Game>> games;
  WebSocketChannel channel;

  @override
  void initState() {
    super.initState();
    games = ApiGameManager.getAllGames();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF21262B),
      body: Center(
        child: RefreshIndicator(
          onRefresh: () async {
            setState(() {
              games = ApiGameManager.getAllGames();
            });
          },
          child: FutureBuilder<List<Game>>(
            future: games,
            builder: (context, snapshot) {
              if (snapshot.hasData) {
                return ListView.builder(
                  itemCount: snapshot.data?.length,
                  itemBuilder: (context, index) {
                    return Padding(
                      padding: const EdgeInsets.symmetric(vertical: 8.0, horizontal: 16.0),
                      child: ElevatedButton(
                        style: ElevatedButton.styleFrom(
                          primary: const Color(0xFF1A2025),
                          minimumSize: const Size(200, 75),
                          shape: RoundedRectangleBorder(
                            borderRadius: BorderRadius.circular(16),
                          ),
                        ),
                        onPressed: () {
                          Navigator.push(
                            context,
                            MaterialPageRoute(builder: (context) => LobbyListView(game: snapshot.data![index], channel: channel)),
                          );
                        },
                        child: Text(
                          snapshot.data?[index].name ?? "",
                          style: const TextStyle(fontSize: 20, color: Colors.white),
                        ),
                      ),
                    );
                  },
                );
              } else if (snapshot.hasError) {
                return Text("${snapshot.error}");
              }

              return CircularProgressIndicator();
            },
          ),
        ),
      ),
    );
  }
}
