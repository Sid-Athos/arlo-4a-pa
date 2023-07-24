import 'package:miku/mapper/game/game_action_zone_response_mapper.dart';
import 'package:miku/model/game/game_action_model.dart';

class GameActionResponseMapper {

  static GameAction fromJson(Map<String, dynamic> json) {
    return GameAction(
      type: json["type"],
      zones: (json['zones'] as List<dynamic>).map((e) => GameActionZoneResponseMapper.fromJson(e)).toList(),
      player: json["player"]
    );
  }
}