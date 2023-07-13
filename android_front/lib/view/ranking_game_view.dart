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

  late Future<List<Ranking>> rankingAll;
  late Future<List<Ranking>> rankingFriends;
  Game game;
  bool isAll = true;

  @override
  void initState() {
    super.initState();
    rankingAll = ApiUser.getRankingForGame(game.id);
    rankingFriends = ApiUser.getRankingFriendsForGame(game.id);
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text("Ranking ${game.name}"),
        backgroundColor: const Color(0xFF21262B),
      ),
      backgroundColor: const Color(0xFF21262B),
      body: Column(
        children: [
          Padding(
            padding: const EdgeInsets.only(top: 10, left: 20, right: 20),
            child: Row(
              mainAxisSize: MainAxisSize.max,
              children: [
                Expanded(
                  child: ElevatedButton(
                    onPressed: () {
                      setState(() {
                        isAll = true;
                      });
                    },
                    style: isAll
                        ? ElevatedButton.styleFrom(
                            backgroundColor: const Color(0xFF626af7),
                            shape: RoundedRectangleBorder(
                                borderRadius: BorderRadius.circular(8)),
                          )
                        : ElevatedButton.styleFrom(
                            backgroundColor: const Color(0xFF21262B),
                            shape: RoundedRectangleBorder(
                                side: const BorderSide(
                                    width: 1, color: Colors.black54),
                                borderRadius: BorderRadius.circular(8)),
                          ),
                    child: const Text("All"),
                  ),
                ),
                Expanded(
                  child: ElevatedButton(
                    onPressed: () {
                      setState(() {
                        isAll = false;
                      });
                    },
                    style: !isAll
                        ? ElevatedButton.styleFrom(
                            backgroundColor: const Color(0xFF626af7),
                            shape: RoundedRectangleBorder(
                                borderRadius: BorderRadius.circular(8)),
                          )
                        : ElevatedButton.styleFrom(
                            backgroundColor: const Color(0xFF21262B),
                            shape: RoundedRectangleBorder(
                                side: const BorderSide(
                                    width: 1, color: Colors.black54),
                                borderRadius: BorderRadius.circular(8)),
                          ),
                    child: const Text("Friends"),
                  ),
                ),
              ],
            ),
          ),
          isAll
              ? Expanded(
                  child: Center(
                    child: RefreshIndicator(
                      onRefresh: () async {
                        setState(() {
                          rankingAll = ApiUser.getRankingForGame(game.id);
                        });
                      },
                      child: FutureBuilder<List<Ranking>>(
                        future: rankingAll,
                        builder: (context, snapshot) {
                          if (snapshot.hasData && snapshot.data!.length > 0) {
                            return ListView.builder(
                              itemCount: snapshot.data?.length,
                              itemBuilder: (context, index) {
                                return RankingCardWidget(
                                    ranking: snapshot.data![index], top: index);
                              },
                            );
                          } else if (snapshot.hasError) {
                            return Text("${snapshot.error}");
                          } else if (snapshot.hasData &&
                              snapshot.data!.length == 0) {
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
                )
              : Expanded(
                  child: Center(
                    child: RefreshIndicator(
                      onRefresh: () async {
                        setState(() {
                          rankingFriends =
                              ApiUser.getRankingFriendsForGame(game.id);
                        });
                      },
                      child: FutureBuilder<List<Ranking>>(
                        future: rankingFriends,
                        builder: (context, snapshot) {
                          if (snapshot.hasData && snapshot.data!.length > 0) {
                            return ListView.builder(
                              itemCount: snapshot.data?.length,
                              itemBuilder: (context, index) {
                                return RankingCardWidget(
                                    ranking: snapshot.data![index], top: index);
                              },
                            );
                          } else if (snapshot.hasError) {
                            return Text("${snapshot.error}");
                          } else if (snapshot.hasData &&
                              snapshot.data!.length == 0) {
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
                                  "No friends is ranked",
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
                ),
        ],
      ),
    );
  }
}
