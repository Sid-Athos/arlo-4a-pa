import '../../model/history/game_move_history_model.dart';
import '../game/game_svg_info_response_mapper.dart';

class GameMoveHistoryResponseMapper {

  static GameMoveHistory fromJson(Map<String, dynamic> json) {
    return GameMoveHistory(
      id: json['id'],
      player: json['player'],
      game_state: GameSvgInfoResponseMapper.fromJson(json['game_state']),
      action: json['action'],
      action_number: json['action_number']
    );
  }
}