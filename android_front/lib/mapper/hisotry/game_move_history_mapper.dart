import 'dart:convert';

import '../../model/history/game_move_history_model.dart';
import '../game/full_game_state_mapper.dart';

class GameMoveHistoryResponseMapper {

  static GameMoveHistory fromJson(Map<String, dynamic> json) {
    print(json['game_state']);
    Map<String, dynamic> game_state = jsonDecode(json['game_state']);
    return GameMoveHistory(
      id: json['id'],
      player: json['player'],
      game_state: FullGameStateMapper.fromJson(game_state),
      action: json['action'],
      action_number: json['action_number']
    );
  }
}