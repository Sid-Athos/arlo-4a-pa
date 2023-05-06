import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:miku/api/game_manager/request/join_lobby_request.dart';
import 'package:miku/model/game_model.dart';
import 'package:miku/model/lobby_model.dart';
import 'package:web_socket_channel/web_socket_channel.dart';

import '../api/game_manager/api_game_manager.dart';

class LobbyListView extends StatefulWidget {
  LobbyListView({super.key, required this.game, required this.channel});

  Game game;
  WebSocketChannel channel;

  @override
  _LobbyListViewState createState() =>
      _LobbyListViewState(game: game, channel: channel);
}

class _LobbyListViewState extends State<LobbyListView> {
  _LobbyListViewState({required this.game, required this.channel});

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
        actions: <Widget>[
          Padding(
              padding: const EdgeInsets.only(right: 20.0),
              child: GestureDetector(
                onTap: _showMyDialog,
                child: const Icon(
                  Icons.add,
                  size: 32.0,
                ),
              )
          ),
        ],
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
                    return LobbyCardWidget(
                        lobby: snapshot.data![index],
                        game: game,
                        channel: channel);
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

  Future<void> _showMyDialog() async {
    bool isPrivate = false;
    return showDialog<void>(
      context: context,
      barrierDismissible: false, // user must tap button!
      builder: (BuildContext context) {
        return AlertDialog(
          backgroundColor: const Color(0xFF21262B),
          title: const Text(
            "Create Lobby",
            style: TextStyle(
              color: Colors.white,
            )
          ),
          content: SingleChildScrollView(
            child: ListBody(
              children: <Widget>[
                Text(
                  "Max Players : ${game.nbPlayer}",
                  style: const TextStyle(
                    color: Colors.white,
                  ),
                ),
                CheckboxListTile(
                  title:
                  const Text(
                    "Private : ",
                    style: TextStyle(
                      color: Colors.white,
                    ),
                  ),
                  value: isPrivate,
                  onChanged: (bool? value) {
                    setState(() {
                      isPrivate = value!;
                    });
                  },
                ),
              ],
            ),
          ),
          actions: <Widget>[
            TextButton(
              style: TextButton.styleFrom(
                primary: const Color(0xFF626af7),
              ),
              child: const Text('Create'),
              onPressed: () {
                Navigator.of(context).pop();
              },
            ),
          ],
        );
      },
    );
  }
}

class LobbyCardWidget extends StatelessWidget {
  LobbyCardWidget(
      {super.key,
      required this.lobby,
      required this.game,
      required this.channel});

  Lobby lobby;
  Game game;
  WebSocketChannel channel;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8.0, horizontal: 16.0),
      child: InkWell(
        onTap: () {
          channel.sink.add(JoinLobbyRequest.toJson(lobby.id));
        },
        child: Card(
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(16),
          ),
          color: const Color(0xFF1A2025),
          child: Padding(
            padding: const EdgeInsets.only(bottom: 16.0, right: 32.0, left: 16.0),
            child: Column(
              mainAxisSize: MainAxisSize.min,
              children: <Widget>[
                ListTile(
                  title: Text(
                    "Host : ${lobby.getHost().pseudo}",
                    style: TextStyle(color: Colors.white),
                  ),
                  subtitle: Text(
                    lobby.code,
                    style: TextStyle(color: Colors.white38),
                  ),
                ),
                Row(
                  mainAxisAlignment: MainAxisAlignment.end,
                  children: <Widget>[
                    Text(
                      "Players : ${lobby.members.length} / ${game.nbPlayer}",
                      style: TextStyle(color: Colors.white),
                    ),
                  ],
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
