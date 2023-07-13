import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:miku/api/user/api_user.dart';
import 'package:miku/model/user_model.dart';
import 'package:miku/view/widget/ranking_card_widget.dart';
import 'package:miku/view/widget/ranking_game_card_widget.dart';

import '../model/game_model.dart';
import '../model/ranking_model.dart';

class RankingUserView extends StatefulWidget {
  RankingUserView({super.key, required this.user});

  User user;

  @override
  _RankingUserViewState createState() => _RankingUserViewState(user: user);
}

class _RankingUserViewState extends State<RankingUserView> {
  _RankingUserViewState({required this.user});

  late Future<List<Ranking>> rankings;
  User user;

  @override
  void initState() {
    super.initState();
    rankings = ApiUser.getRankingForUser(user.id);
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text("Ranking ${user.pseudo}"),
        backgroundColor: const Color(0xFF21262B),
      ),
      backgroundColor: const Color(0xFF21262B),
      body: Center(
        child: RefreshIndicator(
          onRefresh: () async {
            setState(() {
              rankings = ApiUser.getRankingForUser(user.id);
            });
          },
          child: FutureBuilder<List<Ranking>>(
            future: rankings,
            builder: (context, snapshot) {
              if (snapshot.hasData && snapshot.data!.length > 0) {
                return ListView.builder(
                  itemCount: snapshot.data?.length,
                  itemBuilder: (context, index) {
                    return RankingGameCardWidget(ranking: snapshot.data![index], top: index);
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
                      "Not Ranked",
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
