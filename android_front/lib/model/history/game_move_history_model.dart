import '../game/full_game_state_model.dart';
import '../game/game_svg_info_model.dart';

class GameMoveHistory {

  int id;
  int player;
  FullGameState game_state;
  String action;
  int action_number;

  GameMoveHistory({
    required this.id,
    required this.player,
    required this.game_state,
    required this.action,
    required this.action_number,
  });
}