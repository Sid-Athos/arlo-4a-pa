
class GameActionRequest {

  static String toJson(int x, int y) {
    return "{\"GameAction\": {\"x\": $x, \"y\": $y}}";
  }
}