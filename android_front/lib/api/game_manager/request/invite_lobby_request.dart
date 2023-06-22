
class InviteLobbyRequest {

  static String toJson(int userId) {
    return " {\"InviteUserLobby\": {\"user_id\": $userId}}";
  }
}