import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:miku/api/user/api_user.dart';
import 'package:miku/view/widget/ranking_card_widget.dart';

import '../model/game_model.dart';
import '../model/ranking_model.dart';

class RankingGameView extends StatefulWidget {
  RankingGameView({super.key, required this.game});

  Game game;

  @override
  _RankingGameViewState createState() => _RankingGameViewState(game: game);
}

class _RankingGameViewState extends State<RankingGameView> {
  _RankingGameViewState({required this.game});

  late Future<List<Ranking>> rankings;
  Game game;

  @override
  void initState() {
    super.initState();
    rankings = ApiUser.getRankingForGame(game.id);
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text("Ranking ${game.name}"),
        backgroundColor: const Color(0xFF21262B),
      ),
      backgroundColor: const Color(0xFF21262B),
      body: Center(
        child: RefreshIndicator(
          onRefresh: () async {
            setState(() {
              rankings = ApiUser.getRankingForGame(game.id);
            });
          },
          child: FutureBuilder<List<Ranking>>(
            future: rankings,
            builder: (context, snapshot) {
              if (snapshot.hasData && snapshot.data!.length > 0) {
                return ListView.builder(
                  itemCount: snapshot.data?.length,
                  itemBuilder: (context, index) {
                    return RankingCardWidget(ranking: snapshot.data![index], top: index);
                  },
                );
              } else if (snapshot.hasError) {
                return Text("${snapshot.error}");
              } else if (snapshot.hasData && snapshot.data!.length == 0) {
                return Column(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: const <Widget>[
                    Padding(
                      padding: EdgeInsets.all(16.0),
                      child: Icon(
                        Icons.emoji_events_outlined,
                        color: Colors.white,
                        size: 48,
                      ),
                    ),
                    Text(
                      "Nobody is ranked",
                      style: TextStyle(
                        color: Colors.white,
                        fontSize: 16,
                      ),
                    ),
                  ],
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
