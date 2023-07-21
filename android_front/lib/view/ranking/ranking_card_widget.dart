import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:miku/model/ranking/ranking_model.dart';

import '../profile/profile_other_view.dart';

class RankingCardWidget extends StatelessWidget {
  RankingCardWidget({super.key, required this.ranking, required this.top});

  Ranking ranking;
  int top;

  @override
  Widget build(BuildContext context) {
    return Padding(
        padding: const EdgeInsets.symmetric(vertical: 8.0, horizontal: 16.0),
        child: InkWell(
          onTap: () {
            Navigator.push(
              context,
              MaterialPageRoute(
                  builder: (context) => ProfileOtherView(user: ranking.user)),
            );
          },
          child: Card(
            shape: RoundedRectangleBorder(
              borderRadius: BorderRadius.circular(16),
            ),
            color: const Color(0xFF1A2025),
            child: Padding(
              padding: const EdgeInsets.only(
                  bottom: 8.0, right: 16.0, left: 16.0, top: 8.0),
              child: Column(
                mainAxisSize: MainAxisSize.min,
                crossAxisAlignment: CrossAxisAlignment.stretch,
                children: <Widget>[
                  Row(
                    children: [
                      Text(
                        "${top + 1}",
                        style:
                            const TextStyle(color: Colors.white, fontSize: 20),
                      ),
                      Flexible(
                        child: ListTile(
                          title: Text(
                            ranking.user.pseudo,
                            style: const TextStyle(color: Colors.white),
                          ),
                          subtitle: Text(
                            "${ranking.rank} LP | ${ranking.nbGames} Games",
                            style: const TextStyle(color: Colors.white38),
                          ),
                        ),
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
