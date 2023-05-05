import 'dart:convert';
import 'dart:developer' as developer;

import 'lobby_response_ws.dart';
import 'message_response_ws.dart';

class ResponseWS {

  static void computeResponse(String message) {

    switch (message) {
      case "\"BadMessage\"": developer.log("BadMessage"); return;
      case "\"Pong\"": developer.log("Pong"); return;
      case "\"LobbyJoined\"": developer.log("LobbyJoined"); return;
    }

    Map<String, dynamic> json = jsonDecode(message);
    for (var key in json.keys) {
      switch (key) {
        case "Message": MessageResponseWS.compute(json); break;
        case "Lobby": LobbyResponseWS.compute(json); break;
      }
    }
  }
}