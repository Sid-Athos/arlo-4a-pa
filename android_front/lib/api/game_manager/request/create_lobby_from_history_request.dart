class CreateLobbyFromHistoryRequest {

  static String toJson(int gameMoveHistoryId) {
    print("{\"CreateLobbyByGameMoveHistoryIdRequest\": {\"game_move_id\": $gameMoveHistoryId}}");
    return "{\"CreateLobbyByGameMoveHistoryIdRequest\": {\"game_move_id\": $gameMoveHistoryId}}";
  }
}