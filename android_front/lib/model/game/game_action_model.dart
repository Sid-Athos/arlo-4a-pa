import 'game_action_zone_model.dart';

class GameAction {
  String type;
  List<GameActionZone> zones;
  int player;

  GameAction({
    required this.type,
    required this.zones,
    required this.player
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