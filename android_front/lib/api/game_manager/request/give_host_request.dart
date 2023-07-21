
class GiveHostRequest {

  static String toJson(int userId) {
    return " {\"GiveHost\": {\"user_id\": $userId}}";
  }
}