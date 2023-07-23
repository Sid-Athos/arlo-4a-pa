import '../../model/history/game_history_model.dart';
import '../game/game_response_mapper.dart';

class GameHistoryResponseMapper {

  static GameHistory fromJson(Map<String, dynamic> json) {
    return GameHistory(
      id: json['id'],
      game: GameResponseMapper.fromJson(json['game']),
      nb_players: json['nb_players'],
      dateTime: DateTime.parse(json['date_time'])
    );
  }
}