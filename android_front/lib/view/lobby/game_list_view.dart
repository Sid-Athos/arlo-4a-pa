import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:miku/api/game_manager/api_game_manager.dart';
import 'package:miku/model/game/game_model.dart';
import 'package:miku/model/lobby/lobby_model.dart';

import 'package:miku/view/lobby/lobby_list_view.dart';
import 'package:web_socket_channel/web_socket_channel.dart';

class GameScreen extends StatefulWidget {
  GameScreen({super.key, required this.channel, required this.lobby});
  WebSocketChannel channel;
  Lobby lobby;

  @override
  _GameScreenState createState() => _GameScreenState(channel: channel, lobby: lobby);
}

class _GameScreenState extends State<GameScreen> {
  _GameScreenState({required this.channel, required this.lobby});

  late Future<List<Game>> games;
  Lobby lobby;
  WebSocketChannel channel;

  @override
  void initState() {
    super.initState();
    games = ApiGameManager.getAllGames();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        automaticallyImplyLeading: false,
        title: const Text("Games"),
        backgroundColor: const Color(0xFF21262B),
      ),
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
                    return GameCardWidget(game: snapshot.data![index], channel: channel, lobby: lobby);
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

class GameCardWidget extends StatelessWidget {
  GameCardWidget({super.key, required this.game, required this.channel, required this.lobby});

  Game game;
  Lobby lobby;
  WebSocketChannel channel;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8.0, horizontal: 16.0),
      child: InkWell(
        onTap: () {
          Navigator.push(
            context,
            MaterialPageRoute(builder: (context) => LobbyListView(game: game, channel: channel, lobby: lobby)),
          );
        },
        child: Card(
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(16),
          ),
          color: const Color(0xFF1A2025),
          child: Padding(
            padding:
            const EdgeInsets.only(bottom: 16.0, right: 32.0, left: 16.0),
            child: Column(
              mainAxisSize: MainAxisSize.min,
              children: <Widget>[
                ListTile(
                  title: Text(
                    game.name,
                    style: const TextStyle(color: Colors.white),
                  ),
                  subtitle: Text(
                    "Number Players: ${game.minPlayers} - ${game.maxPlayers}",
                    style: const TextStyle(color: Colors.white38),
                  ),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}