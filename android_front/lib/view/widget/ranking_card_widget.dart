
import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:miku/model/ranking_model.dart';

class RankingCardWidget extends StatelessWidget {
  RankingCardWidget(
      {super.key,
        required this.ranking});

  Ranking ranking;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8.0, horizontal: 16.0),
      child: Card(
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(16),
        ),
        color: const Color(0xFF1A2025),
        child: Padding(
          padding: const EdgeInsets.only(bottom: 16.0, right: 16.0, left: 16.0),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: <Widget>[
              Row(
                children: [
                  Flexible(
                    child: ListTile(
                      title: Text(
                        ranking.user.pseudo,
                        style: const TextStyle(color: Colors.white),
                      ),
                      subtitle: Text(
                        "${ranking.rank} lp | ${ranking.nbGames} games",
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
    );
  }
}
