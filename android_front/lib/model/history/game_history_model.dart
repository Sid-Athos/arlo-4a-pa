import '../game/game_model.dart';

class GameHistory {

  int id;
  Game game;
  int nb_players;
  DateTime dateTime;

  GameHistory({
    required this.id,
    required this.game,
    required this.nb_players,
    required this.dateTime,
  });
}