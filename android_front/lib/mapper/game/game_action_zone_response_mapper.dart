import 'package:miku/model/game/game_action_zone_model.dart';

class GameActionZoneResponseMapper {

  static GameActionZone fromJson(Map<String, dynamic> json) {
    return GameActionZone(
        x: json["x"],
        y: json["y"],
        width: json["width"],
        height: json["height"]);
  }
}
