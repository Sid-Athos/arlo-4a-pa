
class AcceptInviteLobbyRequest {

  static String toJson(int userId) {
    return " {\"AcceptInviteLobby\": {\"user_id\": $userId}}";
  }
}