import 'game_action_zone_model.dart';

class GameAction {
  String type;
  List<GameActionZone> zones;

  GameAction({
    required this.type,
    required this.zones,
  });

  bool isInZones(int x, int y) {
    for (int i = 0; i < zones.length; i++) {
      if (zones[i].isInZone(x, y)) {
        return true;
      }
    }
    return false;
  }
}