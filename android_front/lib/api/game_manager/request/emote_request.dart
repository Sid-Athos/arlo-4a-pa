
class EmoteRequest {

  static String toJson(String emote) {
    return " {\"Emote\": {\"emote\": \"$emote\"}}";
  }
}