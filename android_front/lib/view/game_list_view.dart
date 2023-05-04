import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:miku/api/game_manager/api_game_manager.dart';
import 'package:miku/model/game_model.dart';
import 'dart:developer' as developer;

import 'package:miku/view/lobby_list_view.dart';

class GameScreen extends StatefulWidget {
  const GameScreen({super.key});

  @override
  _GameScreenState createState() => _GameScreenState();
}

class _GameScreenState extends State<GameScreen> {

  late Future<List<Game>> games;

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
                          developer.log("Pressed ${snapshot.data?[index].name}");
                          Navigator.push(
                            context,
                            MaterialPageRoute(builder: (context) => const LobbyScreen()),
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
