import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:miku/api/game_manager/request/join_lobby_request.dart';
import 'package:miku/api/game_manager/request/ping_request.dart';
import 'package:miku/model/game_model.dart';
import 'package:miku/model/lobby_model.dart';
import 'package:web_socket_channel/web_socket_channel.dart';

import '../api/game_manager/api_game_manager.dart';

class LobbyScreen extends StatefulWidget {
  LobbyScreen({super.key, required this.game, required this.channel});
  Game game;
  WebSocketChannel channel;

  @override
  _LobbyScreenState createState() => _LobbyScreenState(game: game, channel: channel);
}

class _LobbyScreenState extends State<LobbyScreen> {
  _LobbyScreenState({required this.game, required this.channel});

  late Future<List<Lobby>> lobbies;
  Game game;
  WebSocketChannel channel;

  @override
  void initState() {
    super.initState();
    lobbies = ApiGameManager.getPublicLobbyForGame(game.id);
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text(game.name),
        backgroundColor: const Color(0xFF21262B),
      ),
      backgroundColor: const Color(0xFF21262B),
      body: Center(
        child: RefreshIndicator(
          onRefresh: () async {
            setState(() {
              lobbies = ApiGameManager.getPublicLobbyForGame(game.id);
            });
          },
          child: FutureBuilder<List<Lobby>>(
            future: lobbies,
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
                          channel.sink.add(JoinLobbyRequest.toJson(snapshot.data![index].id));
                        },
                        child: Text(
                          snapshot.data?[index].code ?? "",
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