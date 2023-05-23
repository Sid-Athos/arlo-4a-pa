
class CreateLobbyRequest {

  static String toJson(int gameId, bool private) {
    return " {\"CreateLobby\": {\"game_id\": $gameId, \"private\": $private}}";
  }
}