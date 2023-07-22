import 'package:miku/model/game/game_model.dart';
import 'package:miku/model/user/user_model.dart';

class Ranking {
  int id;
  int rank;
  User user;
  Game game;
  int nbGames;

  Ranking({
    required this.id,
    required this.rank,
    required this.user,
    required this.game,
    required this.nbGames
  });
}
