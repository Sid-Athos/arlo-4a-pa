class CreateLobbyFromHistoryRequest {

  static String toJson(int gameMoveHistoryId) {
    return " {\"CreateLobbyByGameMoveHistoryIdRequest\": {\"game_move_id\": $gameMoveHistoryId}}";
  }
}