import 'dart:convert';
import 'dart:developer' as developer;

import 'message_response.dart';

class ResponseWS {

  static void computeResponse(String message) {

    switch (message) {
      case "\"BadMessage\"": developer.log("BadMessage"); return;
      case "\"Pong\"": developer.log("Pong"); return;
    }

    Map<String, dynamic> json = jsonDecode(message);
    for (var key in json.keys) {
      switch (key) {
        case "Message": MessageResponse.compute(json); break;
      }
    }
  }
}