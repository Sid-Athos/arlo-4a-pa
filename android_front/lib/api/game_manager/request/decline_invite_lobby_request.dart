
class DeclineInviteLobbyRequest {

  static String toJson(int userId) {
    return " {\"DeclineInviteLobby\": {\"user_id\": $userId}}";
  }
}