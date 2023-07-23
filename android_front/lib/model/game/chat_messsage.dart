import 'package:miku/model/user/user_model.dart';

class ChatMessage {

  String message;
  User fromUser;
  bool isEmote;

  ChatMessage({
    required this.message,
    required this.fromUser,
    required this.isEmote,
  });
}