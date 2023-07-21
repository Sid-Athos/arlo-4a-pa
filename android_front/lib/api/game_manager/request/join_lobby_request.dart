
class JoinLobbyRequest {

  static String toJson(int lobbyId) {
    return " {\"JoinLobby\": {\"lobby_id\": $lobbyId}}";
    }
}