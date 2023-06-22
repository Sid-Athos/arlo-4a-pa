
class MessageRequest {

  static String toJson(String message) {
    return " {\"Message\": {\"message\": \"$message\"}}";
  }
}