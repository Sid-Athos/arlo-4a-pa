
class KickUserRequest {

  static String toJson(int userId) {
    return " {\"KickUser\": {\"user_id\": $userId}}";
  }
}