class CreateLobbyFromHistoryRequest {

  static String toJson(int gameMoveHistoryId) {
    return " {\"CreateLobbyFromHistory\": {\"game_move_history_id\": $gameMoveHistoryId}}";
  }
}