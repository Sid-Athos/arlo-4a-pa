import 'package:miku/model/game/game_state_model.dart';

class GameStateMapper {

  static GameState fromJson(Map<String, dynamic> json) {
    return GameState(
      scores: (json['scores'] as List<dynamic>).map((e) => e as int).toList(),
      game_over: json["game_over"],
    );
  }
}