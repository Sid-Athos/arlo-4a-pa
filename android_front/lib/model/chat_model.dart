
import 'package:flutter/cupertino.dart';

import '../api/game_manager/response/message_response_ws.dart';

class Chat extends ChangeNotifier {

  List<MessageResponseWS> messages;

  Chat({
    required this.messages,
  });

  void addMessage(MessageResponseWS message) {
    messages.add(message);
    notifyListeners();
  }
}